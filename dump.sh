#!/usr/bin/env bash
set -euo pipefail

if [ $# -lt 1 ]; then
  echo "Usage: $0 <pid> [min_string_length]" >&2
  exit 2
fi

PID="$1"
MINLEN="${2:-4}"
MAPS="/proc/${PID}/maps"
MEM="/proc/${PID}/mem"

if [ ! -r "$MAPS" ]; then
  echo "Cannot read $MAPS — must be same UID or root" >&2
  exit 3
fi

OUTDIR="$(mktemp -d /tmp/dumpstrings.XXXX)"
echo "Dump dir: $OUTDIR"

awk '$0 ~ /r/ { split($1, a, "-"); printf("%s %s %s\n", a[1], a[2], $6) }' "$MAPS" | while read -r START HEXEND PATHNAME; do
  START_DEC=$((0x${START}))
  END_DEC=$((0x${HEXEND}))
  SIZE=$((END_DEC - START_DEC))
  if [ "$SIZE" -le 0 ]; then
    continue
  fi
  REGIONFILE="${OUTDIR}/region_${START}_${HEXEND}.bin"
  printf "Dumping %s-%s (%d bytes) %s\n" "$START" "$HEXEND" "$SIZE" "${PATHNAME:-}" >&2
  # dd with numeric skip/count (requires root for other-UID processes)
  dd if="$MEM" of="$REGIONFILE" bs=1 skip="$START_DEC" count="$SIZE" status=none 2>/dev/null || {
    echo "dd failed on region $START-$HEXEND (permission or sparse region). Skipping." >&2
    rm -f "$REGIONFILE"
    continue
  }
  strings -n "$MINLEN" "$REGIONFILE" > "${REGIONFILE}.strings" || true
  printf "Strings saved: %s\n" "${REGIONFILE}.strings" >&2
done

echo "All strings files in: $OUTDIR"

