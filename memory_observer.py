#!/usr/bin/env python3
"""
OC-Memory Observer Daemon
Monitors user directories and syncs to OpenClaw Memory

Usage:
    python memory_observer.py [--config CONFIG_PATH]
"""

import argparse
import logging
import signal
import sys
import time
from datetime import datetime
from pathlib import Path

from lib.config import get_config, ConfigError
from lib.file_watcher import FileWatcher
from lib.memory_writer import MemoryWriter, MemoryWriterError


class MemoryObserver:
    """
    Main daemon process for OC-Memory
    Orchestrates FileWatcher and MemoryWriter components
    """

    def __init__(self, config_path: str = "config.yaml"):
        """
        Args:
            config_path: Path to YAML configuration file
        """
        self.config_path = config_path
        self.logger = logging.getLogger(__name__)

        # Load configuration
        try:
            self.config = get_config(config_path)
            self.logger.info(f"Configuration loaded from: {config_path}")
        except ConfigError as e:
            self.logger.error(f"Configuration error: {e}")
            raise

        # Initialize components
        self.memory_writer = MemoryWriter(
            memory_dir=self.config['memory']['dir']
        )

        self.file_watcher = FileWatcher(
            watch_dirs=self.config['watch']['dirs'],
            callback=self.on_file_change,
            recursive=self.config['watch'].get('recursive', True)
        )

        self.running = False
        self.files_processed = 0
        self.errors = 0

    def on_file_change(self, file_path: Path, event_type: str) -> None:
        """
        Handle file change events from FileWatcher

        Args:
            file_path: Path to changed file
            event_type: Type of event ('created' or 'modified')
        """
        try:
            self.logger.info(f"Processing file: {file_path} ({event_type})")

            # Auto-detect category
            category = self._detect_category(file_path)

            # Copy to memory directory
            target_file = self.memory_writer.copy_to_memory(
                source_file=file_path,
                category=category
            )

            # Add metadata
            metadata = {
                "source": str(file_path),
                "synced_at": datetime.now().isoformat(),
                "category": category,
                "event_type": event_type,
                "oc_memory_version": "0.1.0"
            }

            self.memory_writer.add_metadata(target_file, metadata)

            self.files_processed += 1
            self.logger.info(
                f"Synced to memory: {target_file} "
                f"(total: {self.files_processed})"
            )

        except MemoryWriterError as e:
            self.errors += 1
            self.logger.error(f"Error processing file {file_path}: {e}")

        except Exception as e:
            self.errors += 1
            self.logger.exception(f"Unexpected error processing {file_path}: {e}")

    def _detect_category(self, file_path: Path) -> str:
        """
        Auto-detect category from file path

        Args:
            file_path: Path to file

        Returns:
            Category name
        """
        # Check if auto-categorization is enabled
        if not self.config['memory'].get('auto_categorize', True):
            return 'general'

        # Use MemoryWriter's category detection
        return self.memory_writer.get_category_from_path(file_path)

    def start(self) -> None:
        """Start the observer daemon"""
        self.logger.info("=" * 60)
        self.logger.info("Starting OC-Memory Observer")
        self.logger.info("=" * 60)
        self.logger.info(f"Watch directories: {self.config['watch']['dirs']}")
        self.logger.info(f"Memory directory: {self.config['memory']['dir']}")
        self.logger.info("=" * 60)

        # Setup signal handlers
        signal.signal(signal.SIGINT, self._signal_handler)
        signal.signal(signal.SIGTERM, self._signal_handler)

        # Start file watcher
        try:
            self.file_watcher.start()
        except Exception as e:
            self.logger.error(f"Failed to start FileWatcher: {e}")
            raise

        self.running = True
        self.logger.info("OC-Memory Observer started successfully")
        self.logger.info("Monitoring for file changes... (Press Ctrl+C to stop)")

        # Main loop
        try:
            while self.running:
                time.sleep(1)
        except KeyboardInterrupt:
            self.logger.info("Received keyboard interrupt")

        self.stop()

    def stop(self) -> None:
        """Stop the observer daemon"""
        self.logger.info("Stopping OC-Memory Observer...")
        self.running = False

        # Stop file watcher
        if self.file_watcher.is_alive():
            self.file_watcher.stop()

        # Print statistics
        self.logger.info("=" * 60)
        self.logger.info("OC-Memory Observer Statistics")
        self.logger.info("=" * 60)
        self.logger.info(f"Files processed: {self.files_processed}")
        self.logger.info(f"Errors: {self.errors}")
        self.logger.info("=" * 60)
        self.logger.info("OC-Memory Observer stopped")

    def _signal_handler(self, signum: int, frame) -> None:
        """
        Handle system signals

        Args:
            signum: Signal number
            frame: Current stack frame
        """
        self.logger.info(f"Received signal {signum}")
        self.stop()
        sys.exit(0)


def setup_logging(config: dict) -> None:
    """
    Setup logging configuration

    Args:
        config: Configuration dictionary
    """
    log_config = config.get('logging', {})
    level = getattr(logging, log_config.get('level', 'INFO'))
    log_file = log_config.get('file', 'oc-memory.log')
    console = log_config.get('console', True)

    # Create handlers
    handlers = []

    # File handler
    file_handler = logging.FileHandler(log_file, encoding='utf-8')
    file_handler.setFormatter(
        logging.Formatter(
            '%(asctime)s - %(name)s - %(levelname)s - %(message)s'
        )
    )
    handlers.append(file_handler)

    # Console handler
    if console:
        console_handler = logging.StreamHandler()
        console_handler.setFormatter(
            logging.Formatter(
                '%(asctime)s - %(levelname)s - %(message)s',
                datefmt='%H:%M:%S'
            )
        )
        handlers.append(console_handler)

    # Configure root logger
    logging.basicConfig(
        level=level,
        handlers=handlers
    )


def main() -> None:
    """Main entry point"""
    # Parse command line arguments
    parser = argparse.ArgumentParser(
        description='OC-Memory Observer Daemon'
    )
    parser.add_argument(
        '--config',
        default='config.yaml',
        help='Path to configuration file (default: config.yaml)'
    )
    parser.add_argument(
        '--version',
        action='version',
        version='OC-Memory 0.1.0'
    )

    args = parser.parse_args()

    # Check if config file exists
    if not Path(args.config).exists():
        print(f"Error: Configuration file not found: {args.config}")
        print(f"Please copy config.example.yaml to {args.config} and customize it")
        sys.exit(1)

    # Load config for logging setup
    try:
        config = get_config(args.config)
    except ConfigError as e:
        print(f"Configuration error: {e}")
        sys.exit(1)

    # Setup logging
    setup_logging(config)

    # Start observer
    try:
        observer = MemoryObserver(config_path=args.config)
        observer.start()
    except ConfigError as e:
        logging.error(f"Configuration error: {e}")
        sys.exit(1)
    except KeyboardInterrupt:
        logging.info("Interrupted by user")
        sys.exit(0)
    except Exception as e:
        logging.exception(f"Fatal error: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()
