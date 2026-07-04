<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Shazam — match an asset's current microstructure fingerprint against its entire history" width="100%"></a>
</p>

[![Built on Wickra](https://img.shields.io/badge/built%20on-wickra-3b82f6)](https://github.com/wickra-lib/wickra)
[![Status](https://img.shields.io/badge/status-pre--release-orange)](https://github.com/wickra-lib/wickra-shazam)
[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/ci.svg)](https://github.com/wickra-lib/wickra-shazam/actions/workflows/ci.yml)
[![CodeQL](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/codeql.svg)](https://github.com/wickra-lib/wickra-shazam/actions/workflows/codeql.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-shazam)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/license.svg)](#license)
[![OpenSSF Scorecard](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/scorecard.svg)](https://scorecard.dev/viewer/?uri=github.com/wickra-lib/wickra-shazam)
[![OpenSSF Best Practices](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/best-practices.svg)](https://www.bestpractices.dev/)
[![Build provenance](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/provenance.svg)](https://github.com/wickra-lib/wickra-shazam/attestations)
[![Docs](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/docs.svg)](https://wickra.org)

---

# Wickra Shazam

**Point at live data → "that's the May-2021 crash setup". Match the current microstructure fingerprint of an asset against its entire history.**

Wickra Shazam turns an asset's **whole history** into a rolling index of
fixed-dimension **microstructure fingerprints** — a vector built from the full
[Wickra](https://github.com/wickra-lib/wickra) feature space (indicators, price,
and microstructure: order-book imbalance, funding, open interest, liquidations,
footprint) — and matches the **current** fingerprint against that entire index to
name the regime. It is pattern/regime recognition over the **full feature space,
not price alone**.

- **The fingerprint is data** — a serde `FingerprintSpec` (an ordered feature
  list + `window` + `normalize` + `metric`), not Rust closures, so it crosses the
  C ABI and WASM unchanged. A fixed dimension `N` is what makes it deterministic.
- **Deterministic core** — indexing and matching are byte-identical across all
  ten languages and between the parallel (rayon) and sequential (WASM) builds.
- **Three operations, one core** — `index(history, spec)` builds the rolling
  index, `match_current(index, current, k)` finds the `k` most similar historical
  fingerprints, and a label attaches a human name (`"may_2021_crash"`) to a match.

The core is one library ([`shazam-core`](crates/shazam-core)), usable from
**Rust, Python, Node.js, WASM, C, C++, C#, Go, Java and R** over a
JSON-over-C-ABI boundary, plus a reference CLI.

## Status

**Pre-release — functionally complete, CI-verified, not yet published.** The core,
the CLI, all ten language bindings, the byte-exact golden corpus, property + fuzz
tests, benchmarks and one runnable example per language are in place and green
across the full CI matrix (10 languages × 3 OS). Not yet released to any
registry — track progress in [ROADMAP.md](ROADMAP.md).

## Documentation

- [Architecture](ARCHITECTURE.md) — the core, the data-driven boundary, the binding surface.
- Guides under [`docs/`](docs): [Fingerprints & FingerprintSpec](docs/FINGERPRINTS.md) · [Features](docs/FEATURES.md) · [Similarity & metrics](docs/SIMILARITY.md) · [Labels](docs/LABELS.md) · [Cookbook](docs/Cookbook.md) · [Internals](docs/ARCHITECTURE.md).
- [ROADMAP.md](ROADMAP.md) · [BENCHMARKS.md](BENCHMARKS.md) · [THREAT_MODEL.md](THREAT_MODEL.md) · [SECURITY.md](SECURITY.md).

## Quickstart

```bash
# Index a history and match the current state, human-readable table:
cargo run -p wickra-shazam -- --spec golden/specs/crash_setup.json \
  --history golden/data/history/sym-01.csv --current golden/data/current/sym-01.csv

# Raw MatchReport JSON (the same bytes every binding returns), top 5 matches:
cargo run -p wickra-shazam -- --spec golden/specs/price_euclid.json \
  --history golden/data/history/sym-01.csv --k 5 --format json
```

`--current` defaults to the last `window` bars of `--history`. Attach a label to
a historical bar with `--label <ts>=<name>` (repeatable) and it comes back on any
match at that timestamp.

## FingerprintSpec / features

A spec is a JSON (or TOML) document: an ordered `features` list, a `window`, a
`normalize` mode and a `metric`. The feature order **is** the vector's axis order
and never changes within an index, so the dimension `N = features.len() * window`
is fixed and the fingerprint is fully deterministic.

```json
{
  "features": [
    { "kind": "indicator", "name": "Rsi", "params": [14] },
    { "kind": "indicator", "name": "Sma", "params": [20] },
    { "kind": "indicator", "name": "Atr", "params": [14] },
    { "kind": "price", "field": "close" },
    { "kind": "price", "field": "volume" }
  ],
  "window": 1,
  "normalize": "z_score",
  "metric": "cosine"
}
```

- **`indicator`** — any PascalCase Wickra indicator resolved from the registry by
  `name` + `params` (`Rsi`, `Sma`, `Atr`, `Macd`, …), with an optional `field` to
  pick a sub-output of a multi-output indicator.
- **`price`** — a raw OHLCV field (`open`/`high`/`low`/`close`/`volume`).
- **`microstructure`** — an order-book / flow feature (imbalance, funding, open
  interest, liquidations, footprint), resolved from the same registry.
- **`window`** — how many consecutive bars are stacked into one fingerprint
  (`1` = the current bar only; `> 1` = a short shape).

## Similarity & metrics

The `metric` decides how two fingerprints are compared. Similarity is always
mapped to `[0, 1]` (1 = identical) and rounded deterministically:

- **`cosine`** — cosine of the angle between the flat vectors, mapped from
  `[-1, 1]` to `[0, 1]` via `(cos + 1) / 2`. Scale-insensitive; good with
  `z_score` normalization.
- **`euclid`** — `1 / (1 + d)` where `d` is the L2 distance. Scale-sensitive;
  pair with `min_max` or `z_score` to weight features evenly.
- **`dtw`** — dynamic time warping over the per-bar feature vectors of a
  `window > 1` spec, tolerant of small time shifts between two shapes. With
  `window == 1` it is identical to `euclid`.

`normalize` (`none` · `z_score` · `min_max`) is fitted once over the whole index
and reused for the current fingerprint, so history and query live on the same
axes.

## Labels

A label attaches a human-readable name to a historical timestamp; when a match
lands on that bar the name rides along in the report:

```jsonc
{ "cmd": "label", "ts": 1700216000, "label": "may_2021_crash" }
// → a later match at ts 1700216000 comes back as
//   { "ts": 1700216000, "similarity": 0.98, "label": "may_2021_crash" }
```

## Use in any language

The same `Shazam` handle — construct from a JSON spec, drive with
`command(json) -> json`, read `version` — is reachable from every binding. The
commands are `set_spec`, `index`, `match`, `label`, `reset` and `version`;
`index` returns `{"indexed":N}` and `match` returns a `MatchReport` that is
byte-identical to the CLI's `--format json`.

```python
from wickra_shazam import Shazam
s = Shazam('{"features":[{"kind":"price","field":"close"}],'
           '"window":1,"metric":"euclid"}')
s.command('{"cmd":"index","history":[/* candles */]}')
report = s.command('{"cmd":"match","current":[/* candles */],"k":5}')  # JSON MatchReport
```

The C ABI hub ([`bindings/c`](bindings/c)) backs C, C++, C#, Go, Java and R;
Rust, Python, Node.js and WASM are native. See each `bindings/<lang>/README.md`
and the runnable [`examples/`](examples).

## Project layout

```
crates/shazam-core     the deterministic core (FingerprintSpec, index, match_current, labels)
crates/shazam-cli      the CLI (bin: wickra-shazam)
crates/shazam-bench    criterion benchmarks
bindings/{python,node,wasm,c,go,csharp,java,r}   the ten-language surface
golden/                CSV histories, current windows, specs, and byte-exact expected reports
fuzz/                  cargo-fuzz targets (spec_parse, build_index, match_index, normalize_metric)
examples/              one runnable "index a history and match the current state" example per language
```

## Building from source

```bash
cargo build --workspace
cargo test  --workspace --all-features
cargo test  --workspace --no-default-features   # sequential (WASM) index/match path
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo run -p wickra-shazam -- --spec golden/specs/crash_setup.json \
  --history golden/data/history/sym-01.csv
```

## Requirements

- **Rust** ≥ 1.86 (workspace MSRV; the Node binding needs ≥ 1.88).
- Binding toolchains as needed: Node ≥ 22, Python ≥ 3.9, a C toolchain, .NET 8,
  JDK 22+, Go 1.23, R — see each `bindings/<lang>/README.md`.

## Benchmarks

`crates/shazam-bench` measures `build_index` scaling by history length and
feature count, and `match_index` by index size and metric (cosine / euclid /
dtw), parallel vs sequential. See [BENCHMARKS.md](BENCHMARKS.md).

## Ecosystem

Part of the [Wickra](https://github.com/wickra-lib/wickra) family — each one a
data-driven core with a CLI and the same ten-language binding surface:

- [**wickra**](https://github.com/wickra-lib/wickra) — the core library: 514 O(1) streaming indicators across ten languages
- [**wickra-exchange**](https://github.com/wickra-lib/wickra-exchange) — unified market-data + execution across ten crypto exchanges
- [**wickra-backtest**](https://github.com/wickra-lib/wickra-backtest) — event-driven backtester over the Wickra core
- [**wickra-terminal**](https://github.com/wickra-lib/wickra-terminal) — the trading terminal: a TUI and a browser renderer over the stack
- [**wickra-screener**](https://github.com/wickra-lib/wickra-screener) — parallel multi-symbol screening over 514 streaming indicators
- [**wickra-xray**](https://github.com/wickra-lib/wickra-xray) — market-microstructure explorer: footprint, order-book heatmap, liquidation map, funding/OI divergence
- **wickra-radar**, **wickra-copilot** — *coming soon*

Docs at [docs.wickra.org](https://docs.wickra.org); the marketing site and
in-browser demo at [wickra.org](https://wickra.org).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).
Commits are signed and in English; open a PR against `main`.

## Security

See [SECURITY.md](SECURITY.md) and [THREAT_MODEL.md](THREAT_MODEL.md). Report
vulnerabilities privately — never in a public issue.

## License

Dual-licensed under either [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at
your option.

## Disclaimer

Wickra Shazam is analysis software: it computes similarity between market states.
A historical match is a statistical resemblance, **not a prediction** and **not
financial advice** — the past setup did not have to repeat, and neither does this
one. It places no orders. Trading carries risk of loss; review the code and use
at your own discretion.
