# Architecture

`wickra-shazam` is one data-driven core with many thin consumers. A fingerprint
is a piece of **data** — a serde `FingerprintSpec` (an ordered feature list, a
window, a normalization and a distance metric) — that is rolled over an asset's
whole history with the [Wickra](https://github.com/wickra-lib/wickra) feature
space (514 O(1) streaming indicators plus price and microstructure). Because the
fingerprint is data, not code, indexing and matching run natively, across the
C ABI and in WASM, byte-for-byte identical.

## The layers

```
CONSUMERS   CLI: crates/shazam-cli        ·   any language via its binding (command JSON)
      ▲ MatchReport JSON                                    ▲
CORE  crates/shazam-core:  FingerprintSpec (JSON) → rolling Fingerprint([f64; N]) over history
                           → FingerprintIndex → match(current, k) → Vec<HistoricalMatch>
      ▼ data-driven JSON API in ten languages (like backtest run_json / terminal command_json)
BINDINGS  python · node · wasm · c (C-ABI hub) → c / c++ / c# / go / java / r
CORES  wickra-core (indicators + microstructure) · wickra-data (Candle / CSV) · [feature "live"] wickra-exchange
```

Each binding ships the same surface — a `Shazam` handle plus
`command(json) -> json` and `version` — with its own README, tests, a runnable
example, and a completeness guard.

## The core is data-driven

A fingerprint is a serde `FingerprintSpec`: an ordered list of `Feature` axes
(`indicator`, `price`, `microstructure`), a rolling `window`, a `normalize` mode
and a `metric`. It is never a set of Rust closures — closures cannot cross the
C ABI or compile to a WASM data boundary; a serde spec can. So a Python or Go
caller sends the same `FingerprintSpec` JSON a Rust caller would, and gets the
same `MatchReport` back. The **feature order in the spec fixes the vector axes
1:1**, and the dimension `N` never changes within an index — that fixed dimension
is what makes the whole pipeline deterministic.

## Three operations, one result type

- **index** — `index(history, spec)` folds the whole history and emits, at every
  bar from warmup onward, a fixed `Fingerprint([f64; N])` (rolling), building a
  `FingerprintIndex`. Bars fold independently, so it runs in parallel via rayon
  (the default `parallel` feature) and sequentially as the WASM fallback
  (`--no-default-features`) — the two paths produce a byte-identical index.
- **match** — `match_current(index, current, k)` computes the fingerprint of the
  current state and finds the `k` most similar historical fingerprints under the
  chosen metric, returning `Vec<HistoricalMatch>`.
- **label** — an optional human name (`"may_2021_crash"`) attached to a historical
  timestamp comes back on any match that lands there.

## The command boundary

Every consumer talks to the core through a single JSON-in / JSON-out function,
`Shazam::command`. The binding does no logic of its own — it forwards the command
string and returns the core's response verbatim. That verbatim pass-through is
what makes the golden corpus a **cross-language** parity corpus: the same command
produces a byte-identical `MatchReport` in every language, with no per-language
JSON reformatting.

## Determinism

The match output is byte-identical across all ten languages and between the
parallel (rayon) and sequential (WASM) builds. This is enforced by construction:
`BTreeMap` in every output path, a stably-sorted result vector with ties broken by
timestamp, no RNG, similarity rounded to `1e-8`, and every reduction (feature
folds, normalization, distances) run serially in a fixed feature-and-window order
rather than rayon order. Degenerate metric cases are clamped so `NaN`/`±inf`
never reach the output (cosine at zero norm → 0, min-max at `max == min` → 0,
z-score at zero std → 0).

## The feature space comes from the Wickra core

No indicator mathematics lives in this repository. Each `Feature::Indicator` axis
resolves from the `wickra-core` registry by name and parameters (the same
resolver the backtester uses), so shazam inherits all 514 indicators and any
future additions for free. `Feature::Price` reads straight from the candle, and
`Feature::Microstructure` reads order-book imbalance, funding, open interest,
liquidations and footprint from the same core.

## Integration with the rest of Wickra

`wickra-shazam` sits beside the other Wickra consumers — the terminal, the
backtester, the screener and the exchange layer — over the same core. It depends
on `wickra-core` (indicators + microstructure) and `wickra-data` (`Candle` +
CSV); the optional `live` feature pulls `wickra-exchange` to source the current
fingerprint from a live feed. It never places orders and holds no order-secret
material — a historical match is a statistical resemblance, not a prediction.
