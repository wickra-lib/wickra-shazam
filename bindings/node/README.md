<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514-7" alt="Wickra Shazam — match an asset's current microstructure fingerprint against its entire history" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/ci.svg)](https://github.com/wickra-lib/wickra-shazam/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-shazam)
[![npm](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/npm.svg)](https://www.npmjs.com/package/wickra-shazam)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/license.svg)](https://github.com/wickra-lib/wickra-shazam#license)

# Wickra Shazam — Node.js

---

> **▶ Live demo:** all 514 indicators over real Binance market data, computed live in your browser — **[live.wickra.org](https://live.wickra.org)** · zero backend, powered by `wickra-wasm`.

**Point at live data → "that's the May-2021 crash setup". Match the current microstructure fingerprint of an asset against its entire history — for Node.js. `npm install wickra-shazam` — prebuilt native binary, no system dependencies.**

Node.js bindings for [wickra-shazam](https://github.com/wickra-lib/wickra-shazam),
the data-driven history-fingerprint match core, powered by Rust via napi-rs.
Build a `Shazam` from a spec JSON, index an asset's history, and match the
current fingerprint against it — the same command protocol every language
binding speaks.

## Install

```bash
npm install wickra-shazam
```

The native addon ships as a prebuilt binary per platform (Linux, macOS,
Windows — x64 and arm64), selected automatically through optional
dependencies. There is nothing to compile.

### Building from this repository (contributors)

```sh
npm install
npm run build   # napi build --platform --release
npm test        # node --test
```

## Quick start

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

### API

| Method | Description |
|--------|-------------|
| `new Shazam(specJson)` | Build a shazam from a spec JSON (throws if invalid). |
| `shazam.command(cmdJson) -> string` | Apply a command JSON, return the response JSON. Commands: `set_spec`, `index`, `label`, `match`, `reset`, `version`. |
| `shazam.version() -> string` | The library version. |
| `version() -> string` | Module-level version function. |

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of napi-rs, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-shazam/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-shazam>
- **Docs** (guides, spec reference, cookbook): <https://shazam.wickra.org>
- **Runnable example:** [`examples/node/`](https://github.com/wickra-lib/wickra-shazam/tree/main/examples/node)

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
