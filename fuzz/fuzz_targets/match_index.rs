#![no_main]
//! Fuzz the full index + match pipeline: an attacker-controlled
//! `{spec, history, current, k}` object is indexed and matched. Neither an
//! adversarial spec nor a degenerate current window (empty, wrong length,
//! `k == 0`) may panic — each is a clean `Err`.

use libfuzzer_sys::fuzz_target;
use serde::Deserialize;
use shazam_core::{build_index, match_index, Candle, FingerprintSpec};

#[derive(Deserialize)]
struct Input {
    spec: FingerprintSpec,
    history: Vec<Candle>,
    current: Vec<Candle>,
    #[serde(default)]
    k: usize,
}

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    let Ok(input) = serde_json::from_str::<Input>(text) else {
        return;
    };
    if let Ok(index) = build_index(&input.history, &input.spec) {
        let _ = match_index(&index, &input.current, input.k);
    }
});
