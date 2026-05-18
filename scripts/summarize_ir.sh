#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "usage: scripts/summarize_ir.sh <callgrind-run-dir>" >&2
  exit 1
fi

run_dir="$1"

printf "case\tIr\n"
for case_name in to-str-valid lossy-valid to-str-invalid lossy-invalid; do
  file="${run_dir}/${case_name}.annotate.txt"
  ir="$(
    awk '/PROGRAM TOTALS/ { gsub(",", "", $1); print $1; found=1 } END { if (!found) exit 1 }' "${file}"
  )"
  printf "%s\t%s\n" "${case_name}" "${ir}"
done
