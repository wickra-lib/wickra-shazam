//! CLI configuration: a thin wrapper holding a [`FingerprintSpec`] loaded from a
//! JSON or TOML file.

use crate::error::Result;
use crate::spec::FingerprintSpec;

/// A loaded fingerprint configuration. The spec file is a bare `FingerprintSpec`;
/// this wrapper gives the CLI (and future config-level options) a stable type to
/// load into.
#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    /// The fingerprint specification.
    pub spec: FingerprintSpec,
}

impl Config {
    /// Load a config from a JSON spec file (validated).
    pub fn from_json(s: &str) -> Result<Self> {
        Ok(Self {
            spec: FingerprintSpec::from_json(s)?,
        })
    }

    /// Load a config from a TOML spec file (validated).
    pub fn from_toml(s: &str) -> Result<Self> {
        Ok(Self {
            spec: FingerprintSpec::from_toml(s)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const JSON: &str =
        r#"{"features":[{"kind":"price","field":"close"}],"window":2,"metric":"euclid"}"#;

    #[test]
    fn loads_from_json() {
        let cfg = Config::from_json(JSON).unwrap();
        assert_eq!(cfg.spec.window, 2);
        assert_eq!(cfg.spec.features.len(), 1);
    }

    #[test]
    fn loads_from_toml() {
        let toml =
            "window = 3\nmetric = \"cosine\"\n\n[[features]]\nkind = \"price\"\nfield = \"high\"\n";
        let cfg = Config::from_toml(toml).unwrap();
        assert_eq!(cfg.spec.window, 3);
    }
}
