"""
Memory Writer for OC-Memory
Writes files to OpenClaw Memory directory with metadata
"""

import logging
import shutil
from datetime import datetime
from pathlib import Path
from typing import Dict, Any, Optional


class MemoryWriterError(Exception):
    """Memory writer related errors"""
    pass


class MemoryWriter:
    """
    Writes memory files to OpenClaw Memory directory
    Handles file copying, metadata, and conflict resolution
    """

    def __init__(self, memory_dir: str, max_versions_per_source: int = 5):
        """
        Args:
            memory_dir: OpenClaw Memory directory path
                       (typically ~/.openclaw/workspace/memory)
        """
        self.memory_dir = Path(memory_dir).expanduser().resolve()
        self.max_versions_per_source = max(1, int(max_versions_per_source))
        self.logger = logging.getLogger(__name__)

        # Create memory directory if it doesn't exist
        self.memory_dir.mkdir(parents=True, exist_ok=True)
        self.logger.info(f"Memory directory: {self.memory_dir}")

    def copy_to_memory(
        self,
        source_file: Path,
        category: Optional[str] = None,
        preserve_metadata: bool = True
    ) -> Path:
        """
        Copy file to memory directory

        Args:
            source_file: Path to source file
            category: Optional category subdirectory
            preserve_metadata: Preserve file timestamps

        Returns:
            Path to copied file

        Raises:
            MemoryWriterError: If source file not found or copy fails
        """
        if not source_file.exists():
            raise MemoryWriterError(f"Source file not found: {source_file}")

        # Determine target directory
        if category:
            target_dir = self.memory_dir / category
            target_dir.mkdir(exist_ok=True)
        else:
            target_dir = self.memory_dir

        # Determine target filename (handle conflicts)
        target_file = target_dir / source_file.name

        if target_file.exists():
            # Add timestamp to avoid conflicts
            timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
            stem = source_file.stem
            suffix = source_file.suffix
            target_file = target_dir / f"{stem}_{timestamp}{suffix}"
            self.logger.debug(f"Filename conflict resolved: {target_file.name}")

        # Copy file
        try:
            if preserve_metadata:
                shutil.copy2(source_file, target_file)
            else:
                shutil.copy(source_file, target_file)

            # Keep only recent versions to avoid hot folder bloat
            self._enforce_version_retention(target_dir=target_dir, source_file=source_file)
            self.logger.info(f"Copied to memory: {target_file}")
            return target_file

        except Exception as e:
            raise MemoryWriterError(f"Failed to copy file: {e}")

    def _enforce_version_retention(self, target_dir: Path, source_file: Path) -> None:
        """Keep only the latest N versions per source file to prevent unbounded growth."""
        if self.max_versions_per_source <= 0:
            return

        stem = source_file.stem
        suffix = source_file.suffix
        pattern = f"{stem}*{suffix}"
        files = sorted(
            [p for p in target_dir.glob(pattern) if p.is_file()],
            key=lambda p: p.stat().st_mtime,
            reverse=True
        )

        for f in files[self.max_versions_per_source:]:
            try:
                f.unlink()
                self.logger.debug(f"Pruned old memory version: {f}")
            except Exception as e:
                self.logger.warning(f"Failed to prune memory version {f}: {e}")


    def write_memory_entry(
        self,
        content: str,
        filename: str,
        category: Optional[str] = None
    ) -> Path:
        """
        Create new memory entry from content

        Args:
            content: File content to write
            filename: Target filename
            category: Optional category subdirectory

        Returns:
            Path to created file

        Raises:
            MemoryWriterError: If write fails
        """
        # Determine target directory
        if category:
            target_dir = self.memory_dir / category
            target_dir.mkdir(exist_ok=True)
        else:
            target_dir = self.memory_dir

        target_file = target_dir / filename

        # Write file
        try:
            with open(target_file, 'w', encoding='utf-8') as f:
                f.write(content)

            self.logger.info(f"Created memory entry: {target_file}")
            return target_file

        except Exception as e:
            raise MemoryWriterError(f"Failed to write file: {e}")

    def add_metadata(
        self,
        file_path: Path,
        metadata: Dict[str, Any]
    ) -> None:
        """
        Add YAML frontmatter metadata to file

        Args:
            file_path: Path to file
            metadata: Dictionary of metadata fields

        Raises:
            MemoryWriterError: If metadata addition fails
        """
        if not file_path.exists():
            raise MemoryWriterError(f"File not found: {file_path}")

        try:
            # Read existing content
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()

            # Remove existing frontmatter if present
            if content.startswith("---"):
                parts = content.split("---", 2)
                if len(parts) >= 3:
                    content = parts[2].lstrip()

            # Create new frontmatter
            frontmatter = "---\n"
            for key, value in metadata.items():
                # Handle different value types
                if isinstance(value, str):
                    frontmatter += f"{key}: {value}\n"
                elif isinstance(value, (int, float, bool)):
                    frontmatter += f"{key}: {value}\n"
                elif isinstance(value, list):
                    frontmatter += f"{key}:\n"
                    for item in value:
                        frontmatter += f"  - {item}\n"
                else:
                    frontmatter += f"{key}: {str(value)}\n"
            frontmatter += "---\n\n"

            # Write new content
            new_content = frontmatter + content
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(new_content)

            self.logger.debug(f"Added metadata to: {file_path}")

        except Exception as e:
            raise MemoryWriterError(f"Failed to add metadata: {e}")

    def get_category_from_path(self, file_path: Path) -> str:
        """
        Auto-detect category from file path

        Args:
            file_path: Path to file

        Returns:
            Category name
        """
        path_str = str(file_path).lower()

        # Simple rule-based categorization
        if 'project' in path_str:
            return 'projects'
        elif 'note' in path_str:
            return 'notes'
        elif 'doc' in path_str or 'document' in path_str:
            return 'documents'
        elif 'meeting' in path_str:
            return 'meetings'
        else:
            return 'general'


# Example usage and testing
if __name__ == "__main__":
    import tempfile

    # Setup logging
    logging.basicConfig(
        level=logging.INFO,
        format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
    )

    # Create temporary memory directory for testing
    with tempfile.TemporaryDirectory() as temp_dir:
        print(f"Testing with temporary directory: {temp_dir}")

        writer = MemoryWriter(temp_dir)

        # Test 1: Create memory entry
        print("\n--- Test 1: Create memory entry ---")
        content = """# Test Memory Entry

## Overview
This is a test memory file.

## Key Points
- Point 1
- Point 2
"""
        result = writer.write_memory_entry(
            content=content,
            filename="test_entry.md",
            category="tests"
        )
        print(f"Created: {result}")

        # Test 2: Add metadata
        print("\n--- Test 2: Add metadata ---")
        writer.add_metadata(result, {
            "created": datetime.now().isoformat(),
            "category": "tests",
            "priority": "medium",
            "tags": ["test", "example"]
        })
        print(f"Metadata added to: {result}")

        # Read and display result
        with open(result, 'r') as f:
            print("\nFile content:")
            print(f.read())

        # Test 3: Copy file (if test file exists)
        print("\n--- Test 3: Copy file ---")
        test_source = Path("~/Documents/notes/test.md").expanduser()
        if test_source.exists():
            copied = writer.copy_to_memory(test_source, category="copied")
            print(f"Copied: {copied}")
        else:
            print(f"Skipped (source file not found: {test_source})")

        print("\n--- Tests completed ---")
