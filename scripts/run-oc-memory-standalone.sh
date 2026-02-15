#!/usr/bin/env bash
set -euo pipefail

# Standalone OC-Memory launcher (for OpenClaw-independent recovery)
# Loads runtime secrets from ~/.openclaw/.env and starts only memory_observer.py.

APP_DIR="/Users/ailkisap/OC-Stack/Oc-Memory"
ENV_FILE="$HOME/.openclaw/.env"

if [[ -f "$ENV_FILE" ]]; then
  set -a
  # shellcheck disable=SC1090
  source "$ENV_FILE"
  set +a
fi

cd "$APP_DIR"
exec "$APP_DIR/venv/bin/python3" memory_observer.py
