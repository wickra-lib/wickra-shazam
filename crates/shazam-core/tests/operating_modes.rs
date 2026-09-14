//! Operating-mode equivalence over the golden corpus.
//!
//! The command protocol offers two ways to attach a label to a historical
//! window, and both must yield the same `match` report bytes: a `label` sent
//! before `index` is kept on the handle and applied when the index is built; a
//! `label` sent after `index` goes onto the live index. Re-indexing the same
//! history keeps the labels and the report. The unlabelled report is the
//! blessed golden; the labelled one differs from it only by carrying the
//! label on the top match.
//!
//! The bindings repeat this check at their own boundary; this is the in-core
//! anchor.

use std::fs;
use std::path::{Path, PathBuf};

use wickra_shazam_core::Shazam;

const K: usize = 5;
const LABEL: &str = "golden_top";

fn golden_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../golden")
}

/// The candle array as JSON text built from the raw CSV tokens, so no number
/// formatting of this test's own can drift from what the bindings send.
fn candles_json(path: &Path) -> String {
    let text = fs::read_to_string(path).unwrap();
    let rows: Vec<String> = text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let cols: Vec<&str> = line.split(',').collect();
            cols[0].parse::<i64>().ok().map(|_| {
                format!(
                    r#"{{"time":{},"open":{},"high":{},"low":{},"close":{},"volume":{}}}"#,
                    cols[0], cols[1], cols[2], cols[3], cols[4], cols[5]
                )
            })
        })
        .collect();
    format!("[{}]", rows.join(","))
}

/// The `ts` of the first match in a report.
fn top_ts(report: &str) -> i64 {
    let at = report.find("\"ts\":").expect("a match") + 5;
    report[at..]
        .chars()
        .take_while(char::is_ascii_digit)
        .collect::<String>()
        .parse()
        .unwrap()
}

#[test]
fn a_label_before_index_matches_a_label_after_index() {
    let dir = golden_dir();
    let history = candles_json(&dir.join("data/history/sym-01.csv"));
    let current = candles_json(&dir.join("data/current/sym-01.csv"));
    let index_cmd = format!(r#"{{"cmd":"index","history":{history}}}"#);
    let match_cmd = format!(r#"{{"cmd":"match","current":{current},"k":{K}}}"#);
    let mut specs: Vec<PathBuf> = fs::read_dir(dir.join("specs"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    specs.sort();
    assert!(!specs.is_empty(), "no golden specs found");

    for spec_path in specs {
        let name = spec_path.file_name().unwrap().to_str().unwrap().to_string();
        let spec = fs::read_to_string(&spec_path).unwrap();
        let expected = fs::read_to_string(dir.join("expected").join(&name)).unwrap();

        // The blessed path: index, then match.
        let mut plain = Shazam::new(&spec).unwrap();
        plain.command_json(&index_cmd).unwrap();
        let unlabelled = plain.command_json(&match_cmd).unwrap();
        assert_eq!(unlabelled.trim(), expected.trim(), "{name}");
        let ts = top_ts(&unlabelled);
        let label_cmd = format!(r#"{{"cmd":"label","ts":{ts},"label":"{LABEL}"}}"#);

        // Label first, then index: the handle keeps it for the index it builds.
        let mut before = Shazam::new(&spec).unwrap();
        before.command_json(&label_cmd).unwrap();
        before.command_json(&index_cmd).unwrap();
        let labelled_before = before.command_json(&match_cmd).unwrap();

        // Index first, then label: it goes onto the live index.
        let mut after = Shazam::new(&spec).unwrap();
        after.command_json(&index_cmd).unwrap();
        after.command_json(&label_cmd).unwrap();
        let labelled_after = after.command_json(&match_cmd).unwrap();
        assert_eq!(labelled_before, labelled_after, "{name}");
        assert!(labelled_after.contains(LABEL), "{name}");
        assert_ne!(labelled_after, unlabelled, "{name}");

        // Re-indexing the same history keeps the label and the report.
        after.command_json(&index_cmd).unwrap();
        assert_eq!(
            after.command_json(&match_cmd).unwrap(),
            labelled_after,
            "{name}"
        );
    }
}
