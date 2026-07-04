# Features

A feature is one axis source of a fingerprint. Three kinds go on the wire —
`indicator`, `price` and `microstructure` — each a serde-tagged object under the
`kind` key. Features are resolved from the same Wickra registry the rest of the
ecosystem uses, so any indicator the library ships is usable as a fingerprint
axis.

## `indicator`

A streaming indicator resolved by PascalCase `name` + `params`, with an optional
`field` to select a sub-output of a multi-output indicator:

```json
{ "kind": "indicator", "name": "Rsi", "params": [14] }
{ "kind": "indicator", "name": "Macd", "params": [12, 26, 9], "field": "hist" }
```

Names are PascalCase registry identifiers (`Rsi`, `Sma`, `Ema`, `Atr`, `Roc`,
`Obv`, `Mfi`, `Macd`, `BollingerBands`, …). `params` is the ordered parameter
list; `field` is required only for indicators that emit more than one value.

## `price`

A raw OHLCV field read straight from the candle:

```json
{ "kind": "price", "field": "close" }
{ "kind": "price", "field": "volume" }
```

`field` is one of `open` · `high` · `low` · `close` · `volume`.

## `microstructure`

An order-book / flow feature — imbalance, funding, open interest, liquidations,
footprint — resolved from the same registry as `indicator`. These describe *how*
a move happened, not just the price path, which is what lets a fingerprint
distinguish a thin-liquidity spike from a broad, well-supported trend:

```json
{ "kind": "microstructure", "name": "ChaikinMoneyFlow", "params": [20] }
```

## Keys and axis order

Every feature has a stable `Feature::key()` — `Rsi(14)`, `price.close`,
`Macd(12,26,9).hist` — used internally for de-duplication and for the fitted
normalization axes. The **position** of a feature in the spec's `features` list
fixes which axis of the fingerprint vector it occupies; reordering the list
produces a different (equally valid) fingerprint space.

## Choosing features

- **Regime by momentum / trend** — `Rsi`, `Roc`, `Sma`/`Ema` spreads, `close`.
- **Volatility state** — `Atr`, Bollinger width, high–low range.
- **Participation / conviction** — `volume`, `Obv`, `Mfi`, microstructure flow.

Mixing scales (an `Rsi` in `[0,100]` with a raw `close` in the thousands) is fine
as long as you normalize — see [SIMILARITY.md](SIMILARITY.md).

## See also

- [FINGERPRINTS.md](FINGERPRINTS.md) · [SIMILARITY.md](SIMILARITY.md) · [LABELS.md](LABELS.md) · [Cookbook.md](Cookbook.md) · [ARCHITECTURE.md](ARCHITECTURE.md)
