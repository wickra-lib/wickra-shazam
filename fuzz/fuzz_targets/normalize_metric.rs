#![no_main]
//! Fuzz the normalize + metric paths: an arbitrary (validated) spec — driving
//! any `Normalize` × `Metric` combination — is indexed and matched over a fixed,
//! bounded history. Degenerate axes (zero variance, zero norm) must be clamped,
//! never producing `NaN`/`inf` or a panic.

use libfuzzer_sys::fuzz_target;
use shazam_core::{build_index, match_index, Candle, FingerprintSpec};

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    // `from_json` validates, so `dim <= MAX_DIM` and the work is bounded.
    let Ok(spec) = FingerprintSpec::from_json(text) else {
        return;
    };
    let history: Vec<Candle> = (0i64..48)
        .map(|i| {
            let c = 100.0 + i as f64;
            Candle {
                time: i,
                open: c,
                high: c + 1.0,
                low: c - 1.0,
                close: c,
                volume: 1000.0,
            }
        })
        .collect();
    if let Ok(index) = build_index(&history, &spec) {
        let _ = match_index(&index, &history, 5);
    }
});
