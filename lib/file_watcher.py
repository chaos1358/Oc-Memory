"""
File Watcher for OC-Memory
Monitors user directories for markdown file changes
"""

import logging
from pathlib import Path
from typing import Callable, List, Optional
from watchdog.observers import Observer
from watchdog.events import FileSystemEventHandler, FileSystemEvent


class MarkdownFileHandler(FileSystemEventHandler):
    """
    Event handler for markdown file changes
    Filters for .md files and triggers callback
    """

    def __init__(self, callback: Optional[Callable] = None):
        """
        Args:
            callback: Function to call when markdown file changes
                      Signature: callback(file_path: Path, event_type: str)
        """
        super().__init__()
        self.callback = callback
        self.logger = logging.getLogger(__name__)

    def _is_markdown_file(self, path: str) -> bool:
        """Check if file is a markdown file"""
        return Path(path).suffix.lower() in ['.md', '.markdown']

    def on_created(self, event: FileSystemEvent) -> None:
        """Handle file creation events"""
        if event.is_directory:
            return

        if self._is_markdown_file(event.src_path):
            file_path = Path(event.src_path)
            self.logger.info(f"New markdown file detected: {file_path}")

            if self.callback:
                try:
                    self.callback(file_path, event_type='created')
                except Exception as e:
                    self.logger.error(f"Error in callback for {file_path}: {e}")

    def on_modified(self, event: FileSystemEvent) -> None:
        """Handle file modification events"""
        if event.is_directory:
            return

        if self._is_markdown_file(event.src_path):
            file_path = Path(event.src_path)
            self.logger.debug(f"Markdown file modified: {file_path}")

            if self.callback:
                try:
                    self.callback(file_path, event_type='modified')
                except Exception as e:
                    self.logger.error(f"Error in callback for {file_path}: {e}")


class FileWatcher:
    """
    Watches user directories for markdown file changes
    Syncs changes to OpenClaw Memory directory
    """

    def __init__(
        self,
        watch_dirs: List[str],
        callback: Optional[Callable] = None,
        recursive: bool = True
    ):
        """
        Args:
            watch_dirs: List of directory paths to watch
            callback: Function to call when files change
            recursive: Watch subdirectories recursively
        """
        self.watch_dirs = [Path(d).expanduser().resolve() for d in watch_dirs]
        self.callback = callback
        self.recursive = recursive
        self.observer = Observer()
        self.logger = logging.getLogger(__name__)

        # Validate watch directories
        for watch_dir in self.watch_dirs:
            if not watch_dir.exists():
                self.logger.warning(f"Watch directory does not exist: {watch_dir}")

    def start(self) -> None:
        """Start watching directories"""
        handler = MarkdownFileHandler(callback=self.callback)

        for watch_dir in self.watch_dirs:
            if not watch_dir.exists():
                self.logger.warning(f"Skipping non-existent directory: {watch_dir}")
                continue

            self.logger.info(f"Watching directory: {watch_dir} (recursive={self.recursive})")
            self.observer.schedule(handler, str(watch_dir), recursive=self.recursive)

        self.observer.start()
        self.logger.info("FileWatcher started successfully")

    def stop(self) -> None:
        """Stop watching directories"""
        self.logger.info("Stopping FileWatcher...")
        self.observer.stop()
        self.observer.join()
        self.logger.info("FileWatcher stopped")

    def is_alive(self) -> bool:
        """Check if watcher is running"""
        return self.observer.is_alive()


# Example usage and testing
if __name__ == "__main__":
    import time

    # Setup logging
    logging.basicConfig(
        level=logging.INFO,
        format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
    )

    def test_callback(file_path: Path, event_type: str):
        """Test callback function"""
        print(f"[{event_type.upper()}] {file_path}")

    # Create watcher
    watcher = FileWatcher(
        watch_dirs=[
            "~/Documents/notes",
            "~/Projects"
        ],
        callback=test_callback,
        recursive=True
    )

    print("Starting FileWatcher...")
    print("Create or modify .md files in watched directories to test")
    print("Press Ctrl+C to stop")

    watcher.start()

    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        print("\nStopping...")
        watcher.stop()
