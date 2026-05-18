use std::borrow::Cow;
use std::env;
use std::hint::black_box;
use std::path::PathBuf;
use std::process::ExitCode;

const DEFAULT_ITERATIONS: usize = 1_000;
const INPUT_COUNT: usize = 4_096;

#[derive(Clone, Copy)]
enum Case {
    ToStrValid,
    LossyValid,
    ToStrInvalid,
    LossyInvalid,
}

impl Case {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "to-str-valid" => Some(Self::ToStrValid),
            "lossy-valid" => Some(Self::LossyValid),
            "to-str-invalid" => Some(Self::ToStrInvalid),
            "lossy-invalid" => Some(Self::LossyInvalid),
            _ => None,
        }
    }

    fn uses_valid_input(self) -> bool {
        matches!(self, Self::ToStrValid | Self::LossyValid)
    }
}

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let Some(case_name) = args.next() else {
        print_usage();
        return ExitCode::FAILURE;
    };

    let Some(case) = Case::parse(&case_name) else {
        eprintln!("unknown case: {case_name}");
        print_usage();
        return ExitCode::FAILURE;
    };

    let iterations = match args.next() {
        Some(raw) => match raw.parse::<usize>() {
            Ok(value) if value > 0 => value,
            _ => {
                eprintln!("iterations must be a positive integer");
                return ExitCode::FAILURE;
            }
        },
        None => DEFAULT_ITERATIONS,
    };

    let paths = make_inputs(case.uses_valid_input());
    let checksum = match case {
        Case::ToStrValid | Case::ToStrInvalid => bench_to_str(&paths, iterations),
        Case::LossyValid | Case::LossyInvalid => bench_to_string_lossy(&paths, iterations),
    };

    println!(
        "case={case_name} iterations={iterations} inputs={} checksum={checksum}",
        paths.len()
    );
    ExitCode::SUCCESS
}

fn print_usage() {
    eprintln!(
        "usage: to-str-lossy-valgrind <case> [iterations]\n\
         cases: to-str-valid | lossy-valid | to-str-invalid | lossy-invalid"
    );
}

fn make_inputs(valid_utf8: bool) -> Vec<PathBuf> {
    (0..INPUT_COUNT)
        .map(|idx| {
            if valid_utf8 {
                PathBuf::from(format!("/tmp/rust-path-bench/{idx:04}/file-{idx:04}.txt"))
            } else {
                invalid_utf8_path(idx)
            }
        })
        .collect()
}

#[cfg(unix)]
fn invalid_utf8_path(idx: usize) -> PathBuf {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;

    let mut bytes = format!("/tmp/rust-path-bench/{idx:04}/file-").into_bytes();
    bytes.extend_from_slice(&[0xff, b'-', (idx % 251) as u8, b'.', b't', b'x', b't']);
    PathBuf::from(OsString::from_vec(bytes))
}

#[cfg(not(unix))]
fn invalid_utf8_path(idx: usize) -> PathBuf {
    PathBuf::from(format!("/tmp/rust-path-bench/{idx:04}/file-invalid.txt"))
}

fn bench_to_str(paths: &[PathBuf], iterations: usize) -> usize {
    let mut checksum = 0usize;

    for _ in 0..iterations {
        for path in paths {
            let path = black_box(path);
            match path.to_str() {
                Some(value) => checksum = checksum.wrapping_add(value.len()),
                None => checksum = checksum.wrapping_add(1),
            }
        }
    }

    black_box(checksum)
}

fn bench_to_string_lossy(paths: &[PathBuf], iterations: usize) -> usize {
    let mut checksum = 0usize;

    for _ in 0..iterations {
        for path in paths {
            let path = black_box(path);
            match path.to_string_lossy() {
                Cow::Borrowed(value) => checksum = checksum.wrapping_add(value.len()),
                Cow::Owned(value) => checksum = checksum.wrapping_add(value.len()),
            }
        }
    }

    black_box(checksum)
}
