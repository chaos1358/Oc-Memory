#!/usr/bin/env python3
"""
OC-Memory Setup Wizard
Interactive TUI for configuring OC-Memory-Sidecar

Usage:
    python setup.py
    # or
    oc-memory setup
"""

import os
import sys
from pathlib import Path
from typing import Dict, Any

try:
    import questionary
    from questionary import Style
except ImportError:
    print("❌ Required package 'questionary' not found.")
    print("📦 Installing dependencies...")
    os.system(f"{sys.executable} -m pip install questionary pyyaml")
    import questionary
    from questionary import Style

import yaml


# ============================================================================
# Custom Style
# ============================================================================

custom_style = Style([
    ('qmark', 'fg:#673ab7 bold'),       # question mark
    ('question', 'bold'),                # question text
    ('answer', 'fg:#f44336 bold'),       # submitted answer
    ('pointer', 'fg:#673ab7 bold'),      # pointer for select
    ('highlighted', 'fg:#673ab7 bold'),  # highlighted choice
    ('selected', 'fg:#cc5454'),          # selected choice
    ('separator', 'fg:#cc5454'),         # separator
    ('instruction', ''),                 # instruction
    ('text', ''),                        # plain text
    ('disabled', 'fg:#858585 italic')    # disabled choice
])


# ============================================================================
# Setup Wizard
# ============================================================================

