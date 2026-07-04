//! The `Shazam` handle and its single JSON-in / JSON-out command boundary.
//!
//! Every consumer — the CLI and all ten language bindings — drives the core
//! through [`Shazam::command_json`]. The reply is always a JSON string; internal
//! errors come back in-band as `{"ok":false,"error":...}` so every binding parses
//! them identically. `index` must be called before `match`.

use std::collections::BTreeMap;

use serde_json::{json, Value};
use wickra_backtest_core::Candle;

use crate::error::{Error, Result};
use crate::index::{build_index, FingerprintIndex};
use crate::search::{match_index, MatchReport};
use crate::spec::{FingerprintSpec, Metric, Normalize};

/// A stateful fingerprint-matching handle: a spec, an optional built index, and
/// the labels attached to historical timestamps.
pub struct Shazam {
    spec: FingerprintSpec,
    index: Option<FingerprintIndex>,
    labels: BTreeMap<i64, String>,
}

impl Shazam {
    /// Build a handle from a spec JSON. An empty string or `"{}"` yields an empty
    /// handle whose spec is set later with `set_spec`; any other value must be a
    /// valid `FingerprintSpec`.
    pub fn new(spec_json: &str) -> Result<Self> {
        let trimmed = spec_json.trim();
        let spec = if trimmed.is_empty() || trimmed == "{}" {
            empty_spec()
        } else {
            FingerprintSpec::from_json(spec_json)?
        };
        Ok(Self {
            spec,
            index: None,
            labels: BTreeMap::new(),
        })
    }

    /// The library version.
    #[must_use]
    pub fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    /// Replace the spec, discarding any built index (labels are kept).
    pub fn set_spec(&mut self, spec: FingerprintSpec) {
        self.spec = spec;
        self.index = None;
    }

    /// Build the index from a history using the current spec; returns the number
    /// of fingerprints. Any labels set so far are applied to the new index.
    pub fn index(&mut self, history: &[Candle]) -> Result<usize> {
        let mut index = build_index(history, &self.spec)?;
        for (ts, label) in &self.labels {
            index.set_label(*ts, label.clone());
        }
        let indexed = index.indexed();
        self.index = Some(index);
        Ok(indexed)
    }

    /// Attach (or overwrite) a label at a historical timestamp. It takes effect
    /// whenever that timestamp is matched, whether the index exists yet or not.
    pub fn label(&mut self, ts: i64, label: String) {
        self.labels.insert(ts, label.clone());
        if let Some(index) = &mut self.index {
            index.set_label(ts, label);
        }
    }

    /// Match the current fingerprint against the built index. Errors if no index
    /// has been built.
    pub fn match_current(&self, current: &[Candle], k: usize) -> Result<MatchReport> {
        let index = self
            .index
            .as_ref()
            .ok_or_else(|| Error::Data("no index; call `index` first".into()))?;
        match_index(index, current, k)
    }

    /// Clear the index and labels, keeping the spec.
    pub fn reset(&mut self) {
        self.index = None;
        self.labels.clear();
    }

    /// The single JSON-in / JSON-out command boundary. Never returns `Err` for a
    /// well-formed call: internal errors come back as `{"ok":false,"error":...}`.
    pub fn command_json(&mut self, cmd_json: &str) -> Result<String> {
        Ok(self
            .dispatch(cmd_json)
            .unwrap_or_else(|e| error_json(&e.to_string())))
    }

    fn dispatch(&mut self, cmd_json: &str) -> Result<String> {
        let value: Value = serde_json::from_str(cmd_json)?;
        let cmd = value
            .get("cmd")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::BadSpec("missing \"cmd\"".into()))?;
        match cmd {
            "set_spec" => {
                let spec: FingerprintSpec = serde_json::from_value(field(&value, "spec")?)?;
                spec.validate()?;
                self.set_spec(spec);
                Ok(ok_json())
            }
            "index" => {
                let history: Vec<Candle> = serde_json::from_value(field(&value, "history")?)?;
                let indexed = self.index(&history)?;
                Ok(json!({ "ok": true, "indexed": indexed }).to_string())
            }
            "label" => {
                let ts = i64_field(&value, "ts")?;
                let label = str_field(&value, "label")?.to_string();
                self.label(ts, label);
                Ok(ok_json())
            }
            "match" => {
                let current: Vec<Candle> = serde_json::from_value(field(&value, "current")?)?;
                let k = usize_field(&value, "k")?;
                Ok(serde_json::to_string(&self.match_current(&current, k)?)?)
            }
            "reset" => {
                self.reset();
                Ok(ok_json())
            }
            "version" => Ok(json!({ "version": Self::version() }).to_string()),
            other => Err(Error::BadSpec(format!("unknown cmd: {other}"))),
        }
    }
}

/// A placeholder spec for an empty handle (unvalidated; a real spec is set later).
fn empty_spec() -> FingerprintSpec {
    FingerprintSpec {
        features: Vec::new(),
        window: 1,
        normalize: Normalize::None,
        metric: Metric::Cosine,
    }
}

