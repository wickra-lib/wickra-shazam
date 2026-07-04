#![no_main]
//! Fuzz index construction: an attacker-controlled `{spec, history}` object is
//! parsed and indexed. `build_index` validates the spec first (`dim <= MAX_DIM`),
//! so it returns a clean `Err` rather than panicking or over-allocating on an
//! adversarial spec.

use libfuzzer_sys::fuzz_target;
use serde::Deserialize;
use shazam_core::{build_index, Candle, FingerprintSpec};

#[derive(Deserialize)]
struct Input {
    spec: FingerprintSpec,
    history: Vec<Candle>,
}

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    let Ok(input) = serde_json::from_str::<Input>(text) else {
        return;
    };
    let _ = build_index(&input.history, &input.spec);
});
