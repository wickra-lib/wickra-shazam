# Wickra Shazam — WASM

WASM bindings for the `wickra-shazam` data-driven core, compiled to WebAssembly
with wasm-bindgen. Build a `Shazam` from a spec JSON, drive it with command JSON,
read back match reports — the same protocol as every other binding, running in
the browser.

The core is built with `--no-default-features`, so indexing and matching run
**sequentially** (no rayon thread pool in the browser sandbox) and are
byte-identical to the native parallel path.

## Build

```bash
wasm-pack build --target web
```

This emits `pkg/` with the `.wasm` module and JS glue.

## Usage

```js
import init, { Shazam, version } from "./pkg/wickra_shazam_wasm.js";

await init();

const spec = JSON.stringify({
  features: [{ kind: "price", field: "close" }],
  window: 1,
  metric: "euclid",
});

const shazam = new Shazam(spec);

const candle = (time, close) => ({
  time, open: close, high: close, low: close, close, volume: 1.0,
});

const history = Array.from({ length: 10 }, (_, i) => candle(i + 1, 100 + i + 1));
shazam.command(JSON.stringify({ cmd: "index", history }));

const report = JSON.parse(shazam.command(JSON.stringify({
  cmd: "match",
  current: [candle(11, 110.0)],
  k: 3,
})));

console.log(report.matches.map((m) => m.ts));
console.log(version());
```

## API

| Member | Description |
|--------|-------------|
| `new Shazam(specJson)` | Build a shazam from a spec JSON (throws on an invalid spec). |
| `shazam.command(cmdJson)` | Apply a command JSON, return the response JSON. |
| `shazam.version()` / `version()` | The library version. |

## License

`MIT OR Apache-2.0`.
