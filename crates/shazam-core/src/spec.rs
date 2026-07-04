//! The fingerprint specification — the data that defines a fingerprint.
//!
//! A [`FingerprintSpec`] is an ordered list of [`Feature`] axes plus a rolling
//! `window`, an axis [`Normalize`] mode and a distance [`Metric`]. The feature
//! order fixes the vector axes 1:1, and the dimension `N = features.len() *
//! window` is constant across an index — that fixed dimension is what makes the
//! whole pipeline deterministic.

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::feature::Feature;

/// Hard upper bound on the fingerprint dimension `N`, guarding against an
/// out-of-memory allocation from a hostile spec (see the threat model).
pub const MAX_DIM: usize = 4096;

/// How each feature axis is normalized across the index before distances are
/// computed. Different magnitudes (RSI 0–100 vs price 60000) are made
/// comparable by normalizing per axis over the whole history.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum Normalize {
    /// Raw values, no normalization.
    #[default]
    None,
    /// `(v - mean) / std_pop` per axis (`std_pop == 0` → `0`).
    ZScore,
    /// `(v - min) / (max - min)` per axis (`max == min` → `0`), range `[0, 1]`.
    MinMax,
}

/// The distance metric turned into a `[0, 1]` similarity (1 = identical).
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Metric {
    /// Cosine similarity mapped to `[0, 1]` via `(cos + 1) / 2`.
    Cosine,
    /// Euclidean distance mapped via `1 / (1 + d)`.
    Euclid,
    /// Dynamic time warping over the window (Sakoe-Chiba band), `1 / (1 + d)`.
    Dtw,
}

/// A complete fingerprint specification.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FingerprintSpec {
    /// The ordered feature axes; determines the vector axes 1:1.
    pub features: Vec<Feature>,
    /// Bars per fingerprint (`>= 1`); `> 1` yields a sequence fingerprint.
    #[serde(default = "one")]
    pub window: usize,
    /// Axis normalization mode (default `None`).
    #[serde(default)]
    pub normalize: Normalize,
    /// Distance metric (default `Cosine`).
    #[serde(default = "cosine")]
    pub metric: Metric,
}

fn one() -> usize {
    1
}

fn cosine() -> Metric {
    Metric::Cosine
}

impl FingerprintSpec {
    /// Parse a spec from JSON and validate it.
    pub fn from_json(s: &str) -> Result<Self> {
        let spec: FingerprintSpec = serde_json::from_str(s)?;
        spec.validate()?;
        Ok(spec)
    }

    /// Parse a spec from TOML and validate it.
    pub fn from_toml(s: &str) -> Result<Self> {
        let spec: FingerprintSpec = toml::from_str(s)?;
        spec.validate()?;
        Ok(spec)
    }

    /// The fixed fingerprint dimension `N = features.len() * window`.
    #[must_use]
    pub fn dim(&self) -> usize {
        self.features.len().saturating_mul(self.window)
    }

    /// Structural validation: the feature list is non-empty, the window is at
    /// least one, and the dimension does not exceed [`MAX_DIM`]. (Feature
    /// existence in the registry is enforced when the feature set is built.)
    pub(crate) fn validate(&self) -> Result<()> {
        if self.features.is_empty() {
            return Err(Error::BadSpec("features is empty".into()));
        }
        if self.window < 1 {
            return Err(Error::BadSpec("window must be >= 1".into()));
        }
        let dim = self.dim();
        if dim > MAX_DIM {
            return Err(Error::BadSpec(format!(
                "dimension {dim} exceeds MAX_DIM {MAX_DIM}"
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::feature::{Feature, PriceField};

    fn spec_json() -> &'static str {
        r#"{"features":[{"kind":"price","field":"close"},{"kind":"indicator","name":"rsi","params":[14]}],"window":3,"normalize":"z_score","metric":"euclid"}"#
    }

    #[test]
    fn parses_and_computes_dim() {
        let spec = FingerprintSpec::from_json(spec_json()).unwrap();
        assert_eq!(spec.features.len(), 2);
        assert_eq!(spec.window, 3);
        assert_eq!(spec.normalize, Normalize::ZScore);
        assert_eq!(spec.metric, Metric::Euclid);
        assert_eq!(spec.dim(), 6);
    }

    #[test]
    fn defaults_apply() {
        let spec: FingerprintSpec =
            serde_json::from_str(r#"{"features":[{"kind":"price","field":"open"}]}"#).unwrap();
        assert_eq!(spec.window, 1);
        assert_eq!(spec.normalize, Normalize::None);
        assert_eq!(spec.metric, Metric::Cosine);
        assert_eq!(spec.dim(), 1);
    }

    #[test]
    fn empty_features_rejected() {
        let err = FingerprintSpec::from_json(r#"{"features":[]}"#).unwrap_err();
        assert!(matches!(err, Error::BadSpec(_)));
    }

    #[test]
    fn zero_window_rejected() {
        let json = r#"{"features":[{"kind":"price","field":"close"}],"window":0}"#;
        assert!(matches!(
            FingerprintSpec::from_json(json).unwrap_err(),
            Error::BadSpec(_)
        ));
    }

    #[test]
    fn over_max_dim_rejected() {
        let spec = FingerprintSpec {
            features: vec![Feature::Price {
                field: PriceField::Close,
            }],
            window: MAX_DIM + 1,
            normalize: Normalize::None,
            metric: Metric::Cosine,
        };
        assert!(matches!(spec.validate().unwrap_err(), Error::BadSpec(_)));
    }

    #[test]
    fn roundtrips_through_json() {
        let spec = FingerprintSpec::from_json(spec_json()).unwrap();
        let json = serde_json::to_string(&spec).unwrap();
        let back: FingerprintSpec = serde_json::from_str(&json).unwrap();
        assert_eq!(spec, back);
    }

    #[test]
    fn parses_from_toml() {
        let toml = "window = 2\nnormalize = \"min_max\"\nmetric = \"cosine\"\n\n[[features]]\nkind = \"price\"\nfield = \"high\"\n";
        let spec = FingerprintSpec::from_toml(toml).unwrap();
        assert_eq!(spec.window, 2);
        assert_eq!(spec.normalize, Normalize::MinMax);
        assert_eq!(spec.dim(), 2);
    }
}