class SetupWizard:
    """Interactive setup wizard for OC-Memory-Sidecar"""

    def __init__(self):
        self.config = {}
        self.config_path = Path.home() / ".openclaw" / "oc-memory" / "config.yaml"

    def run(self):
        """Run the setup wizard"""
        self.print_banner()

        if not self.confirm_start():
            print("👋 Setup cancelled.")
            return

        # Step 1: Basic Configuration
        self.setup_basic()

        # Step 2: Memory Tiers
        self.setup_memory_tiers()

        # Step 3: LLM Configuration
        self.setup_llm()

        # Step 4: Optional Features
        self.setup_optional_features()

        # Step 5: Review and Save
        self.review_and_save()

        # Step 6: Post-installation
        self.post_install()

    def print_banner(self):
        """Print welcome banner"""
        banner = """
╔═══════════════════════════════════════════════════════════╗
║                                                           ║
║   🧠 OC-Memory Setup Wizard                              ║
║                                                           ║
║   OpenClaw Observational Memory System                   ║
║   Version 1.0.0                                          ║
║                                                           ║
╚═══════════════════════════════════════════════════════════╝
        """
        print(banner)

    def confirm_start(self) -> bool:
        """Confirm to start setup"""
        return questionary.confirm(
            "Would you like to start the setup?",
            default=True,
            style=custom_style
        ).ask()

    def setup_basic(self):
        """Setup basic configuration"""
        print("\n" + "="*60)
        print("📋 STEP 1: Basic Configuration")
        print("="*60 + "\n")

        # OpenClaw log file path
        default_log_path = str(Path.home() / ".openclaw" / "logs" / "chat.log")
        log_path = questionary.path(
            "OpenClaw log file path:",
            default=default_log_path,
            style=custom_style
        ).ask()

        # Memory file path
        default_memory_path = str(Path.home() / ".openclaw" / "active_memory.md")
        memory_path = questionary.path(
            "Active memory file path:",
            default=default_memory_path,
            style=custom_style
        ).ask()

        # Poll interval
        poll_interval = questionary.text(
            "Log monitoring interval (seconds):",
            default="1.0",
            validate=lambda x: x.replace('.', '').isdigit(),
            style=custom_style
        ).ask()

        self.config['logs'] = {
            'path': log_path,
            'poll_interval': float(poll_interval)
        }

        self.config['memory'] = {
            'active_file': memory_path
        }

    def setup_memory_tiers(self):
        """Setup memory tier configuration"""
        print("\n" + "="*60)
        print("🗂️  STEP 2: Memory Tiers Configuration")
        print("="*60 + "\n")

        print("OC-Memory uses a 3-tier memory system:")
        print("  🔥 Hot Memory (0-90 days): Fast, semantic search")
        print("  ♨️  Warm Memory (90-365 days): Archive storage")
        print("  ❄️  Cold Memory (365+ days): Long-term backup\n")

        # Hot Memory
        hot_ttl = questionary.text(
            "Hot Memory TTL (days):",
            default="90",
            validate=lambda x: x.isdigit() and int(x) > 0,
            style=custom_style
        ).ask()

        hot_storage = questionary.select(
            "Hot Memory storage backend:",
            choices=[
                "chromadb (Recommended - Semantic search)",
                "file (Simple file storage)"
            ],
            style=custom_style
        ).ask()

        # Warm Memory
        warm_ttl = questionary.text(
            "Warm Memory TTL (days):",
            default="365",
            validate=lambda x: x.isdigit() and int(x) > int(hot_ttl),
            style=custom_style
        ).ask()

        # ChromaDB path
        if "chromadb" in hot_storage:
            db_path = questionary.path(
                "ChromaDB storage path:",
                default=str(Path.home() / ".openclaw" / "memory_db"),
                style=custom_style
            ).ask()
        else:
            db_path = None

        # Archive path
        archive_path = questionary.path(
            "Archive storage path:",
            default=str(Path.home() / ".openclaw" / "memory_archive"),
            style=custom_style
        ).ask()

        self.config['memory'].update({
            'hot': {
                'storage': 'chromadb' if 'chromadb' in hot_storage else 'file',
                'db_path': db_path,
                'ttl_days': int(hot_ttl),
                'max_tokens': 30000
            },
            'warm': {
                'storage': 'markdown',
                'archive_path': archive_path,
                'ttl_days': int(warm_ttl)
            }
        })

    def setup_llm(self):
        """Setup LLM configuration"""
        print("\n" + "="*60)
        print("🤖 STEP 3: LLM Configuration")
        print("="*60 + "\n")

        # LLM Provider
        provider = questionary.select(
            "Select LLM provider for Observer/Reflector:",
            choices=[
                "Google Gemini (Recommended - Fast & Free tier)",
                "OpenAI GPT-4",
                "OpenAI GPT-3.5",
                "Anthropic Claude",
                "Custom (OpenAI-compatible API)"
            ],
            style=custom_style
        ).ask()

        # Model mapping
        model_map = {
            "Google Gemini": "google/gemini-2.5-flash",
            "OpenAI GPT-4": "gpt-4-turbo",
            "OpenAI GPT-3.5": "gpt-3.5-turbo",
            "Anthropic Claude": "claude-3-sonnet-20240229",
            "Custom": None
        }

        provider_key = [k for k in model_map.keys() if k in provider][0]
        model = model_map[provider_key]

        if provider_key == "Custom":
            model = questionary.text(
                "Enter model name:",
                style=custom_style
            ).ask()

            api_base = questionary.text(
                "Enter API base URL:",
                default="http://localhost:8000/v1",
                style=custom_style
            ).ask()
        else:
            api_base = None

        # API Key
        api_key_name = {
            "Google Gemini": "GOOGLE_API_KEY",
            "OpenAI GPT-4": "OPENAI_API_KEY",
            "OpenAI GPT-3.5": "OPENAI_API_KEY",
            "Anthropic Claude": "ANTHROPIC_API_KEY",
            "Custom": "CUSTOM_API_KEY"
        }[provider_key]

        has_api_key = questionary.confirm(
            f"Do you have {api_key_name} set in environment?",
            default=False,
            style=custom_style
        ).ask()

        if not has_api_key:
            api_key = questionary.password(
                f"Enter {api_key_name}:",
                style=custom_style
            ).ask()

            # Save to .env
            self.save_api_key(api_key_name, api_key)

        self.config['observer'] = {
            'model': model,
            'temperature': 0.3,
            'max_output_tokens': 2000
        }

        if api_base:
            self.config['observer']['api_base'] = api_base

        self.config['reflector'] = {
            'model': model,
            'temperature': 0.0,
            'max_output_tokens': 5000
        }

        if api_base:
            self.config['reflector']['api_base'] = api_base

    def setup_optional_features(self):
        """Setup optional features"""
        print("\n" + "="*60)
        print("✨ STEP 4: Optional Features")
        print("="*60 + "\n")

        # Obsidian Integration
        use_obsidian = questionary.confirm(
            "Enable Obsidian integration for Cold Memory? (P2 - Optional)",
            default=False,
            style=custom_style
        ).ask()

        if use_obsidian:
            print("\n📝 Obsidian requires yakitrak/obsidian-cli")
            print("   Install: brew install yakitrak/yakitrak/obsidian-cli\n")

            has_cli = questionary.confirm(
                "Is Obsidian CLI (yakitrak) installed?",
                default=False,
                style=custom_style
            ).ask()

            if has_cli:
                vault_name = questionary.text(
                    "Obsidian vault name:",
                    default="Main",
                    style=custom_style
                ).ask()

                vault_path = questionary.path(
                    "Obsidian vault path:",
                    default=str(Path.home() / "Documents" / "Obsidian" / "Main"),
                    style=custom_style
                ).ask()

                self.config['memory']['cold'] = {
                    'storage': 'obsidian',
                    'vault_path': vault_path,
                    'memory_folder': 'OC-Memory'
                }

                self.config['obsidian'] = {
                    'enabled': True,
                    'cli': 'obsidian-cli',
                    'vault_name': vault_name
                }
            else:
                print("⚠️  Obsidian CLI not installed. Skipping Obsidian integration.")
                self.config['obsidian'] = {'enabled': False}
        else:
            self.config['obsidian'] = {'enabled': False}

        # Dropbox Integration
        if use_obsidian:
            use_dropbox = questionary.confirm(
                "Enable Dropbox sync for cloud backup? (P2 - Optional)",
                default=False,
                style=custom_style
            ).ask()

            if use_dropbox:
                print("\n☁️  Dropbox requires API access token")
                print("   Get token: https://www.dropbox.com/developers/apps\n")

                has_token = questionary.confirm(
                    "Do you have Dropbox access token?",
                    default=False,
                    style=custom_style
                ).ask()

                if has_token:
                    dropbox_token = questionary.password(
                        "Enter Dropbox access token:",
                        style=custom_style
                    ).ask()

                    vault_path_in_dropbox = questionary.text(
                        "Dropbox vault path:",
                        default="/Apps/Obsidian/Main",
                        style=custom_style
                    ).ask()

                    self.config['dropbox'] = {
                        'enabled': True,
                        'access_token': f"${{{api_key_name or 'DROPBOX_ACCESS_TOKEN'}}}",
                        'vault_path': vault_path_in_dropbox
                    }

                    # Save to .env
                    self.save_api_key('DROPBOX_ACCESS_TOKEN', dropbox_token)
                else:
                    print("⚠️  Dropbox token not provided. Skipping Dropbox integration.")
                    self.config['dropbox'] = {'enabled': False}
            else:
                self.config['dropbox'] = {'enabled': False}
        else:
            self.config['dropbox'] = {'enabled': False}

    def review_and_save(self):
        """Review configuration and save"""
        print("\n" + "="*60)
        print("📄 STEP 5: Review Configuration")
        print("="*60 + "\n")

        # Print configuration summary
        print("Configuration Summary:")
        print("-" * 60)
        print(f"✅ Log monitoring: {self.config['logs']['path']}")
        print(f"✅ Hot Memory: {self.config['memory']['hot']['storage']} ({self.config['memory']['hot']['ttl_days']} days)")
        print(f"✅ Warm Memory: {self.config['memory']['warm']['ttl_days']} days")
        print(f"✅ LLM: {self.config['observer']['model']}")
        print(f"{'✅' if self.config['obsidian']['enabled'] else '❌'} Obsidian: {'Enabled' if self.config['obsidian']['enabled'] else 'Disabled'}")
        print(f"{'✅' if self.config.get('dropbox', {}).get('enabled') else '❌'} Dropbox: {'Enabled' if self.config.get('dropbox', {}).get('enabled') else 'Disabled'}")
        print("-" * 60 + "\n")

        # Confirm
        confirmed = questionary.confirm(
            "Save this configuration?",
            default=True,
            style=custom_style
        ).ask()

        if not confirmed:
            print("❌ Configuration not saved. Exiting.")
            sys.exit(1)

        # Save configuration
        self.save_config()

    def save_config(self):
        """Save configuration to file"""
        # Ensure directory exists
        self.config_path.parent.mkdir(parents=True, exist_ok=True)

        # Write YAML
        with open(self.config_path, 'w') as f:
            yaml.dump(self.config, f, default_flow_style=False, sort_keys=False)

        print(f"\n✅ Configuration saved to: {self.config_path}")

    def save_api_key(self, key_name: str, value: str):
        """Save API key to .env file"""
        env_path = self.config_path.parent / ".env"

        # Read existing .env
        existing_keys = {}
        if env_path.exists():
            with open(env_path, 'r') as f:
                for line in f:
                    if '=' in line and not line.startswith('#'):
                        k, v = line.strip().split('=', 1)
                        existing_keys[k] = v

        # Add/update key
        existing_keys[key_name] = value

        # Write .env
        with open(env_path, 'w') as f:
            f.write("# OC-Memory Environment Variables\n")
            f.write("# DO NOT COMMIT THIS FILE TO GIT\n\n")
            for k, v in existing_keys.items():
                f.write(f"{k}={v}\n")

        # Set permissions (Unix only)
        if os.name != 'nt':
            os.chmod(env_path, 0o600)

        print(f"🔑 API key saved to: {env_path}")

    def post_install(self):
        """Post-installation instructions"""
        print("\n" + "="*60)
        print("🎉 STEP 6: Installation Complete!")
        print("="*60 + "\n")

        print("Next steps:")
        print("1. Review configuration:")
        print(f"   cat {self.config_path}\n")

        print("2. Start OC-Memory daemon:")
        print("   python memory_observer.py\n")

        print("3. Add to OpenClaw System Prompt:")
        print("   Before responding, read ~/.openclaw/active_memory.md\n")

        if self.config['obsidian']['enabled']:
            print("4. Setup Obsidian:")
            print(f"   obsidian-cli set-default {self.config['obsidian']['vault_name']}\n")

        print("📚 Documentation: ./specs/")
        print("🐛 Issues: https://github.com/[username]/oc-memory/issues")
        print("\nThank you for using OC-Memory! 🧠")


# ============================================================================
# Main Entry Point
# ============================================================================

def main():
    """Main entry point"""
    wizard = SetupWizard()

    try:
        wizard.run()
    except KeyboardInterrupt:
        print("\n\n👋 Setup cancelled by user.")
        sys.exit(1)
    except Exception as e:
        print(f"\n❌ Error: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()
