//! A runnable Rust example: index a short history with the native `build_index`
//! API and match the current state.
//!
//! ```bash
//! cargo run -p wickra-shazam-example
//! ```

use shazam_core::{build_index, match_index, Candle, FingerprintSpec};

const SPEC: &str = r#"{
    "features": [{"kind": "price", "field": "close"}],
    "window": 1,
    "metric": "euclid"
}"#;

fn candle(time: i64, close: f64) -> Candle {
    Candle {
        time,
        open: close,
        high: close,
        low: close,
        close,
        volume: 1.0,
    }
}

fn main() {
    let spec: FingerprintSpec = FingerprintSpec::from_json(SPEC).expect("valid spec");

    let history = vec![candle(1, 100.0), candle(2, 101.0), candle(3, 102.0)];
    let index = build_index(&history, &spec).expect("index");

    let current = vec![candle(4, 102.0)];
    let report = match_index(&index, &current, 2).expect("match");

    println!("wickra-shazam {}", shazam_core::version());
    println!(
        "{}",
        serde_json::to_string(&report).expect("serialize report")
    );
    for m in &report.matches {
        println!("  match ts {} similarity {}", m.ts, m.similarity);
    }
}
