//! Data-driven core of Wickra Shazam — "Shazam for markets".
//!
//! A serde [`FingerprintSpec`] is rolled over an asset's whole history with the
//! [Wickra](https://github.com/wickra-lib/wickra) feature space (514 O(1)
//! streaming indicators plus price and microstructure) into a
//! [`FingerprintIndex`] of fixed-dimension fingerprints, and the current
//! fingerprint is matched against that index to name the regime. Indexing is
//! serial; matching runs in parallel (rayon) or sequentially (the WASM fallback)
//! and returns a byte-identical [`MatchReport`].
//!
//! Two free functions and one handle share the core: [`build_index`] builds the
//! index and [`match_index`] queries it, while [`Shazam`] wraps both behind a
//! single JSON-in / JSON-out boundary, [`Shazam::command_json`], that every
//! language binding drives.
//!
//! Features are resolved by name from the `wickra-core` registry (via the
//! backtester's factory), and candles use [`Candle`] re-exported below.

mod config;
mod error;
mod feature;
mod feature_set;
mod fingerprint;
mod index;
mod metric;
mod normalize;
mod search;
mod shazam;
mod spec;

pub use config::Config;
pub use error::{Error, Result};
pub use feature::{Feature, PriceField};
pub use fingerprint::Fingerprint;
pub use index::{build_index, FingerprintIndex};
pub use search::{match_index, HistoricalMatch, MatchReport};
pub use shazam::Shazam;
pub use spec::{FingerprintSpec, Metric, Normalize};

// The candle type consumers index and match with (the backtester's OHLCV bar,
// which the registry features are driven by).
pub use wickra_backtest_core::Candle;

/// The shazam-core version string.
#[must_use]
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