/// Clone a named field out of the envelope, erroring if absent.
fn field(value: &Value, name: &str) -> Result<Value> {
    value
        .get(name)
        .cloned()
        .ok_or_else(|| Error::BadSpec(format!("missing \"{name}\"")))
}

/// Read a named string field out of the envelope.
fn str_field<'a>(value: &'a Value, name: &str) -> Result<&'a str> {
    value
        .get(name)
        .and_then(Value::as_str)
        .ok_or_else(|| Error::BadSpec(format!("missing string \"{name}\"")))
}

/// Read a named i64 field out of the envelope.
fn i64_field(value: &Value, name: &str) -> Result<i64> {
    value
        .get(name)
        .and_then(Value::as_i64)
        .ok_or_else(|| Error::BadSpec(format!("missing integer \"{name}\"")))
}

/// Read a named usize field out of the envelope.
fn usize_field(value: &Value, name: &str) -> Result<usize> {
    let n = value
        .get(name)
        .and_then(Value::as_u64)
        .ok_or_else(|| Error::BadSpec(format!("missing unsigned \"{name}\"")))?;
    Ok(usize::try_from(n).unwrap_or(usize::MAX))
}

fn ok_json() -> String {
    json!({ "ok": true }).to_string()
}

fn error_json(message: &str) -> String {
    json!({ "ok": false, "error": message }).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec_json() -> &'static str {
        r#"{"features":[{"kind":"price","field":"close"}],"window":1,"metric":"euclid"}"#
    }

    fn candles(n: i64) -> String {
        let items: Vec<String> = (0..n)
            .map(|i| {
                format!(
                    r#"{{"time":{i},"open":{i},"high":{i},"low":{i},"close":{i},"volume":1.0}}"#
                )
            })
            .collect();
        format!("[{}]", items.join(","))
    }

    #[test]
    fn version_command() {
        let mut s = Shazam::new(spec_json()).unwrap();
        let out = s.command_json(r#"{"cmd":"version"}"#).unwrap();
        assert_eq!(out, format!(r#"{{"version":"{}"}}"#, Shazam::version()));
    }

    #[test]
    fn index_then_match() {
        let mut s = Shazam::new(spec_json()).unwrap();
        let indexed = s
            .command_json(&format!(r#"{{"cmd":"index","history":{}}}"#, candles(5)))
            .unwrap();
        assert_eq!(indexed, r#"{"indexed":5,"ok":true}"#);
        let report = s
            .command_json(r#"{"cmd":"match","current":[{"time":9,"open":2,"high":2,"low":2,"close":2,"volume":1}],"k":2}"#)
            .unwrap();
        assert!(report.contains(r#""indexed":5"#));
        assert!(report.contains(r#""ts":2"#));
    }

    #[test]
    fn match_without_index_is_in_band_error() {
        let mut s = Shazam::new(spec_json()).unwrap();
        let out = s
            .command_json(r#"{"cmd":"match","current":[],"k":1}"#)
            .unwrap();
        assert!(out.contains(r#""ok":false"#));
    }

    #[test]
    fn unknown_cmd_is_in_band_error() {
        let mut s = Shazam::new(spec_json()).unwrap();
        let out = s.command_json(r#"{"cmd":"nope"}"#).unwrap();
        assert!(out.contains(r#""ok":false"#));
    }

    #[test]
    fn label_rides_a_match() {
        let mut s = Shazam::new(spec_json()).unwrap();
        s.command_json(&format!(r#"{{"cmd":"index","history":{}}}"#, candles(5)))
            .unwrap();
        s.command_json(r#"{"cmd":"label","ts":2,"label":"may_2021_crash"}"#)
            .unwrap();
        let report = s
            .command_json(r#"{"cmd":"match","current":[{"time":9,"open":2,"high":2,"low":2,"close":2,"volume":1}],"k":1}"#)
            .unwrap();
        assert!(report.contains("may_2021_crash"));
    }

    #[test]
    fn empty_handle_then_set_spec() {
        let mut s = Shazam::new("").unwrap();
        assert_eq!(
            s.command_json(&format!(
                r#"{{"cmd":"set_spec","spec":{spec}}}"#,
                spec = spec_json()
            ))
            .unwrap(),
            r#"{"ok":true}"#
        );
        let indexed = s
            .command_json(&format!(r#"{{"cmd":"index","history":{}}}"#, candles(3)))
            .unwrap();
        assert_eq!(indexed, r#"{"indexed":3,"ok":true}"#);
    }

    #[test]
    fn reset_clears_index() {
        let mut s = Shazam::new(spec_json()).unwrap();
        s.command_json(&format!(r#"{{"cmd":"index","history":{}}}"#, candles(3)))
            .unwrap();
        assert_eq!(
            s.command_json(r#"{"cmd":"reset"}"#).unwrap(),
            r#"{"ok":true}"#
        );
        let out = s
            .command_json(r#"{"cmd":"match","current":[{"time":9,"open":1,"high":1,"low":1,"close":1,"volume":1}],"k":1}"#)
            .unwrap();
        assert!(out.contains(r#""ok":false"#));
    }
}
