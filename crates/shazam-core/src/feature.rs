//! A single axis of a fingerprint.
//!
//! A [`Feature`] is one component of the fixed-dimension fingerprint vector. It
//! is **data, not code** (a serde enum), so a spec crosses the C ABI and WASM
//! unchanged. Three kinds are on the wire — `indicator`, `price` and
//! `microstructure` — and each carries a canonical [`Feature::key`] string used
//! for stable, human-readable identification of the axis.

use serde::{Deserialize, Serialize};

/// One axis of the fingerprint. A constant is deliberately not allowed —
/// constants carry no signal.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Feature {
    /// A streaming indicator resolved from the `wickra-core` registry by name
    /// and parameters. `field` selects a sub-output of a multi-output indicator
    /// (`None` → the primary field).
    Indicator {
        /// Registry name (e.g. `"rsi"`, `"macd"`).
        name: String,
        /// Numeric parameters, in registry order.
        #[serde(default)]
        params: Vec<f64>,
        /// Optional sub-output field of a multi-output indicator.
        #[serde(default)]
        field: Option<String>,
    },
    /// A raw price field read straight from the candle.
    Price {
        /// Which OHLCV field to read.
        field: PriceField,
    },
    /// A microstructure feature (order-book imbalance, funding, open interest,
    /// liquidations, footprint), resolved from the same registry as `indicator`.
    Microstructure {
        /// Registry name (e.g. `"ob_imbalance"`).
        name: String,
        /// Numeric parameters, in registry order.
        #[serde(default)]
        params: Vec<f64>,
        /// Optional sub-output field.
        #[serde(default)]
        field: Option<String>,
    },
}

/// A raw OHLCV price field.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PriceField {
    /// The bar open.
    Open,
    /// The bar high.
    High,
    /// The bar low.
    Low,
    /// The bar close.
    Close,
    /// The bar volume.
    Volume,
}

impl PriceField {
    /// The canonical lowercase name of this field.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            PriceField::Open => "open",
            PriceField::High => "high",
            PriceField::Low => "low",
            PriceField::Close => "close",
            PriceField::Volume => "volume",
        }
    }
}

impl Feature {
    /// A canonical, stable string identifying this axis. The format is fixed:
    /// `indicator` → `"<name>(<p,p>)"` (optionally `".<field>"`); `price` →
    /// `"price.<field>"`; `microstructure` → `"micro.<name>(<p,p>)"` (optionally
    /// `".<field>"`).
    #[must_use]
    pub fn key(&self) -> String {
        match self {
            Feature::Indicator {
                name,
                params,
                field,
            } => keyed(name, params, field.as_deref(), ""),
            Feature::Price { field } => format!("price.{}", field.as_str()),
            Feature::Microstructure {
                name,
                params,
                field,
            } => keyed(name, params, field.as_deref(), "micro."),
        }
    }
}

/// Render `"<prefix><name>(<p,p>)"` with an optional `".<field>"` suffix.
fn keyed(name: &str, params: &[f64], field: Option<&str>, prefix: &str) -> String {
    let joined = params
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",");
    let mut key = format!("{prefix}{name}({joined})");
    if let Some(field) = field {
        key.push('.');
        key.push_str(field);
    }
    key
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roundtrip(feature: &Feature) -> Feature {
        let json = serde_json::to_string(feature).unwrap();
        serde_json::from_str(&json).unwrap()
    }

    #[test]
    fn indicator_roundtrips_and_keys() {
        let feature = Feature::Indicator {
            name: "rsi".to_string(),
            params: vec![14.0],
            field: None,
        };
        assert_eq!(roundtrip(&feature), feature);
        assert_eq!(feature.key(), "rsi(14)");
    }

    #[test]
    fn indicator_with_field_keys() {
        let feature = Feature::Indicator {
            name: "macd".to_string(),
            params: vec![12.0, 26.0, 9.0],
            field: Some("hist".to_string()),
        };
        assert_eq!(roundtrip(&feature), feature);
        assert_eq!(feature.key(), "macd(12,26,9).hist");
    }

    #[test]
    fn price_roundtrips_and_keys() {
        let feature = Feature::Price {
            field: PriceField::Close,
        };
        assert_eq!(roundtrip(&feature), feature);
        assert_eq!(feature.key(), "price.close");
    }

    #[test]
    fn microstructure_roundtrips_and_keys() {
        let feature = Feature::Microstructure {
            name: "ob_imbalance".to_string(),
            params: vec![10.0],
            field: None,
        };
        assert_eq!(roundtrip(&feature), feature);
        assert_eq!(feature.key(), "micro.ob_imbalance(10)");
    }

    #[test]
    fn indicator_json_shape_is_tagged() {
        let json = serde_json::to_value(Feature::Indicator {
            name: "rsi".to_string(),
            params: vec![14.0],
            field: None,
        })
        .unwrap();
        assert_eq!(json["kind"], "indicator");
        assert_eq!(json["name"], "rsi");
    }

    #[test]
    fn price_field_names_are_snake_case() {
        assert_eq!(
            serde_json::to_string(&PriceField::Volume).unwrap(),
            "\"volume\""
        );
        assert_eq!(PriceField::High.as_str(), "high");
    }
}
