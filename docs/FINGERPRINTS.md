# Fingerprints & FingerprintSpec

A **fingerprint** is a fixed-length `f64` vector that captures the state of an
asset at (or around) one bar. A **`FingerprintSpec`** is the data recipe that
defines how every fingerprint in an index is built — an ordered feature list, a
window, a normalization mode and a similarity metric. Because the spec is
**data, not code** (a serde document), the exact same fingerprint definition
crosses the C ABI and WASM unchanged.

## The spec

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

| Field | Meaning |
|-------|---------|
| `features` | Ordered list of axis sources (see [FEATURES.md](FEATURES.md)). The order **is** the axis order. |
| `window` | Consecutive bars stacked into one fingerprint. `1` = the current bar only; `> 1` = a short shape. |
| `normalize` | `none` · `z_score` · `min_max` — fitted over the whole index (see [SIMILARITY.md](SIMILARITY.md)). |
| `metric` | `cosine` · `euclid` · `dtw` — how two fingerprints are compared. |

## Fixed dimension = determinism

The dimension is `N = features.len() * window`, fixed the moment the spec is
parsed and never changing within an index. That is what makes matches
reproducible: axis *k* always means the same feature at the same window offset,
in every language and on both the parallel and sequential build. `dim()` returns
`N`; a spec whose `N` exceeds `MAX_DIM` (4096) is rejected at validation, as is
an empty feature list or a `window` of 0.

## Window and warmup

With `window = w`, a fingerprint at bar *i* stacks the feature vectors of bars
`i-w+1 .. i`. The first fingerprint is therefore emitted only once every feature
has warmed up **and** `w` bars are available; earlier bars produce no fingerprint
(they are skipped, not zero-filled). Longer indicators (e.g. `Sma(200)`) push the
first fingerprint later.

## TOML

Specs can also be written in TOML (chosen by file extension); the CLI's `--spec`
accepts either. The JSON form above is the canonical wire format every binding
speaks.

## See also

- [FEATURES.md](FEATURES.md) · [SIMILARITY.md](SIMILARITY.md) · [LABELS.md](LABELS.md) · [Cookbook.md](Cookbook.md) · [ARCHITECTURE.md](ARCHITECTURE.md)
