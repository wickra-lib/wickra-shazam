#![no_main]
//! Fuzz spec parsing: arbitrary bytes are fed to `FingerprintSpec::from_json`.
//! No input — however malformed — may panic; anything unparsable or structurally
//! invalid (empty features, `dim > MAX_DIM`) is a clean `Err`.

use libfuzzer_sys::fuzz_target;
use shazam_core::FingerprintSpec;

fuzz_target!(|data: &[u8]| {
    if let Ok(text) = std::str::from_utf8(data) {
        let _ = FingerprintSpec::from_json(text);
    }
});
