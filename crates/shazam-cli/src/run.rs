//! Load the spec and history, build the index, match the current fingerprint,
//! and render the report.

use crate::args::{Args, Format};
use shazam_core::{build_index, match_index, Candle, Config, FingerprintSpec, MatchReport};
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

/// Load the inputs, run the match and return the rendered output.
pub fn run(args: &Args) -> Result<String, String> {
    let spec = load_spec(&args.spec)?;
    let history = load_csv(&args.history)?;
    // The current state defaults to the history itself: its last fingerprint is
    // the most recent state, matched against the whole index.
    let current = match &args.current {
        Some(path) => load_csv(path)?,
        None => history.clone(),
    };

    let mut index = build_index(&history, &spec).map_err(|e| e.to_string())?;
    for label in &args.label {
        let (ts, name) = parse_label(label)?;
        index.set_label(ts, name);
    }

    let report = match_index(&index, &current, args.k).map_err(|e| e.to_string())?;

    Ok(match args.format {
        Format::Json => {
            let mut json = serde_json::to_string(&report).map_err(|e| e.to_string())?;
            json.push('\n');
            json
        }
        Format::Text => render_text(&report),
    })
}

/// Read and parse a spec file, choosing JSON or TOML by extension.
fn load_spec(path: &Path) -> Result<FingerprintSpec, String> {
    let content =
        fs::read_to_string(path).map_err(|e| format!("read spec {}: {e}", path.display()))?;
    let is_toml = path
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| e.eq_ignore_ascii_case("toml"));
    let cfg = if is_toml {
        Config::from_toml(&content)
    } else {
        Config::from_json(&content)
    };
    cfg.map(|c| c.spec).map_err(|e| e.to_string())
}

/// Read a CSV file of OHLCV candles.
fn load_csv(path: &Path) -> Result<Vec<Candle>, String> {
    let content =
        fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    parse_csv(&content)
}

/// Parse OHLCV rows (`ts,open,high,low,close,volume`) into candles; a
/// non-numeric first row is treated as a header and skipped.
fn parse_csv(content: &str) -> Result<Vec<Candle>, String> {
    let mut candles = Vec::new();
    for (idx, line) in content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split(',').map(str::trim).collect();
        if cols.len() < 6 {
            return Err(format!(
                "CSV line {}: expected 6 columns, got {}",
                idx + 1,
                cols.len()
            ));
        }
        let time = match cols[0].parse::<i64>() {
            Ok(t) => t,
            Err(_) if idx == 0 => continue, // header row
            Err(e) => return Err(format!("CSV line {}: bad timestamp: {e}", idx + 1)),
        };
        let field = |i: usize, name: &str| {
            cols[i]
                .parse::<f64>()
                .map_err(|e| format!("CSV line {}: {name}: {e}", idx + 1))
        };
        candles.push(Candle {
            time,
            open: field(1, "open")?,
            high: field(2, "high")?,
            low: field(3, "low")?,
            close: field(4, "close")?,
            volume: field(5, "volume")?,
        });
    }
    Ok(candles)
}

/// Parse a `<ts>=<name>` label argument.
fn parse_label(arg: &str) -> Result<(i64, String), String> {
    let (ts, name) = arg
        .split_once('=')
        .ok_or_else(|| format!("bad label {arg:?}: expected <ts>=<name>"))?;
    let ts = ts
        .trim()
        .parse::<i64>()
        .map_err(|e| format!("bad label timestamp in {arg:?}: {e}"))?;
    Ok((ts, name.trim().to_string()))
}

/// Render a match report as an aligned text table.
fn render_text(report: &MatchReport) -> String {
    if report.matches.is_empty() {
        return format!("no matches ({} indexed)\n", report.indexed);
    }

    let header = ["rank", "ts", "similarity", "label"];
    let mut rows: Vec<[String; 4]> = Vec::new();
    for (rank, m) in report.matches.iter().enumerate() {
        rows.push([
            format!("{}", rank + 1),
            format!("{}", m.ts),
            format!("{}", m.similarity),
            m.label.clone().unwrap_or_else(|| "-".to_string()),
        ]);
    }

    let mut widths: [usize; 4] = header.map(str::len);
    for row in &rows {
        for (i, cell) in row.iter().enumerate() {
            widths[i] = widths[i].max(cell.len());
        }
    }

    let format_row = |cells: &[String]| -> String {
        cells
            .iter()
            .enumerate()
            .map(|(i, cell)| format!("{cell:<width$}", width = widths[i]))
            .collect::<Vec<_>>()
            .join("  ")
    };

    let mut out = String::new();
    out.push_str(&format_row(&header.map(String::from)));
    out.push('\n');
    for row in &rows {
        out.push_str(&format_row(row));
        out.push('\n');
    }
    let _ = write!(
        out,
        "\n{} match(es), {} indexed\n",
        report.matches.len(),
        report.indexed
    );
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_csv_with_a_header() {
        let csv = "ts,open,high,low,close,volume\n1,10,11,9,10.5,100\n2,10.5,12,10,11,200\n";
        let candles = parse_csv(csv).unwrap();
        assert_eq!(candles.len(), 2);
        assert_eq!(candles[0].time, 1);
        assert!((candles[1].close - 11.0).abs() < 1e-9);
    }

    #[test]
    fn parse_csv_rejects_a_short_row() {
        assert!(parse_csv("1,2,3\n").is_err());
    }

    #[test]
    fn parses_a_label() {
        assert_eq!(
            parse_label("100=may_2021_crash").unwrap(),
            (100, "may_2021_crash".to_string())
        );
        assert!(parse_label("nope").is_err());
    }

    #[test]
    fn render_text_reports_no_matches() {
        let report = MatchReport {
            matches: vec![],
            indexed: 5,
        };
        assert!(render_text(&report).contains("no matches"));
    }
}
