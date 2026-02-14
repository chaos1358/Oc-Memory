#!/usr/bin/env bash

set -euo pipefail

OC_MEMORY_DIR="/Users/ailkisap/Documents/Oc-Memory"
MEMORY_DIR="/Users/ailkisap/.openclaw/workspace/memory"
OBSIDIAN_DIR="/Users/ailkisap/Documents/Obsidian Vault/OC-Memory"
GUARDIAN_CFG="/usr/local/etc/oc-guardian/guardian.toml"
GUARDIAN_BIN="/usr/local/bin/oc-guardian"
LOG_FILE="$OC_MEMORY_DIR/oc-memory-health.log"
TMP_REPORT="$(mktemp)"
NOW="$(date '+%Y-%m-%d %H:%M:%S %Z')"

status_code=0

write_line() {
  printf '%s\n' "$1" >> "$TMP_REPORT"
}

{
  write_line "=================================================="
  write_line "[$NOW] OC-Memory Daily Health Check"

  write_line ""
  write_line "-- Guardian status --"
  if cd /usr/local/etc/oc-guardian && "$GUARDIAN_BIN" status > /tmp/ocg_status.out 2>&1; then
    write_line "guardian_status: ok"
    while IFS= read -r line; do
      write_line "$line"
    done < /tmp/ocg_status.out
  else
    write_line "guardian_status: FAIL"
    while IFS= read -r line; do
      write_line "$line"
    done < /tmp/ocg_status.out
    status_code=1
  fi

  write_line ""
  write_line "-- Process check --"
  openclaw_pid="$(pgrep -if "openclaw-gateway|openclaw" | tr '\n' ' ' || true)"
  if [[ -z "$openclaw_pid" ]]; then
    if launchctl print gui/$(id -u)/ai.openclaw.gateway 2>/dev/null | grep -q "state ="; then
      openclaw_pid="guardian-service"
    fi
  fi

  oc_memory_pid="$(pgrep -if "memory_observer.py" | tr '\n' ' ' || true)"
  write_line "openclaw_pids: ${openclaw_pid:-none}"
  write_line "oc_memory_pids: ${oc_memory_pid:-none}"
  if [[ -z "$openclaw_pid" ]]; then
    write_line "[WARN] openclaw process not found"
    status_code=1
  fi
  if [[ -z "$oc_memory_pid" ]]; then
    write_line "[WARN] oc-memory observer process not found"
    status_code=1
  fi

  write_line ""
  write_line "-- Files check --"
  for f in "$GUARDIAN_CFG" "$OC_MEMORY_DIR/config.yaml" "$MEMORY_DIR" "$OBSIDIAN_DIR" "$OBSIDIAN_DIR/hot" "$MEMORY_DIR/archive"; do
    if [[ -e "$f" ]]; then
      write_line "ok: $f"
    else
      write_line "MISSING: $f"
      status_code=1
    fi
  done

  write_line ""
  write_line "-- Memory tier stats --"
  hot_count="$(find "$MEMORY_DIR" -maxdepth 1 -type f -name '*.md' | wc -l | tr -d ' ')"
  warm_count="$(find "$MEMORY_DIR/archive" -type f -name '*.md' 2>/dev/null | wc -l | tr -d ' ')"
  obs_hot_count="$(find "$OBSIDIAN_DIR/hot" -type f -name '*.md' 2>/dev/null | wc -l | tr -d ' ')"
  obs_archive_count="$(find "$OBSIDIAN_DIR/archive" -type f -name '*.md' 2>/dev/null | wc -l | tr -d ' ')"
  write_line "hot_count=$hot_count"
  write_line "warm_count=$warm_count"
  write_line "obsidian_hot_count=$obs_hot_count"
  write_line "obsidian_archive_count=$obs_archive_count"

  write_line ""
  write_line "-- Log summary (last 10 lines) --"
  if [[ -f "$OC_MEMORY_DIR/oc-memory.log" ]]; then
    tail -n 10 "$OC_MEMORY_DIR/oc-memory.log" >> "$TMP_REPORT"
  else
    write_line "[WARN] oc-memory.log not found"
    status_code=1
  fi

  write_line ""
  write_line "-- Recent observer summary --"
  if [[ -f "$OC_MEMORY_DIR/oc-memory.log" ]]; then
    write_line "ttl moves: $(grep -a "TTL archive" "$OC_MEMORY_DIR/oc-memory.log" | tail -n 1 || true)"
    write_line "obsidian sync: $(grep -a "Obsidian sync:" "$OC_MEMORY_DIR/oc-memory.log" | tail -n 1 || true)"
    write_line "reverse lookup: $(grep -a "reverse lookup" "$OC_MEMORY_DIR/oc-memory.log" | tail -n 1 || true)"
  fi

  if [[ "$status_code" -eq 0 ]]; then
    write_line ""
    write_line "RESULT: PASS"
  else
    write_line ""
    write_line "RESULT: WARN/FAIL"
  fi
} 

if [[ "$status_code" -eq 0 ]]; then
  # Success-only mode: keep logs concise/no notification spam.
  rm -f "$TMP_REPORT" /tmp/ocg_status.out
  exit 0
fi

# Failure only: write detailed report and print once.
{
  cat "$TMP_REPORT"
  printf "\n"
} | tee -a "$LOG_FILE"

rm -f "$TMP_REPORT" /tmp/ocg_status.out
exit 1
