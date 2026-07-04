//! The rolling fingerprint index over an asset's history.
//!
//! [`build_index`] folds a whole history one candle at a time (O(1) per bar),
//! emitting a fingerprint at every bar from warmup onward, then fits and applies
//! the per-axis normalization. The fold is serial by construction — each bar
//! depends on the previous one — so there is no parallelism here; the
//! parallelism lives in the match (see `search`). The index stores the fitted
//! [`AxisStats`] so a query can be normalized into the same space without the
//! full history.

use std::collections::BTreeMap;

use wickra_backtest_core::Candle;

pub use crate::normalize::AxisStats;

use crate::error::Result;
use crate::feature_set::FeatureSet;
use crate::fingerprint::{Fingerprint, RollBuffer};
use crate::normalize::{apply, fit_axes};
use crate::spec::{FingerprintSpec, Normalize};

/// A built index: the spec, the fitted per-axis stats, the rolling fingerprints
/// (ts ascending) and the timestamp labels.
pub struct FingerprintIndex {
    pub(crate) spec: FingerprintSpec,
    pub(crate) axes: Vec<AxisStats>,
    pub(crate) prints: Vec<Fingerprint>,
    pub(crate) labels: BTreeMap<i64, String>,
}

impl FingerprintIndex {
    /// The number of fingerprints in the index.
    #[must_use]
    pub fn indexed(&self) -> usize {
        self.prints.len()
    }

    /// Whether the index holds no fingerprints.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.prints.is_empty()
    }

    /// Attach (or overwrite) a human label at a historical timestamp.
    pub fn set_label(&mut self, ts: i64, label: String) {
        self.labels.insert(ts, label);
    }
}

/// Build a fingerprint index from a history and a spec.
pub fn build_index(history: &[Candle], spec: &FingerprintSpec) -> Result<FingerprintIndex> {
    spec.validate()?;
    let mut features = FeatureSet::new(spec)?;
    let mut roll = RollBuffer::new(spec.window);
    let mut prints: Vec<Fingerprint> = Vec::new();

    for candle in history {
        features.update(candle);
        if let Some(row) = features.row() {
            roll.push(row);
            if roll.is_full() {
                prints.push(roll.emit(candle.time));
            }
        }
    }

    let (axes, normalize) = if spec.normalize == Normalize::None {
        (Vec::new(), Normalize::None)
    } else {
        (fit_axes(&prints, spec.dim()), spec.normalize)
    };
    if normalize != Normalize::None {
        for print in &mut prints {
            apply(&mut print.v, &axes, normalize);
        }
    }

    Ok(FingerprintIndex {
        spec: spec.clone(),
        axes,
        prints,
        labels: BTreeMap::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::feature::{Feature, PriceField};
    use crate::spec::Metric;

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

    fn spec(window: usize, normalize: Normalize) -> FingerprintSpec {
        FingerprintSpec {
            features: vec![Feature::Price {
                field: PriceField::Close,
            }],
            window,
            normalize,
            metric: Metric::Euclid,
        }
    }

    #[test]
    fn indexes_price_history_window_one() {
        let history: Vec<Candle> = (0..5).map(|i| candle(i, i as f64)).collect();
        let index = build_index(&history, &spec(1, Normalize::None)).unwrap();
        assert_eq!(index.indexed(), 5);
        assert_eq!(index.prints[0].ts, 0);
        assert_eq!(&*index.prints[4].v, &[4.0]);
    }

    #[test]
    fn window_gates_first_fingerprint() {
        let history: Vec<Candle> = (0..5).map(|i| candle(i, i as f64)).collect();
        let index = build_index(&history, &spec(3, Normalize::None)).unwrap();
        // First full 3-bar window ends at bar index 2 (ts = 2).
        assert_eq!(index.indexed(), 3);
        assert_eq!(index.prints[0].ts, 2);
        // axis-major of closes [0,1,2] -> [0,1,2]
        assert_eq!(&*index.prints[0].v, &[0.0, 1.0, 2.0]);
    }

    #[test]
    fn zscore_normalizes_the_index() {
        let history: Vec<Candle> = (0..4).map(|i| candle(i, i as f64)).collect();
        let index = build_index(&history, &spec(1, Normalize::ZScore)).unwrap();
        assert_eq!(index.axes.len(), 1);
        // Normalized closes have zero mean; the middle values straddle it.
        assert!(index.prints[0].v[0] < 0.0);
        assert!(index.prints[3].v[0] > 0.0);
    }

    #[test]
    fn none_normalize_leaves_axes_empty() {
        let history: Vec<Candle> = (0..3).map(|i| candle(i, i as f64)).collect();
        let index = build_index(&history, &spec(1, Normalize::None)).unwrap();
        assert!(index.axes.is_empty());
    }

    #[test]
    fn labels_can_be_set() {
        let history: Vec<Candle> = (0..3).map(|i| candle(i, i as f64)).collect();
        let mut index = build_index(&history, &spec(1, Normalize::None)).unwrap();
        index.set_label(1, "may_2021_crash".to_string());
        assert_eq!(
            index.labels.get(&1).map(String::as_str),
            Some("may_2021_crash")
        );
    }
}
