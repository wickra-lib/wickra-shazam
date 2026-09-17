<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514-7" alt="Wickra Shazam — match an asset's current microstructure fingerprint against its entire history" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/ci.svg)](https://github.com/wickra-lib/wickra-shazam/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-shazam)
[![npm](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/npm.svg)](https://www.npmjs.com/package/wickra-shazam-wasm)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/license.svg)](https://github.com/wickra-lib/wickra-shazam#license)

# Wickra Shazam — WASM

---

> **▶ Live demo:** all 514 indicators over real Binance market data, computed live in your browser — **[live.wickra.org](https://live.wickra.org)** · zero backend, powered by `wickra-wasm`.

**Point at live data → "that's the May-2021 crash setup". Match the current microstructure fingerprint of an asset against its entire history — for WASM. `npm install wickra-shazam-wasm` — pure WebAssembly, runs anywhere a modern JS engine does.**

WASM bindings for the `wickra-shazam` data-driven core, compiled to WebAssembly
with wasm-bindgen. Build a `Shazam` from a spec JSON, drive it with command JSON,
read back match reports — the same protocol as every other binding, running in
the browser.

The core is built with `--no-default-features`, so indexing and matching run
**sequentially** (no rayon thread pool in the browser sandbox) and are
byte-identical to the native parallel path.

## Install

```bash
npm install wickra-shazam-wasm
```

### Building from this repository (contributors)

```bash
wasm-pack build --target web
```

This emits `pkg/` with the `.wasm` module and JS glue.

## Quick start

```js
import init, { Shazam, version } from "wickra-shazam-wasm";

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

### API

| Member | Description |
|--------|-------------|
| `new Shazam(specJson)` | Build a shazam from a spec JSON (throws on an invalid spec). |
| `shazam.command(cmdJson)` | Apply a command JSON, return the response JSON. |
| `shazam.version()` / `version()` | The library version. |

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of wasm-bindgen, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-shazam/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-shazam>
- **Docs** (guides, spec reference, cookbook): <https://shazam.wickra.org>
- **Runnable example:** [`examples/wasm/`](https://github.com/wickra-lib/wickra-shazam/tree/main/examples/wasm)

Wickra Shazam ships native bindings for Python, Node.js, WASM and Rust, plus a C ABI hub that any
C-capable language (C, C++, C#, Go, Java, R) links against — all forwarding to the
same data-driven, `unsafe`-forbidden Rust core.

## Security

Found a security issue? **Please don't open a public issue.** Report it privately
via the repository's *Security* tab (*"Report a vulnerability"*) or email
**support@wickra.org** with a subject line starting `[wickra security]`. Full
policy: <https://github.com/wickra-lib/wickra-shazam/blob/main/SECURITY.md>.

## Disclaimer

Wickra Shazam is analysis software: it computes similarity between market states.
A historical match is a statistical resemblance, **not a prediction** and **not
financial advice** — the past setup did not have to repeat, and neither does this
one. It places no orders. Trading carries risk of loss; review the code and use
at your own discretion.

## License

Licensed under either of [Apache-2.0](https://github.com/wickra-lib/wickra-shazam/blob/main/LICENSE-APACHE)
or [MIT](https://github.com/wickra-lib/wickra-shazam/blob/main/LICENSE-MIT) at your option.
