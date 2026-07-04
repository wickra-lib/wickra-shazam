//! Parallel == sequential: `match_index` is byte-stable across repeated runs and
//! equal to the blessed golden reference.
//!
//! The `parallel` feature is compile-time, so one test binary exercises exactly
//! one path. Determinism across repeated runs rules out rayon reduction-order
//! drift within the parallel build; asserting the result equals the blessed
//! reference under **both** the parallel (default) build and the sequential
//! (`--no-default-features`, the WASM path) build is what pins the two paths to
//! bit-for-bit agreement. CI runs this test under each feature set.

use shazam_core::{build_index, match_index, Candle, FingerprintSpec};
use std::fs;
use std::path::{Path, PathBuf};

const K: usize = 5;

fn golden_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../golden")
}

fn load_csv(path: &Path) -> Vec<Candle> {
    let text = fs::read_to_string(path).unwrap();
    let mut candles = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split(',').collect();
        if cols[0].parse::<i64>().is_err() {
            continue; // header row
        }
        let json = format!(
            r#"{{"time":{},"open":{},"high":{},"low":{},"close":{},"volume":{}}}"#,
            cols[0], cols[1], cols[2], cols[3], cols[4], cols[5]
        );
        candles.push(serde_json::from_str(&json).unwrap());
    }
    candles
}

#[test]
fn match_is_deterministic_and_equals_golden() {
    let dir = golden_dir();
    let history = load_csv(&dir.join("data/history/sym-01.csv"));
    let current = load_csv(&dir.join("data/current/sym-01.csv"));

    // Cover the cosine reduction and the DTW 2D path — the two shapes most
    // likely to expose reduction-order sensitivity.
    for name in ["crash_setup.json", "microstructure_dtw.json"] {
        let spec =
            FingerprintSpec::from_json(&fs::read_to_string(dir.join("specs").join(name)).unwrap())
                .unwrap();
        let index = build_index(&history, &spec).unwrap();

        let first = serde_json::to_string(&match_index(&index, &current, K).unwrap()).unwrap();
        for _ in 0..8 {
            let again = serde_json::to_string(&match_index(&index, &current, K).unwrap()).unwrap();
            assert_eq!(again, first, "non-deterministic match for {name}");
        }

        let expected = fs::read_to_string(dir.join("expected").join(name)).unwrap();
        assert_eq!(first.trim(), expected.trim(), "golden mismatch for {name}");
    }
}
