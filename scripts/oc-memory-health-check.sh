#!/usr/bin/env bash

set -euo pipefail

OC_MEMORY_DIR="/Users/ailkisap/OC-Stack/Oc-Memory"
MEMORY_DIR="/Users/ailkisap/.openclaw/workspace/memory"
OBSIDIAN_DIR="/Users/ailkisap/OC-Stack/oc-memory-data/OC-Memory"
GUARDIAN_CFG="/usr/local/etc/oc-guardian/guardian.toml"
GUARDIAN_BIN="/usr/local/bin/oc-guardian"
LOG_FILE="$OC_MEMORY_DIR/oc-memory-health.log"
STATE_LOG="$OC_MEMORY_DIR/oc-memory-health-state.log"
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

  # Use runtime process scan first, then LaunchAgent state as authoritative fallback.
  openclaw_pid="$(ps -axo pid=,command= | grep -E "openclaw-gateway|openclaw" | awk '{print $1}' | tr '\n' ' ' || true)"
  oc_memory_pid="$(ps -axo pid=,command= | grep -E "memory_observer.py" | awk '{print $1}' | tr '\n' ' ' || true)"

  if [[ -z "$openclaw_pid" ]] && launchctl print gui/$(id -u)/ai.openclaw.gateway 2>/dev/null | grep -q "state = running"; then
    openclaw_pid="guardian-service"
  fi

  # If process list and service state are transiently missing,
  # fallback to guardian self-status to avoid false-negative noise.
  # Use plain string checks (not regex) for reliability across launchd context.
  openclaw_via_guardian="0"
  oc_memory_via_guardian="0"
  if grep -Fq "openclaw" /tmp/ocg_status.out 2>/dev/null; then
    openclaw_via_guardian="1"
  fi
  if grep -Fq "oc-memory" /tmp/ocg_status.out 2>/dev/null; then
    oc_memory_via_guardian="1"
  fi

  write_line "openclaw_pids: ${openclaw_pid:-none}"
  write_line "oc_memory_pids: ${oc_memory_pid:-none}"

  if [[ -z "$openclaw_pid" && "$openclaw_via_guardian" != "1" ]]; then
    write_line "[WARN] openclaw process and guardian state not found"
    status_code=1
  fi
  if [[ -z "$oc_memory_pid" && "$oc_memory_via_guardian" != "1" ]]; then
    write_line "[WARN] oc-memory process and guardian state not found"
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
  # Count hot md files excluding archive subtree so hot reflects active memory only.
  hot_count="$(find "$MEMORY_DIR" -path "$MEMORY_DIR/archive" -prune -o -name '*.md' -type f -print 2>/dev/null | wc -l | tr -d ' ')"
  warm_count="$(find "$MEMORY_DIR/archive" -type f -name '*.md' 2>/dev/null | wc -l | tr -d ' ')"
  obs_hot_count="$(find "$OBSIDIAN_DIR/hot" -type f -name '*.md' 2>/dev/null | wc -l | tr -d ' ')"
  obs_archive_count="$(find "$OBSIDIAN_DIR/archive" -type f -name '*.md' 2>/dev/null | wc -l | tr -d ' ')"
  if [[ -d "$OBSIDIAN_DIR/cold" ]]; then
    obs_cold_count="$(find "$OBSIDIAN_DIR/cold" -type f -name '*.md' 2>/dev/null | wc -l | tr -d ' ')"
  else
    obs_cold_count=0
  fi
  write_line "hot_count=$hot_count"
  write_line "warm_count=$warm_count"
  write_line "obsidian_hot_count=$obs_hot_count"
  write_line "obsidian_archive_count=$obs_archive_count"
  write_line "obsidian_cold_count=$obs_cold_count"

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
    write_line "cold archive: $(grep -a "Cold archive:" "$OC_MEMORY_DIR/oc-memory.log" | tail -n 1 || true)"
    write_line "obsidian sync: $(grep -a "Obsidian sync:" "$OC_MEMORY_DIR/oc-memory.log" | tail -n 1 || true)"
    write_line "dropbox sync: $(grep -a "Dropbox sync:" "$OC_MEMORY_DIR/oc-memory.log" | tail -n 1 || true)"
    write_line "reverse lookup: $(grep -a "reverse lookup" "$OC_MEMORY_DIR/oc-memory.log" | tail -n 1 || true)"
  fi

  write_line ""
  write_line "-- Recent failure reasons --"
  if [[ -f "$OC_MEMORY_DIR/oc-memory.log" ]]; then
    recent_failures="$(grep -aE "Cold archive failed|Cold archive check failed|Dropbox sync failed|Obsidian sync failed" "$OC_MEMORY_DIR/oc-memory.log" | tail -n 5 || true)"
    if [[ -n "$recent_failures" ]]; then
      while IFS= read -r line; do
        write_line "$line"
      done <<< "$recent_failures"
    else
      write_line "none"
    fi
  fi

  if [[ "$status_code" -eq 0 ]]; then
    write_line ""
    write_line "RESULT: PASS"
    current_result="PASS"
  else
    write_line ""
    write_line "RESULT: WARN/FAIL"
    current_result="WARN/FAIL"
  fi
}

if [[ "$status_code" -eq 0 ]]; then
  # Success-only mode: keep logs concise/no notification spam.
  printf "%s\tRESULT=%s\topenclaw=%s\toc-memory=%s\thot=%s\twarm=%s\tobs_hot=%s\tobs_arch=%s\tobs_cold=%s\n" \
    "$NOW" "$current_result" "${openclaw_pid:-none}" "${oc_memory_pid:-none}" "$hot_count" "$warm_count" "$obs_hot_count" "$obs_archive_count" "$obs_cold_count" \
    >> "$STATE_LOG"
  if [ -f "$STATE_LOG" ]; then
    tail -n 20 "$STATE_LOG" > "${STATE_LOG}.tmp" && mv "${STATE_LOG}.tmp" "$STATE_LOG"
  fi
  rm -f "$TMP_REPORT" /tmp/ocg_status.out
  exit 0
fi

# Failure only: write detailed report and print once.
{
  cat "$TMP_REPORT"
  printf "\n"
} | tee -a "$LOG_FILE"
printf "%s\tRESULT=%s\topenclaw=%s\toc-memory=%s\thot=%s\twarm=%s\tobs_hot=%s\tobs_arch=%s\tobs_cold=%s\n" \
  "$NOW" "$current_result" "${openclaw_pid:-none}" "${oc_memory_pid:-none}" "$hot_count" "$warm_count" "$obs_hot_count" "$obs_archive_count" "$obs_cold_count" \
  >> "$STATE_LOG"
if [ -f "$STATE_LOG" ]; then
  tail -n 20 "$STATE_LOG" > "${STATE_LOG}.tmp" && mv "${STATE_LOG}.tmp" "$STATE_LOG"
fi
rm -f "$TMP_REPORT" /tmp/ocg_status.out
exit 1
