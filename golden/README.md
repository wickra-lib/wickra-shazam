# Golden fixtures

The golden fixtures pin the shazam's output byte-for-byte. They are generated
once and replayed everywhere: the Rust core, the CLI and every language binding
must reproduce `expected/<spec>.json` **exactly**. Byte equality holds across all
ten languages because each binding returns the core's `command_json` string
verbatim — there is no per-language JSON re-formatting — and every reduction runs
serially in axis order, so the parallel (rayon) and sequential (WASM) builds
agree bit-for-bit.

> **Do not edit any file under `golden/` by hand.** Regenerate them with the
> bless command below and commit the result.

## Layout

| Path | What |
|------|------|
| `data/history/sym-01.csv` … `sym-06.csv` | The canonical universe — each asset's full history (OHLCV) to index. |
| `data/current/sym-01.csv` … `sym-06.csv` | Each asset's current-state window to match (the deterministic tail of its history). |
| `specs/*.json` | The four canonical fingerprint specs. |
| `expected/<spec>.json` | The byte-exact `MatchReport` for each spec. |

## Data formula

Each symbol `s ∈ {1..6}` has 64 bars. Bar `i` (0-based) is derived from a single
closed-form price function

```
p(x) = 100 + 20*s + 12*sin((x + 5*s) * 0.20) + 0.25*x - 25*exp(-((x - 44)^2) / 6)
```

— a per-symbol cycle plus linear drift with a deterministic Gaussian **crash**
centered at bar 44 (so a recognizable crash-setup regime exists to match). The
OHLCV fields are:

```
time(i)   = 1_700_000_000 + 3600 * i          (hourly)
open(i)   = round(p(i),     4)
close(i)  = round(p(i + 0.5), 4)
high(i)   = round(max(open, close) + 0.5 + 0.05 * ((i*7 + s) mod 10), 4)
low(i)    = round(min(open, close) - 0.5 - 0.05 * ((i*3 + s) mod 10), 4)
volume(i) = round(1000 + 8*i + 25*s, 1)
```

Every value is written with at most four decimals, so the CSV text parses to the
**identical** `f64` in every language — this is what makes the golden
byte-identical across languages.

`data/current/sym-NN.csv` is simply the header plus the **last 40 bars** of the
matching `data/history/sym-NN.csv` — long enough to cover the largest
warmup + window across all four specs.

## Specs

| spec | features | window | normalize | metric |
|------|----------|-------:|-----------|--------|
| `crash_setup` | `Rsi(14)`, `Roc(10)`, `close` | 10 | z-score | cosine |
| `regime_zscore` | `Rsi(14)`, `Sma(20)`, `Atr(14)`, `close`, `volume` | 1 | z-score | cosine |
| `microstructure_dtw` | `Obv`, `Mfi(14)`, `ForceIndex(14)` (as `microstructure`) | 8 | none | DTW |
| `price_euclid` | `close` | 5 | none | euclid |

Indicator and microstructure features resolve through the backtest registry's
PascalCase kinds (`Rsi`, `Roc`, `Sma`, `Atr`, `Obv`, `Mfi`, `ForceIndex`).
Microstructure features resolve through the same candle-only registry, so the
DTW spec uses candle-computable flow indicators.

## Canonical harness

Each `expected/<spec>.json` is the `MatchReport` from indexing **`sym-01`'s
history** and matching its **current** window with **`k = 5`** — the exact call
every binding reproduces:

```text
index(history = data/history/sym-01.csv)
match(current = data/current/sym-01.csv, k = 5)
```

## Bless (regenerate)

`expected/*.json` is the core's compact `MatchReport` JSON, byte-for-byte. To
re-bless from the CLI:

```bash
cargo build -p wickra-shazam --release
for spec in crash_setup regime_zscore microstructure_dtw price_euclid; do
  ./target/release/wickra-shazam \
    --spec golden/specs/$spec.json \
    --history golden/data/history/sym-01.csv \
    --current golden/data/current/sym-01.csv \
    --k 5 --format json | tr -d '\n' > golden/expected/$spec.json
done
```

Run it once to bless, review the diff, and commit. Regenerate the CSV universe
only from the formula above — never by editing the files.

## Cross-language verification

Every binding replays the same fixtures and asserts byte equality against
`expected/*.json`, so the golden is the cross-language contract: a binding loads
the history and current CSVs, drives an `index` then a `match` command, and
compares the returned string to `expected/<spec>.json`. Byte equality holds
regardless of how each language serializes the input, because the core returns
its `command_json` string verbatim and runs every reduction in serial axis order.
