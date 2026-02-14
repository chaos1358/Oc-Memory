"""
Dropbox Sync Client for OC-Memory
Cloud backup and reverse lookup for Cold memory

Provides upload/download/search operations for syncing
Obsidian vault or memory archives to Dropbox.
"""

import logging
import os
from datetime import datetime
from pathlib import Path
from typing import Dict, Any, List, Optional

logger = logging.getLogger(__name__)


# =============================================================================
# Sync Result
# =============================================================================

class SyncResult:
    """Result of a sync operation"""

    def __init__(self):
        self.uploaded: int = 0
        self.downloaded: int = 0
        self.skipped: int = 0
        self.errors: int = 0
        self.conflicts: int = 0

    def __repr__(self):
        return (
            f"SyncResult(uploaded={self.uploaded}, downloaded={self.downloaded}, "
            f"skipped={self.skipped}, errors={self.errors})"
        )

    @property
    def total_synced(self) -> int:
        return self.uploaded + self.downloaded


# =============================================================================
# Dropbox Sync Client
# =============================================================================

class DropboxSync:
    """
    Dropbox sync client for OC-Memory cold storage.
    Handles upload, download, and bidirectional sync.
    """

    def __init__(
        self,
        app_key: Optional[str] = None,
        app_secret: Optional[str] = None,
        refresh_token: Optional[str] = None,
        remote_folder: str = "/OC-Memory",
        local_dir: Optional[str] = None,
    ):
        """
        Args:
            app_key: Dropbox app key
            app_secret: Dropbox app secret
            refresh_token: OAuth2 refresh token
            remote_folder: Remote folder path in Dropbox
            local_dir: Local directory to sync
        """
        self.app_key = app_key or os.environ.get("DROPBOX_APP_KEY", "")
        self.app_secret = app_secret or os.environ.get("DROPBOX_APP_SECRET", "")
        self.refresh_token = refresh_token or os.environ.get("DROPBOX_REFRESH_TOKEN", "")
        self.remote_folder = remote_folder
        self.local_dir = Path(local_dir).expanduser().resolve() if local_dir else None
        self._client = None

    @property
    def is_configured(self) -> bool:
        """Check if Dropbox credentials are configured"""
        return bool(self.app_key and self.refresh_token)

    def _ensure_client(self):
        """Lazy-initialize Dropbox client"""
        if self._client is not None:
            return

        if not self.is_configured:
            raise RuntimeError(
                "Dropbox not configured. Set DROPBOX_APP_KEY and "
                "DROPBOX_REFRESH_TOKEN environment variables."
            )

        try:
            import dropbox
            self._client = dropbox.Dropbox(
                app_key=self.app_key,
                app_secret=self.app_secret,
                oauth2_refresh_token=self.refresh_token,
            )
            # Verify connection
            self._client.users_get_current_account()
            logger.info("Dropbox client initialized successfully")
        except ImportError:
            logger.error("dropbox package not installed. Run: pip install dropbox")
            raise
        except Exception as e:
            logger.error(f"Failed to initialize Dropbox client: {e}")
            self._client = None
            raise

    def upload_file(
        self,
        local_path: Path,
        remote_path: Optional[str] = None,
    ) -> Optional[str]:
        """
        Upload a file to Dropbox.

        Args:
            local_path: Local file path
            remote_path: Remote path (auto-generated if None)

        Returns:
            Remote path string, or None on failure
        """
        local_path = Path(local_path)
        if not local_path.exists():
            logger.error(f"Local file not found: {local_path}")
            return None

        self._ensure_client()
        import dropbox

        if remote_path is None:
            remote_path = f"{self.remote_folder}/{local_path.name}"

        try:
            with open(local_path, 'rb') as f:
                self._client.files_upload(
                    f.read(),
                    remote_path,
                    mode=dropbox.files.WriteMode.overwrite,
                )
            logger.info(f"Uploaded: {local_path.name} -> {remote_path}")
            return remote_path
        except Exception as e:
            logger.error(f"Upload failed for {local_path.name}: {e}")
            return None

    def download_file(
        self,
        remote_path: str,
        local_path: Optional[Path] = None,
    ) -> Optional[Path]:
        """
        Download a file from Dropbox.

        Args:
            remote_path: Remote file path
            local_path: Local save path (auto-generated if None)

        Returns:
            Local Path, or None on failure
        """
        self._ensure_client()

        if local_path is None:
            if self.local_dir is None:
                raise ValueError("local_dir must be set to auto-generate download paths")
            filename = remote_path.rsplit("/", 1)[-1]
            local_path = self.local_dir / filename

        local_path = Path(local_path)
        local_path.parent.mkdir(parents=True, exist_ok=True)

        try:
            self._client.files_download_to_file(str(local_path), remote_path)
            logger.info(f"Downloaded: {remote_path} -> {local_path}")
            return local_path
        except Exception as e:
            logger.error(f"Download failed for {remote_path}: {e}")
            return None

    def sync_folder(
        self,
        local_dir: Optional[Path] = None,
        remote_folder: Optional[str] = None,
    ) -> SyncResult:
        """
        Sync local directory with Dropbox folder.
        Uploads new/modified local files, downloads new remote files.

        Args:
            local_dir: Local directory to sync
            remote_folder: Remote folder path

        Returns:
            SyncResult with operation counts
        """
        self._ensure_client()

        local_dir = Path(local_dir) if local_dir else self.local_dir
        remote_folder = remote_folder or self.remote_folder
        result = SyncResult()

        if local_dir is None:
            raise ValueError("local_dir must be specified")

        local_dir = Path(local_dir).expanduser().resolve()
        local_dir.mkdir(parents=True, exist_ok=True)

        # Get remote file listing
        remote_files = self._list_remote_files(remote_folder)

        # Get local files
        local_files = {}
        for md_file in local_dir.rglob("*.md"):
            rel_path = md_file.relative_to(local_dir)
            remote_path = f"{remote_folder}/{rel_path.as_posix()}"
            local_files[remote_path] = md_file

        # Upload new/modified local files
        for remote_path, local_path in local_files.items():
            if remote_path in remote_files:
                local_mtime = local_path.stat().st_mtime
                remote_mtime = remote_files[remote_path].get('modified', 0)
                if local_mtime <= remote_mtime:
                    result.skipped += 1
                    continue

            if self.upload_file(local_path, remote_path):
                result.uploaded += 1
            else:
                result.errors += 1

        # Download new remote files
        for remote_path, info in remote_files.items():
            if remote_path not in local_files:
                rel_path = remote_path[len(remote_folder):].lstrip("/")
                local_path = local_dir / rel_path
                if self.download_file(remote_path, local_path):
                    result.downloaded += 1
                else:
                    result.errors += 1

        logger.info(f"Sync complete: {result}")
        return result

    def search(
        self,
        query: str,
        max_results: int = 10,
    ) -> List[Dict[str, Any]]:
        """
        Search for files in Dropbox.

        Args:
            query: Search query
            max_results: Maximum results

        Returns:
            List of result dicts with 'path', 'name', 'modified'
        """
        self._ensure_client()

        try:
            result = self._client.files_search_v2(query)
            results = []

            for match in result.matches[:max_results]:
                metadata = match.metadata.get_metadata()
                results.append({
                    'path': metadata.path_display,
                    'name': metadata.name,
                    'modified': getattr(metadata, 'client_modified', ''),
                })

            return results
        except Exception as e:
            logger.error(f"Search failed: {e}")
            return []

    def reverse_lookup(
        self,
        query: str,
        download_dir: Optional[Path] = None,
        max_results: int = 3,
    ) -> List[Path]:
        """
        Search Dropbox and download matching files (Cold -> Hot reverse lookup).

        Args:
            query: Search query
            download_dir: Directory to download to
            max_results: Maximum number of files to download

        Returns:
            List of downloaded file paths
        """
        search_results = self.search(query, max_results=max_results)
        download_dir = Path(download_dir) if download_dir else self.local_dir

        if download_dir is None:
            raise ValueError("download_dir must be specified")

        downloaded = []
        for result in search_results:
            local_path = download_dir / result['name']
            # Skip if file already exists locally
            if local_path.exists():
                logger.debug(f"Reverse lookup: skipping {result['name']} (already exists)")
                continue
            path = self.download_file(result['path'], local_path)
            if path:
                downloaded.append(path)

        logger.info(f"Reverse lookup: downloaded {len(downloaded)} files for '{query}'")
        return downloaded

    def get_stats(self) -> Dict[str, Any]:
        """Get sync statistics"""
        return {
            'configured': self.is_configured,
            'remote_folder': self.remote_folder,
            'local_dir': str(self.local_dir) if self.local_dir else None,
        }

    # =========================================================================
    # Private helpers
    # =========================================================================

    def _list_remote_files(self, folder: str) -> Dict[str, Dict[str, Any]]:
        """List files in a remote folder"""
        import dropbox

        files = {}
        try:
            result = self._client.files_list_folder(folder, recursive=True)

            while True:
                for entry in result.entries:
                    if isinstance(entry, dropbox.files.FileMetadata):
                        modified_ts = entry.client_modified.timestamp() if entry.client_modified else 0
                        files[entry.path_display] = {
                            'name': entry.name,
                            'modified': modified_ts,
                            'size': entry.size,
                        }

                if not result.has_more:
                    break
                result = self._client.files_list_folder_continue(result.cursor)

        except Exception as e:
            if "not_found" not in str(e).lower():
                logger.error(f"Error listing remote folder {folder}: {e}")

        return files


