# Wickra Shazam — Node.js

Node.js bindings for [wickra-shazam](https://github.com/wickra-lib/wickra-shazam),
the data-driven history-fingerprint match core, powered by Rust via napi-rs.
Build a `Shazam` from a spec JSON, index an asset's history, and match the
current fingerprint against it — the same command protocol every language
binding speaks.

## Install

```sh
npm install wickra-shazam
```

## Usage

```js
const { Shazam, version } = require("wickra-shazam");

const spec = JSON.stringify({
  features: [{ kind: "price", field: "close" }],
  window: 1,
  metric: "euclid",
});

const shazam = new Shazam(spec);

const candle = (time, close) => ({
  time, open: close, high: close, low: close, close, volume: 1.0,
});

// Index the asset's history.
const history = Array.from({ length: 10 }, (_, i) => candle(i + 1, 100 + i + 1));
shazam.command(JSON.stringify({ cmd: "index", history }));

// Match the current state against the history.
const response = shazam.command(JSON.stringify({
  cmd: "match",
  current: [candle(11, 110.0)],
  k: 3,
}));

const report = JSON.parse(response);
console.log(report.matches.map((m) => m.ts));
```

## API

| Method | Description |
|--------|-------------|
| `new Shazam(specJson)` | Build a shazam from a spec JSON (throws if invalid). |
| `shazam.command(cmdJson) -> string` | Apply a command JSON, return the response JSON. Commands: `set_spec`, `index`, `label`, `match`, `reset`, `version`. |
| `shazam.version() -> string` | The library version. |
| `version() -> string` | Module-level version function. |

## Build from source

```sh
npm install
npm run build   # napi build --platform --release
npm test        # node --test
```

## License

`MIT OR Apache-2.0`.
