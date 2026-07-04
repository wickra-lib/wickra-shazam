//! Resolves the features a spec references and folds candles through them.
//!
//! Indicator and microstructure features are resolved by name and parameters
//! from the `wickra-core` registry — reused through the `wickra-backtest-core`
//! factory, the only name -> indicator resolver in the ecosystem. Price features
//! read straight from the candle. Each `update` ticks every resolved indicator
//! once (O(1)); [`FeatureSet::row`] then reads the current values back in
//! **spec feature order** — the fixed axis order that makes a fingerprint
//! deterministic.

use std::collections::BTreeMap;

use wickra_backtest_core::registry::{build, BarInput};
use wickra_backtest_core::{Candle, EvalIndicator};

use crate::error::{Error, Result};
use crate::feature::{Feature, PriceField};
use crate::spec::FingerprintSpec;

/// One resolved indicator plus its canonical base key (`<name>(<p,p>)`).
struct Entry {
    key: String,
    indicator: Box<dyn EvalIndicator>,
}

/// How one fingerprint axis reads its value each bar.
enum Axis {
    /// A raw price field from the candle.
    Price(PriceField),
    /// A value looked up in the current indicator map by its full key
    /// (`<base>` or `<base>.<field>`).
    Keyed(String),
}

/// The ordered set of features a spec needs, folded one candle at a time.
pub(crate) struct FeatureSet {
    indicators: Vec<Entry>,
    axes: Vec<Axis>,
    cur: BTreeMap<String, f64>,
    last: Option<Candle>,
}

impl FeatureSet {
    /// Resolve every feature a spec references. Indicator and microstructure
    /// features resolve through the registry (deduplicated by base key); price
    /// features need none. Errors if the registry does not know a feature or
    /// rejects its parameters.
    pub(crate) fn new(spec: &FingerprintSpec) -> Result<Self> {
        let mut indicators: Vec<Entry> = Vec::new();
        let mut axes: Vec<Axis> = Vec::with_capacity(spec.features.len());
        for feature in &spec.features {
            match feature {
                Feature::Price { field } => axes.push(Axis::Price(*field)),
                Feature::Indicator {
                    name,
                    params,
                    field,
                }
                | Feature::Microstructure {
                    name,
                    params,
                    field,
                } => {
                    let base = base_key(name, params);
                    if indicators.iter().all(|e| e.key != base) {
                        let indicator = build(name, params)
                            .map_err(|e| Error::UnknownFeature(format!("{name}: {e}")))?;
                        indicators.push(Entry {
                            key: base.clone(),
                            indicator,
                        });
                    }
                    let lookup = match field {
                        Some(field) => format!("{base}.{field}"),
                        None => base,
                    };
                    axes.push(Axis::Keyed(lookup));
                }
            }
        }
        Ok(Self {
            indicators,
            axes,
            cur: BTreeMap::new(),
            last: None,
        })
    }

    /// Fold one candle: every indicator ticks and records its primary value and
    /// named fields; the candle is kept for price passthrough.
    pub(crate) fn update(&mut self, candle: &Candle) {
        self.cur.clear();
        let bar = BarInput {
            candle,
            reference: None,
            deriv: None,
            orderbook: None,
            trades: &[],
            cross_section: None,
        };
        for entry in &mut self.indicators {
            if let Some(value) = entry.indicator.update(&bar) {
                self.cur.insert(entry.key.clone(), value);
                for (field, field_value) in entry.indicator.fields() {
                    self.cur
                        .insert(format!("{}.{field}", entry.key), field_value);
                }
            }
        }
        self.last = Some(*candle);
    }

    /// The current feature vector, in spec feature order, or `None` until every
    /// axis is ready (a candle has been folded and every indicator has warmed up
    /// and produced its referenced field).
    pub(crate) fn row(&self) -> Option<Vec<f64>> {
        let candle = self.last.as_ref()?;
        let mut row = Vec::with_capacity(self.axes.len());
        for axis in &self.axes {
            let value = match axis {
                Axis::Price(field) => price(candle, *field),
                Axis::Keyed(key) => self.cur.get(key).copied()?,
            };
            row.push(value);
        }
        Some(row)
    }
}

/// Read a price field from a candle.
fn price(candle: &Candle, field: PriceField) -> f64 {
    match field {
        PriceField::Open => candle.open,
        PriceField::High => candle.high,
        PriceField::Low => candle.low,
        PriceField::Close => candle.close,
        PriceField::Volume => candle.volume,
    }
}

/// Canonical base key for an indicator, without any field suffix:
/// `<name>(<p,p,...>)`. Matches `Feature::key` for a field-less indicator.
fn base_key(name: &str, params: &[f64]) -> String {
    Feature::Indicator {
        name: name.to_string(),
        params: params.to_vec(),
        field: None,
    }
    .key()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::feature::PriceField;
    use crate::spec::{Metric, Normalize};

    fn candle(close: f64) -> Candle {
        Candle {
            time: 0,
            open: close,
            high: close,
            low: close,
            close,
            volume: 10.0,
        }
    }

    fn spec(features: Vec<Feature>) -> FingerprintSpec {
        FingerprintSpec {
            features,
            window: 1,
            normalize: Normalize::None,
            metric: Metric::Cosine,
        }
    }

    #[test]
    fn price_only_is_ready_immediately_and_ordered() {
        let mut set = FeatureSet::new(&spec(vec![
            Feature::Price {
                field: PriceField::Close,
            },
            Feature::Price {
                field: PriceField::Volume,
            },
        ]))
        .unwrap();
        assert_eq!(set.row(), None);
        set.update(&candle(42.0));
        assert_eq!(set.row(), Some(vec![42.0, 10.0]));
    }

    #[test]
    fn indicator_gates_row_until_warmup() {
        let mut set = FeatureSet::new(&spec(vec![
            Feature::Price {
                field: PriceField::Close,
            },
            Feature::Indicator {
                name: "Sma".into(),
                params: vec![3.0],
                field: None,
            },
        ]))
        .unwrap();
        set.update(&candle(1.0));
        assert_eq!(set.row(), None); // Sma not warmed up yet
        set.update(&candle(2.0));
        set.update(&candle(3.0));
        // Now the 3-bar SMA is ready: close=3.0, Sma=2.0, in spec order.
        assert_eq!(set.row(), Some(vec![3.0, 2.0]));
    }

    #[test]
    fn unknown_feature_errors() {
        let result = FeatureSet::new(&spec(vec![Feature::Indicator {
            name: "NotAnIndicator".into(),
            params: vec![],
            field: None,
        }]));
        assert!(matches!(result, Err(Error::UnknownFeature(_))));
    }

    #[test]
    fn duplicate_indicator_resolved_once() {
        let set = FeatureSet::new(&spec(vec![
            Feature::Indicator {
                name: "Sma".into(),
                params: vec![3.0],
                field: None,
            },
            Feature::Indicator {
                name: "Sma".into(),
                params: vec![3.0],
                field: None,
            },
        ]))
        .unwrap();
        assert_eq!(set.indicators.len(), 1);
        assert_eq!(set.axes.len(), 2);
    }
}
