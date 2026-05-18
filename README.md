# to-str-lossy-valgrind

Small Rust repo for comparing `Path::to_str()` and `Path::to_string_lossy()` with Valgrind Callgrind.

The benchmark has four isolated cases:

| Case | Input | API |
| --- | --- | --- |
| `to-str-valid` | valid UTF-8 paths | `Path::to_str()` |
| `lossy-valid` | valid UTF-8 paths | `Path::to_string_lossy()` |
| `to-str-invalid` | invalid UTF-8 paths on Unix | `Path::to_str()` |
| `lossy-invalid` | invalid UTF-8 paths on Unix | `Path::to_string_lossy()` |

`to_str()` returns `Option<&str>` and never allocates. `to_string_lossy()` returns `Cow<str>`: it is borrowed for valid UTF-8, but allocates a repaired `String` for invalid UTF-8.

## Run

```bash
scripts/run_callgrind.sh
```

The script writes raw Callgrind files, annotations, stdout, run metadata, and a compact `summary.tsv` under:

```text
optimization-artifacts/valgrind/callgrind/<timestamp>/
```

Pass a larger iteration count when you want a longer run:

```bash
scripts/run_callgrind.sh 1000
```

## Local Sample

On this machine with `rustc 1.91.1` and `valgrind-3.26.0.codspeed`, `scripts/run_callgrind.sh 100` produced:

| Case | Ir | Ratio |
| --- | ---: | ---: |
| `to-str-valid` | 45,713,427 | 1.00x |
| `lossy-valid` | 146,884,831 | 3.21x vs `to-str-valid` |
| `to-str-invalid` | 66,429,736 | 1.00x |
| `lossy-invalid` | 440,190,883 | 6.63x vs `to-str-invalid` |

Interpretation: for valid UTF-8, `to_string_lossy()` is still more expensive because it has to build and return a `Cow<str>` path through the lossy conversion machinery. For invalid UTF-8, the gap is much larger because `to_string_lossy()` allocates and creates a repaired string with replacement characters, while `to_str()` only reports `None`.

## Manual Cases

```bash
cargo build --release
valgrind --tool=callgrind ./target/release/to-str-lossy-valgrind to-str-valid 1000
valgrind --tool=callgrind ./target/release/to-str-lossy-valgrind lossy-valid 1000
valgrind --tool=callgrind ./target/release/to-str-lossy-valgrind to-str-invalid 1000
valgrind --tool=callgrind ./target/release/to-str-lossy-valgrind lossy-invalid 1000
```

Use Callgrind `Ir` as the primary metric. Lower is better.

## CI

GitHub Actions runs the same Callgrind comparison on pushes, pull requests, and manual dispatches:

```text
.github/workflows/valgrind.yml
```

The workflow installs Valgrind, runs `scripts/run_callgrind.sh`, writes a Markdown comparison table to the job summary, and uploads the raw Callgrind outputs as an artifact.
