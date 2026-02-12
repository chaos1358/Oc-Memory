"""
Configuration Management for OC-Memory
Loads and validates YAML configuration files
"""

import yaml
from pathlib import Path
from typing import Dict, Any, Optional


class ConfigError(Exception):
    """Configuration-related errors"""
    pass


def load_config(config_path: str = "config.yaml") -> Dict[str, Any]:
    """
    Load configuration from YAML file

    Args:
        config_path: Path to YAML configuration file

    Returns:
        Configuration dictionary

    Raises:
        ConfigError: If config file not found or invalid
    """
    path = Path(config_path).expanduser()

    if not path.exists():
        raise ConfigError(f"Config file not found: {config_path}")

    try:
        with open(path, 'r', encoding='utf-8') as f:
            config = yaml.safe_load(f)
    except yaml.YAMLError as e:
        raise ConfigError(f"Invalid YAML in config file: {e}")

    # Validate required sections
    required_sections = ['watch', 'memory']
    for section in required_sections:
        if section not in config:
            raise ConfigError(f"Missing required section in config: {section}")

    return config


def validate_config(config: Dict[str, Any]) -> bool:
    """
    Validate configuration structure and values

    Args:
        config: Configuration dictionary

    Returns:
        True if valid

    Raises:
        ConfigError: If configuration is invalid
    """
    # Validate watch section
    if 'dirs' not in config['watch']:
        raise ConfigError("Missing 'dirs' in watch section")

    if not isinstance(config['watch']['dirs'], list):
        raise ConfigError("'watch.dirs' must be a list")

    # Validate memory section
    if 'dir' not in config['memory']:
        raise ConfigError("Missing 'dir' in memory section")

    return True


def expand_paths(config: Dict[str, Any]) -> Dict[str, Any]:
    """
    Expand ~ and environment variables in paths

    Args:
        config: Configuration dictionary

    Returns:
        Configuration with expanded paths
    """
    # Expand watch directories
    if 'watch' in config and 'dirs' in config['watch']:
        config['watch']['dirs'] = [
            str(Path(d).expanduser().resolve())
            for d in config['watch']['dirs']
        ]

    # Expand memory directory
    if 'memory' in config and 'dir' in config['memory']:
        config['memory']['dir'] = str(
            Path(config['memory']['dir']).expanduser().resolve()
        )

    return config


def get_config(config_path: str = "config.yaml") -> Dict[str, Any]:
    """
    Load, validate, and expand configuration

    Args:
        config_path: Path to YAML configuration file

    Returns:
        Validated and expanded configuration dictionary
    """
    config = load_config(config_path)
    validate_config(config)
    config = expand_paths(config)
    return config


# Example usage
if __name__ == "__main__":
    try:
        config = get_config("config.yaml")
        print("Configuration loaded successfully:")
        print(f"Watch directories: {config['watch']['dirs']}")
        print(f"Memory directory: {config['memory']['dir']}")
    except ConfigError as e:
        print(f"Configuration error: {e}")
