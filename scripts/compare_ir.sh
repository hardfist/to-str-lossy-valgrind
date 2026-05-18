#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "usage: scripts/compare_ir.sh <summary.tsv>" >&2
  exit 1
fi

summary_file="$1"

read_ir() {
  local case_name="$1"
  awk -v case_name="${case_name}" 'BEGIN { FS="\t" } $1 == case_name { print $2; found=1 } END { if (!found) exit 1 }' "${summary_file}"
}

to_str_valid="$(read_ir to-str-valid)"
lossy_valid="$(read_ir lossy-valid)"
to_str_invalid="$(read_ir to-str-invalid)"
lossy_invalid="$(read_ir lossy-invalid)"

ratio() {
  awk -v numerator="$1" -v denominator="$2" 'BEGIN { printf "%.2f", numerator / denominator }'
}

valid_ratio="$(ratio "${lossy_valid}" "${to_str_valid}")"
invalid_ratio="$(ratio "${lossy_invalid}" "${to_str_invalid}")"

cat <<EOF
| Group | to_str Ir | to_string_lossy Ir | lossy / to_str |
| --- | ---: | ---: | ---: |
| valid UTF-8 | ${to_str_valid} | ${lossy_valid} | ${valid_ratio}x |
| invalid UTF-8 | ${to_str_invalid} | ${lossy_invalid} | ${invalid_ratio}x |
EOF