# =============================================================================
# Factory
# =============================================================================

def create_dropbox_sync(config: Dict[str, Any]) -> Optional[DropboxSync]:
    """Create a DropboxSync from config dictionary"""
    dropbox_config = config.get('dropbox', {})

    if not dropbox_config.get('enabled', False):
        return None

    # Resolve credentials from env var names or direct values
    app_key_env = dropbox_config.get('app_key_env', 'DROPBOX_APP_KEY')
    app_secret_env = dropbox_config.get('app_secret_env', 'DROPBOX_APP_SECRET')
    refresh_token_env = dropbox_config.get('refresh_token_env', 'DROPBOX_REFRESH_TOKEN')

    app_key = dropbox_config.get('app_key') or os.environ.get(app_key_env, '')
    app_secret = dropbox_config.get('app_secret') or os.environ.get(app_secret_env, '')
    refresh_token = dropbox_config.get('refresh_token') or os.environ.get(refresh_token_env, '')

    # Determine local_dir from obsidian vault if available
    obsidian_config = config.get('obsidian', {})
    local_dir = None
    if obsidian_config.get('enabled', False):
        vault_path = obsidian_config.get('vault_path', '~/Documents/ObsidianVault')
        local_dir = str(Path(vault_path).expanduser() / 'OC-Memory')

    return DropboxSync(
        app_key=app_key,
        app_secret=app_secret,
        refresh_token=refresh_token,
        remote_folder=dropbox_config.get('remote_folder', '/OC-Memory'),
        local_dir=local_dir,
    )
