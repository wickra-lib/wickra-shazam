//! Property tests: over random price-only histories and specs, `match_index`
//! never panics and its report obeys the structural invariants — bounded match
//! count, similarities in `[0, 1]`, a monotone-descending sort — and an
//! identical current-vs-history window yields a perfect self-match.

use proptest::prelude::*;
use shazam_core::{build_index, match_index, Candle, FingerprintSpec};

const FIELDS: [&str; 5] = ["open", "high", "low", "close", "volume"];
const METRICS: [&str; 3] = ["cosine", "euclid", "dtw"];
const NORMS: [&str; 3] = ["none", "z_score", "min_max"];

/// Build a valid OHLCV candle (high/low bracket open/close, all finite, volume
/// non-negative) through serde, matching how every binding parses candles.
fn candle(time: i64, open: f64, close: f64, spread: f64, volume: f64) -> Candle {
    let high = open.max(close) + spread;
    let low = open.min(close) - spread;
    serde_json::from_str(&format!(
        r#"{{"time":{time},"open":{open},"high":{high},"low":{low},"close":{close},"volume":{volume}}}"#
    ))
    .unwrap()
}

/// Assemble a price-only spec JSON from the sampled parameters.
fn spec_json(fields: &[usize], window: usize, metric: usize, norm: usize) -> String {
    let features: Vec<String> = fields
        .iter()
        .map(|&f| format!(r#"{{"kind":"price","field":"{}"}}"#, FIELDS[f]))
        .collect();
    format!(
        r#"{{"features":[{}],"window":{},"metric":"{}","normalize":"{}"}}"#,
        features.join(","),
        window,
        METRICS[metric],
        NORMS[norm]
    )
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(96))]

    #[test]
    fn match_report_invariants(
        bars in prop::collection::vec((10.0f64..500.0, 10.0f64..500.0, 0.0f64..5.0, 0.0f64..10_000.0), 20..60),
        fields in prop::collection::vec(0usize..5, 1..5),
        window in 1usize..7,
        metric in 0usize..3,
        norm in 0usize..3,
        k in 1usize..11,
    ) {
        let history: Vec<Candle> = bars
            .iter()
            .enumerate()
            .map(|(i, &(o, c, s, v))| {
                candle(1_700_000_000 + i64::try_from(i).unwrap() * 3600, o, c, s, v)
            })
            .collect();
        let spec = FingerprintSpec::from_json(&spec_json(&fields, window, metric, norm)).unwrap();

        let index = build_index(&history, &spec).unwrap();
        // Price features have no warmup, so the index is non-empty and the whole
        // history is a valid current window.
        let report = match_index(&index, &history, k).unwrap();

        prop_assert!(report.matches.len() <= k.min(report.indexed));
        for m in &report.matches {
            prop_assert!(m.similarity >= -1e-9 && m.similarity <= 1.0 + 1e-9, "similarity {} out of range", m.similarity);
        }
        for pair in report.matches.windows(2) {
            prop_assert!(pair[0].similarity >= pair[1].similarity, "matches not sorted descending");
        }
    }
}

#[test]
fn identical_window_is_a_perfect_self_match() {
    // A euclid spec with no normalization: the most recent fingerprint of the
    // history matched against the history itself is identical to a stored one,
    // so its distance is zero and its similarity is exactly 1.0.
    let history: Vec<Candle> = (0..30)
        .map(|i| {
            candle(
                1_700_000_000 + i * 3600,
                100.0 + i as f64,
                101.0 + i as f64,
                0.5,
                1000.0,
            )
        })
        .collect();
    let spec = FingerprintSpec::from_json(
        r#"{"features":[{"kind":"price","field":"close"},{"kind":"price","field":"high"}],"window":4,"metric":"euclid","normalize":"none"}"#,
    )
    .unwrap();
    let index = build_index(&history, &spec).unwrap();
    let report = match_index(&index, &history, 1).unwrap();
    assert_eq!(report.matches.len(), 1);
    assert!((report.matches[0].similarity - 1.0).abs() < 1e-9);
}
