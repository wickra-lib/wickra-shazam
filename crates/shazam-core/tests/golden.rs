//! Golden: replay the canonical harness (index `sym-01`'s history, match its
//! current window with `k = 5`) for every committed spec and assert the
//! `MatchReport` JSON is byte-for-byte equal to `golden/expected/<spec>.json`.
//!
//! The fixtures are blessed via the CLI (see `golden/README.md`); this test is
//! the Rust half of the cross-language golden contract.

use shazam_core::{build_index, match_index, Candle, FingerprintSpec};
use std::fs;
use std::path::{Path, PathBuf};

const K: usize = 5;

fn golden_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../golden")
}

/// Parse an OHLCV CSV (`time,open,high,low,close,volume`, one header row) into
/// candles, going through serde so the parse matches every other binding.
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
fn golden_reports_match_byte_for_byte() {
    let dir = golden_dir();
    let history = load_csv(&dir.join("data/history/sym-01.csv"));
    let current = load_csv(&dir.join("data/current/sym-01.csv"));

    let mut specs: Vec<PathBuf> = fs::read_dir(dir.join("specs"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    specs.sort();
    assert!(!specs.is_empty(), "no golden specs found");

    for spec_path in specs {
        let name = spec_path.file_name().unwrap().to_str().unwrap().to_string();
        let spec = FingerprintSpec::from_json(&fs::read_to_string(&spec_path).unwrap()).unwrap();
        let index = build_index(&history, &spec).unwrap();
        let report = match_index(&index, &current, K).unwrap();
        let got = serde_json::to_string(&report).unwrap();
        let expected = fs::read_to_string(dir.join("expected").join(&name)).unwrap();
        assert_eq!(got.trim(), expected.trim(), "golden mismatch for {name}");
    }
}
