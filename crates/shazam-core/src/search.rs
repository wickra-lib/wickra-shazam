//! Matching the current fingerprint against a built index.
//!
//! [`match_index`] computes the current fingerprint, normalizes it into the
//! index's space, scores it against every historical fingerprint (in parallel
//! under the `parallel` feature, sequentially otherwise), then sorts by a total
//! order and returns the top `k`. Because the scores are rounded and the sort is
//! a stable total order, the parallel and sequential paths return a
//! byte-identical report.

use serde::{Deserialize, Serialize};

#[cfg(feature = "parallel")]
use rayon::prelude::*;

use wickra_backtest_core::Candle;

use crate::error::{Error, Result};
use crate::feature_set::FeatureSet;
use crate::fingerprint::{Fingerprint, RollBuffer};
use crate::index::FingerprintIndex;
use crate::metric::{dtw_similarity, similarity};
use crate::normalize::apply;
use crate::spec::Metric;

/// One historical match: a timestamp, its similarity to the current fingerprint,
/// and an optional human label.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct HistoricalMatch {
    /// Timestamp of the matched historical fingerprint.
    pub ts: i64,
    /// Similarity in `[0, 1]` (1 = identical).
    pub similarity: f64,
    /// A human label attached to this timestamp, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// The result of a match: the top matches and how many fingerprints were searched.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MatchReport {
    /// The top-`k` matches, most similar first.
    pub matches: Vec<HistoricalMatch>,
    /// The number of fingerprints in the searched index.
    pub indexed: usize,
}

/// Round a similarity to a fixed `1e-8` precision so the output is byte-stable.
fn round_to(x: f64) -> f64 {
    (x * 1e8).round() / 1e8
}

/// Reshape an axis-major flat fingerprint into a `[window][features]` sequence.
fn reshape(flat: &[f64], window: usize) -> Vec<Vec<f64>> {
    let features = flat.len() / window;
    (0..window)
        .map(|t| (0..features).map(|f| flat[f * window + t]).collect())
        .collect()
}

/// Match the current fingerprint (built from `current`) against `index`, returning
/// the `k` most similar historical fingerprints.
pub fn match_index(index: &FingerprintIndex, current: &[Candle], k: usize) -> Result<MatchReport> {
    if k == 0 {
        return Err(Error::BadSpec("k must be > 0".into()));
    }

    let query = current_fingerprint(index, current)?;
    let metric = index.spec.metric;
    let window = index.spec.window;
    let query_seq = if metric == Metric::Dtw {
        reshape(&query.v, window)
    } else {
        Vec::new()
    };

    let compute = |print: &Fingerprint| -> (i64, f64) {
        let sim = if metric == Metric::Dtw {
            let print_seq = reshape(&print.v, window);
            let q: Vec<&[f64]> = query_seq.iter().map(Vec::as_slice).collect();
            let p: Vec<&[f64]> = print_seq.iter().map(Vec::as_slice).collect();
            dtw_similarity(&q, &p, window)
        } else {
            similarity(&query.v, &print.v, metric)
        };
        (print.ts, round_to(sim))
    };

    #[cfg(feature = "parallel")]
    let mut scored: Vec<(i64, f64)> = index.prints.par_iter().map(compute).collect();
    #[cfg(not(feature = "parallel"))]
    let mut scored: Vec<(i64, f64)> = index.prints.iter().map(compute).collect();

    // Stable total order: similarity descending, ties broken by ts ascending.
    scored.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
    scored.truncate(k);

    let matches = scored
        .into_iter()
        .map(|(ts, similarity)| HistoricalMatch {
            ts,
            similarity,
            label: index.labels.get(&ts).cloned(),
        })
        .collect();

    Ok(MatchReport {
        matches,
        indexed: index.prints.len(),
    })
}

/// Build the current fingerprint from `current` using the index's spec, normalized
/// into the index's space. Errors if there are too few bars for a full window.
fn current_fingerprint(index: &FingerprintIndex, current: &[Candle]) -> Result<Fingerprint> {
    let mut features = FeatureSet::new(&index.spec)?;
    let mut roll = RollBuffer::new(index.spec.window);
    for candle in current {
        features.update(candle);
        if let Some(row) = features.row() {
            roll.push(row);
        }
    }
    if !roll.is_full() {
        return Err(Error::Data(
            "not enough current bars to form a fingerprint".into(),
        ));
    }
    let ts = current.last().map_or(0, |c| c.time);
    let mut fingerprint = roll.emit(ts);
    apply(&mut fingerprint.v, &index.axes, index.spec.normalize);
    Ok(fingerprint)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::feature::{Feature, PriceField};
    use crate::index::build_index;
    use crate::spec::{FingerprintSpec, Normalize};

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

    fn euclid_spec() -> FingerprintSpec {
        FingerprintSpec {
            features: vec![Feature::Price {
                field: PriceField::Close,
            }],
            window: 1,
            normalize: Normalize::None,
            metric: Metric::Euclid,
        }
    }

    fn index() -> FingerprintIndex {
        let history: Vec<Candle> = (0..5).map(|i| candle(i, i as f64)).collect();
        build_index(&history, &euclid_spec()).unwrap()
    }

    #[test]
    fn zero_k_is_rejected() {
        assert!(matches!(
            match_index(&index(), &[candle(9, 2.0)], 0),
            Err(Error::BadSpec(_))
        ));
    }

    #[test]
    fn too_few_current_bars_errors() {
        let mut spec = euclid_spec();
        spec.window = 3;
        let history: Vec<Candle> = (0..5).map(|i| candle(i, i as f64)).collect();
        let idx = build_index(&history, &spec).unwrap();
        assert!(matches!(
            match_index(&idx, &[candle(9, 2.0)], 3),
            Err(Error::Data(_))
        ));
    }

    #[test]
    fn finds_the_closest_and_sorts_descending() {
        let report = match_index(&index(), &[candle(99, 2.0)], 3).unwrap();
        assert_eq!(report.indexed, 5);
        assert_eq!(report.matches.len(), 3);
        // close 2.0 exactly matches the fingerprint at ts=2.
        assert_eq!(report.matches[0].ts, 2);
        assert!((report.matches[0].similarity - 1.0).abs() < 1e-9);
        // ties (ts 1 and 3, both distance 1) break by ts ascending.
        assert_eq!(report.matches[1].ts, 1);
        assert!(report.matches[0].similarity >= report.matches[1].similarity);
    }

    #[test]
    fn labels_ride_along() {
        let mut idx = index();
        idx.set_label(2, "may_2021_crash".to_string());
        let report = match_index(&idx, &[candle(99, 2.0)], 1).unwrap();
        assert_eq!(report.matches[0].label.as_deref(), Some("may_2021_crash"));
    }
}
