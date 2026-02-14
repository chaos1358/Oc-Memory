"""Tests for lib/dropbox_sync.py"""

import pytest
from pathlib import Path

from lib.dropbox_sync import DropboxSync, SyncResult, create_dropbox_sync


class TestSyncResult:
    def test_init_defaults(self):
        r = SyncResult()
        assert r.uploaded == 0
        assert r.downloaded == 0
        assert r.skipped == 0
        assert r.errors == 0
        assert r.conflicts == 0

    def test_total_synced(self):
        r = SyncResult()
        r.uploaded = 3
        r.downloaded = 2
        assert r.total_synced == 5

    def test_repr(self):
        r = SyncResult()
        r.uploaded = 1
        assert "uploaded=1" in repr(r)


class TestDropboxSync:
    def test_init_defaults(self):
        client = DropboxSync()
        assert client.remote_folder == "/OC-Memory"
        assert client.local_dir is None

    def test_is_configured_false(self):
        client = DropboxSync(app_key="", refresh_token="")
        assert client.is_configured is False

    def test_is_configured_true(self):
        client = DropboxSync(app_key="key123", refresh_token="token456")
        assert client.is_configured is True

    def test_ensure_client_not_configured(self):
        client = DropboxSync(app_key="", refresh_token="")
        with pytest.raises(RuntimeError, match="not configured"):
            client._ensure_client()

    def test_upload_missing_file(self, temp_dir):
        client = DropboxSync(app_key="key", refresh_token="token")
        # Mock client so _ensure_client doesn't fail with real Dropbox
        client._client = True  # sentinel
        result = client.upload_file(Path("/nonexistent/file.md"))
        assert result is None

    def test_get_stats_not_configured(self):
        client = DropboxSync()
        stats = client.get_stats()
        assert stats['configured'] is False
        assert stats['remote_folder'] == "/OC-Memory"

    def test_get_stats_configured(self, temp_dir):
        client = DropboxSync(
            app_key="key",
            refresh_token="token",
            local_dir=str(temp_dir),
        )
        stats = client.get_stats()
        assert stats['configured'] is True
        assert stats['local_dir'] == str(temp_dir.resolve())

    def test_download_no_local_dir(self):
        client = DropboxSync(app_key="key", refresh_token="token")
        client._client = True
        with pytest.raises(ValueError, match="local_dir"):
            client.download_file("/remote/file.md")

    def test_sync_folder_no_local_dir(self):
        client = DropboxSync(app_key="key", refresh_token="token")
        client._client = True
        with pytest.raises(ValueError, match="local_dir"):
            client.sync_folder()

    def test_reverse_lookup_no_dir(self):
        client = DropboxSync(app_key="key", refresh_token="token")
        client._client = True
        with pytest.raises(ValueError, match="download_dir"):
            client.reverse_lookup("test query")


class TestCreateDropboxSync:
    def test_disabled(self):
        client = create_dropbox_sync({'dropbox': {'enabled': False}})
        assert client is None

    def test_not_configured(self):
        assert create_dropbox_sync({}) is None

    def test_enabled(self):
        config = {
            'dropbox': {
                'enabled': True,
                'app_key': 'test_key',
                'remote_folder': '/MyMemory',
            }
        }
        client = create_dropbox_sync(config)
        assert client is not None
        assert client.remote_folder == '/MyMemory'
