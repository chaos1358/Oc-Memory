#!/usr/bin/env bash
set -euo pipefail

# Purpose:
#   Keep OC-Memory hot folder (.openclaw/workspace/memory/general) growth bounded.
#   - retain latest N versions per source file
#   - remove files older than TTL days (optional)
#   - report current size before/after

MEMORY_DIR="${HOME}/.openclaw/workspace/memory/general"
RETAIN_PER_SOURCE="${RETAIN_PER_SOURCE:-6}"
MAX_AGE_DAYS="${MAX_AGE_DAYS:-0}"
DRY_RUN="${DRY_RUN:-1}"

if [[ ! -d "$MEMORY_DIR" ]]; then
  echo "[skip] memory dir not found: $MEMORY_DIR"
  exit 0
fi

if [[ "$DRY_RUN" != "0" ]]; then
  echo "[dry-run] defaulting ON. set DRY_RUN=0 to actually delete files"
fi

before_bytes=$(du -sk "$MEMORY_DIR" | awk '{print $1}')

# Track per source-group retention by basename stem before optional timestamp suffix
python3 - "$MEMORY_DIR" "$RETAIN_PER_SOURCE" "$MAX_AGE_DAYS" "$DRY_RUN" <<'PY'
from pathlib import Path
import os
import re
import sys
from datetime import datetime, timezone

mem_dir = Path(sys.argv[1])
retain = int(sys.argv[2])
max_age_days = int(sys.argv[3])
dry_run = bool(int(sys.argv[4]))

audit_prefix = mem_dir.as_posix()
pattern = re.compile(r"^(?P<base>.+?)_(?P<ts>\d{8}_\d{6})(?P<suffix>\.[^.]+)$")

cands = list(mem_dir.glob('*.jsonl')) + list(mem_dir.glob('*.md')) + list(mem_dir.glob('*.markdown'))
by_source = {}
for p in cands:
    if not p.is_file():
        continue
    stem = p.stem
    m = pattern.match(p.name)
    if m:
        base = m.group('base')
    else:
        base = stem
    by_source.setdefault(base, []).append(p)

# version pruning
removed = []
kept = []
for base, files in by_source.items():
    files = sorted(files, key=lambda x: x.stat().st_mtime, reverse=True)
    for f in files[retain:]:
        removed.append(f)
    kept.extend(files[:retain])

# age pruning
age_cutoff = None
if max_age_days > 0:
    age_cutoff = datetime.now().timestamp() - (max_age_days * 86400)
    kept_set = set(kept)
    for f in kept:
        if f.stat().st_mtime < age_cutoff:
            removed.append(f)

# dedupe
removed = sorted(set(removed), key=lambda p: p.stat().st_mtime, reverse=True)

print(f"[plan] candidates: {len(cands)}  keep:{sum(1 for _ in kept)} remove:{len(removed)}")
for p in removed[:200]:
    print(f"[remove] {p}")

if removed and not dry_run:
    for p in removed:
        try:
            p.unlink()
        except Exception as e:
            print(f"[warn] failed to delete {p}: {e}", file=sys.stderr)
elif not removed:
    print("[done] nothing to remove")
PY

after_bytes=$(du -sk "$MEMORY_DIR" | awk '{print $1}')

echo "[size] before: ${before_bytes} KB"
echo "[size] after:  ${after_bytes} KB"