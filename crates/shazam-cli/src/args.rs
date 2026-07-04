//! CLI argument parsing.

use clap::{Parser, ValueEnum};
use std::path::PathBuf;

/// Match the current fingerprint of an asset against its history.
#[derive(Parser, Debug)]
#[command(name = "wickra-shazam", version, about)]
pub struct Args {
    /// Path to the fingerprint spec (JSON or TOML, chosen by file extension).
    #[arg(long)]
    pub spec: PathBuf,

    /// CSV file of the asset's history candles to index.
    #[arg(long)]
    pub history: PathBuf,

    /// CSV file of the current-state candles to match. Defaults to the last
    /// `window` bars of the history.
    #[arg(long)]
    pub current: Option<PathBuf>,

    /// Number of matches to return.
    #[arg(long, default_value_t = 5)]
    pub k: usize,

    /// Attach a label to a historical timestamp, as `<ts>=<name>`. Repeatable.
    #[arg(long)]
    pub label: Vec<String>,

    /// Output format.
    #[arg(long, value_enum, default_value_t = Format::Text)]
    pub format: Format,
}

/// The report output format.
#[derive(Clone, Copy, Debug, ValueEnum, PartialEq, Eq)]
pub enum Format {
    /// A human-readable aligned table.
    Text,
    /// The raw `MatchReport` JSON.
    Json,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn arg_config_is_valid() {
        Args::command().debug_assert();
    }

    #[test]
    fn parses_defaults() {
        let args =
            Args::try_parse_from(["wickra-shazam", "--spec", "s.json", "--history", "h.csv"])
                .unwrap();
        assert_eq!(args.k, 5);
        assert_eq!(args.format, Format::Text);
        assert!(args.current.is_none());
        assert!(args.label.is_empty());
    }

    #[test]
    fn parses_overrides_and_repeated_labels() {
        let args = Args::try_parse_from([
            "wickra-shazam",
            "--spec",
            "s.toml",
            "--history",
            "h.csv",
            "--current",
            "c.csv",
            "--k",
            "3",
            "--format",
            "json",
            "--label",
            "100=may_2021_crash",
            "--label",
            "200=covid_crash",
        ])
        .unwrap();
        assert_eq!(args.k, 3);
        assert_eq!(args.format, Format::Json);
        assert_eq!(args.current, Some(PathBuf::from("c.csv")));
        assert_eq!(args.label.len(), 2);
    }

    #[test]
    fn spec_and_history_are_required() {
        assert!(Args::try_parse_from(["wickra-shazam", "--spec", "s.json"]).is_err());
    }
}
