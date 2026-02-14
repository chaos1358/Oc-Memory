#!/usr/bin/env bash
set -euo pipefail

# Load environment variables from OpenClaw workspace env file.
# This keeps secrets out of launchd plist and supports launchd/startup execution.
ENV_FILE="$HOME/.openclaw/.env"
if [[ -f "$ENV_FILE" ]]; then
  set -a
  # shellcheck disable=SC1090
  source "$ENV_FILE"
  set +a
fi

exec /usr/local/bin/oc-guardian --config /usr/local/etc/oc-guardian/guardian.toml start
