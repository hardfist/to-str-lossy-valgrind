#!/usr/bin/env bash
set -euo pipefail

iters="${1:-100}"
bin="target/release/to-str-lossy-valgrind"
run_id="$(date -u +%Y%m%dT%H%M%SZ)"
out_dir="optimization-artifacts/valgrind/callgrind/${run_id}"

cargo build --release
mkdir -p "${out_dir}"

{
  echo "command: scripts/run_callgrind.sh ${iters}"
  echo "benchmark binary: ${bin}"
  echo "iterations: ${iters}"
  echo "rustc: $(rustc --version)"
  echo "cargo: $(cargo --version)"
  echo "valgrind: $(valgrind --version)"
  echo "uname: $(uname -a)"
} > "${out_dir}/run-info.txt"

for case_name in to-str-valid lossy-valid to-str-invalid lossy-invalid; do
  out_file="${out_dir}/${case_name}.callgrind.out"
  stdout_file="${out_dir}/${case_name}.stdout.txt"
  annotate_file="${out_dir}/${case_name}.annotate.txt"

  valgrind \
    --tool=callgrind \
    --callgrind-out-file="${out_file}" \
    "${bin}" "${case_name}" "${iters}" > "${stdout_file}"

  callgrind_annotate \
    --show=Ir \
    --sort=Ir \
    --threshold=99 \
    "${out_file}" > "${annotate_file}"
done

scripts/summarize_ir.sh "${out_dir}" | tee "${out_dir}/summary.tsv"
echo "wrote ${out_dir}"
