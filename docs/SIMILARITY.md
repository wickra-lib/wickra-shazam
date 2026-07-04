# Similarity & metrics

Matching turns two fingerprints into a single **similarity** in `[0, 1]`, where
`1` means identical. Two knobs control it: `normalize` (how the axes are scaled
before comparison) and `metric` (how the scaled vectors are compared). Both live
in the [`FingerprintSpec`](FINGERPRINTS.md).

## Normalization

Normalization is **fitted once over the whole index** and the same axis statistics
are reused for the current fingerprint, so history and query always live on the
same axes.

| `normalize` | Effect |
|-------------|--------|
| `none` | Raw values. Only sensible when every feature already shares a scale. |
| `z_score` | Per-axis `(x - mean) / std`. Centres and unit-scales each feature; degenerate (zero-variance) axes collapse to 0. |
| `min_max` | Per-axis `(x - min) / (max - min)` into `[0, 1]`; degenerate axes collapse to 0. |

Because features usually mix scales (an oscillator with a raw price), pair a
scale-sensitive metric (`euclid`, `dtw`) with `z_score` or `min_max` so no single
large-magnitude axis dominates.

## Metrics

| `metric` | Formula | Notes |
|----------|---------|-------|
| `cosine` | `(cos(θ) + 1) / 2` over the flat vectors | Angle-based, magnitude-insensitive. Pairs well with `z_score`. |
| `euclid` | `1 / (1 + d)`, `d` = L2 distance | Magnitude-sensitive. Normalize so axes weigh evenly. |
| `dtw` | banded dynamic time warping over the per-bar feature vectors | For `window > 1`; tolerant of small time shifts. With `window == 1` it equals `euclid`. |

All three map into `[0, 1]` and are rounded deterministically (to `1e-8`) so the
report is byte-stable across languages and build configurations.

## DTW and windows

`dtw` only differs from `euclid` when `window > 1`. It reshapes each fingerprint
back into its `window` per-bar feature vectors and aligns the two sequences with
a banded DP table, where each cell cost is the Euclidean distance over the
feature axes. This absorbs a query whose shape is the same but shifted a bar or
two — a "crash setup forming one bar early" still matches.

## Ranking and ties

`match(current, k)` scores every historical fingerprint, sorts by
`(similarity desc, ts asc)` and keeps the top `k`. The timestamp tie-break makes
equal-similarity matches deterministic (the earliest bar wins), which matters when
a flat or perfectly self-similar history produces many `1.0` scores.

## See also

- [FINGERPRINTS.md](FINGERPRINTS.md) · [FEATURES.md](FEATURES.md) · [LABELS.md](LABELS.md) · [Cookbook.md](Cookbook.md) · [ARCHITECTURE.md](ARCHITECTURE.md)
