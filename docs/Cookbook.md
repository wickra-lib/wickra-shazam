# Cookbook

Ready-to-adapt [`FingerprintSpec`](FINGERPRINTS.md) recipes. Each is a complete
spec — drop it in a file and run it with the CLI against a history:

```bash
cargo run -p wickra-shazam -- --spec recipe.json \
  --history golden/data/history/sym-01.csv --k 5 --format json
```

The `golden/specs/` directory holds these as runnable, byte-pinned fixtures.

## Plain price shape (Euclidean)

Match the recent close-price path, nothing else — the simplest fingerprint:

```json
{
  "features": [{ "kind": "price", "field": "close" }],
  "window": 5,
  "normalize": "none",
  "metric": "euclid"
}
```

## Regime by oscillators (z-score cosine)

A multi-feature snapshot of the current bar — momentum, trend, volatility and
participation — compared by angle so absolute levels do not dominate:

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

## Crash setup (windowed shape, cosine)

Stack ten bars into one fingerprint so the spec captures a *forming shape*, not a
single instant — "the market looked like this over the last ten bars":

```json
{
  "features": [
    { "kind": "indicator", "name": "Roc", "params": [1] },
    { "kind": "indicator", "name": "Atr", "params": [14] }
  ],
  "window": 10,
  "normalize": "z_score",
  "metric": "cosine"
}
```

## Microstructure DTW (time-shift tolerant)

Flow features over a window, compared with DTW so a setup forming a bar early or
late still matches — see [SIMILARITY.md](SIMILARITY.md):

```json
{
  "features": [
    { "kind": "microstructure", "name": "ChaikinMoneyFlow", "params": [20] },
    { "kind": "indicator", "name": "Obv", "params": [] }
  ],
  "window": 8,
  "normalize": "min_max",
  "metric": "dtw"
}
```

## Name the moment (labels)

Any recipe becomes more legible with [labels](LABELS.md) — tag the bars you care
about and they come back on the match:

```python
from wickra_shazam import Shazam
s = Shazam(open("recipe.json").read())
s.command('{"cmd":"index","history":[...]}')
s.command('{"cmd":"label","ts":1700216000,"label":"may_2021_crash"}')
print(s.command('{"cmd":"match","current":[...],"k":5}'))  # JSON MatchReport
```

## See also

- [FINGERPRINTS.md](FINGERPRINTS.md) · [FEATURES.md](FEATURES.md) · [SIMILARITY.md](SIMILARITY.md) · [LABELS.md](LABELS.md) · [ARCHITECTURE.md](ARCHITECTURE.md)
