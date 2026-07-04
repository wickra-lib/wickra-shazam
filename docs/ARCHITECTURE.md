# Architecture (internals)

The top-level [ARCHITECTURE.md](../ARCHITECTURE.md) gives the high-level shape;
this page covers how the core actually builds an index and finds matches. The
whole product is **one data-driven core** (`shazam-core`) and N thin consumers —
the CLI and the ten language bindings — each of which only ships a
`FingerprintSpec` plus candles and reads back a match report.

## The pipeline

```
FingerprintSpec (JSON/TOML)
   │  parse + validate (non-empty features, window >= 1, dim <= MAX_DIM)
   ▼
FeatureSet   one O(1) streaming instance per feature (indicator / price /
   │         microstructure), resolved once from the Wickra registry
   ▼
index(history): fold every candle → at each bar from warmup emit a fixed
   │            Fingerprint([f64; N]) where N = features.len() * window
   ▼
fit normalize (none / z_score / min_max) over the whole index → axis stats
   ▼
match(current, k): build the current fingerprint on the same axes, score it
   │                against every historical fingerprint under `metric`
   ▼
sort by (similarity desc, ts asc), truncate to k, attach labels → MatchReport
```

## Key types

- **`Feature`** (tag `kind`) — one axis source: `indicator` (name + params +
  optional sub-output `field`), `price` (an OHLCV field), `microstructure` (an
  order-book / flow feature). `Feature::key()` is the stable identity string.
- **`FingerprintSpec`** — the ordered `features` list plus `window`, `normalize`
  and `metric`. The feature order **is** the vector's axis order, so the
  dimension `N` is fixed for the life of an index.
- **`FingerprintIndex`** — the rolling history of fingerprints plus the fitted
  normalization axes and any attached labels.
- **`MatchReport`** — `{ matches: [{ ts, similarity, label? }], indexed }`,
  serialized with sorted keys so it is byte-stable.

## Parallel vs sequential

`match` scores the current fingerprint against each historical fingerprint
independently, so the distance sweep runs with rayon by default (`parallel`
feature) and sequentially in the WASM build / `--no-default-features`. The
reduction order over the feature axes is fixed (never rayon-ordered), so both
paths produce **byte-identical** JSON — the `golden` corpus and the
`parallel_eq_seq` test pin that.

## Boundary: JSON in, JSON out

The public surface is a JSON-over-C-ABI data API. `Shazam::command_json` (and
each binding's `command`) takes a command string (`set_spec`, `index`, `match`,
`label`, `reset`, `version`) and returns a response string; the `match` response
is the same `MatchReport` bytes the CLI prints with `--format json`. Because
every binding returns the core's response **verbatim**, the output is identical
in all ten languages — there is no per-language JSON reformatting to drift.

## Integration

The indicator registry, the `Candle` type and the O(1) feature implementations
come from the Wickra ecosystem (`wickra-backtest-core`'s registry over the
`wickra` indicator library); `shazam-core` adds only the spec model, the rolling
fingerprint fold, the normalization / metric layers and the k-nearest search.

## See also

- [FINGERPRINTS.md](FINGERPRINTS.md) · [FEATURES.md](FEATURES.md) · [SIMILARITY.md](SIMILARITY.md) · [LABELS.md](LABELS.md) · [Cookbook.md](Cookbook.md)
