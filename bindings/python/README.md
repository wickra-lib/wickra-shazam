<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Shazam — match an asset's current microstructure fingerprint against its entire history" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/ci.svg)](https://github.com/wickra-lib/wickra-shazam/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-shazam)
[![PyPI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/pypi.svg)](https://pypi.org/project/wickra-shazam/)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/license.svg)](https://github.com/wickra-lib/wickra-shazam#license)

# Wickra Shazam — Python

---

> **▶ Live demo:** all 514 indicators over real Binance market data, computed live in your browser — **[live.wickra.org](https://live.wickra.org)** · zero backend, powered by `wickra-wasm`.

**Point at live data → "that's the May-2021 crash setup". Match the current microstructure fingerprint of an asset against its entire history — for Python. `pip install wickra-shazam` — prebuilt wheels for Linux, macOS and Windows, nothing to compile.**

Python bindings for [wickra-shazam](https://github.com/wickra-lib/wickra-shazam),
the data-driven history-fingerprint match core. Build a `Shazam` from a spec
JSON, index an asset's history, and match the current fingerprint against it —
the same command protocol every language binding speaks.

## Install

```bash
pip install wickra-shazam
```

Pre-built wheels ship for Linux, macOS and Windows — there is nothing to
compile and no C library to track down.

### Building from this repository (contributors)

```sh
maturin develop --release
pytest -q
```

## Quick start

```python
import json
from wickra_shazam import Shazam

spec = json.dumps({
    "features": [{"kind": "price", "field": "close"}],
    "window": 1,
    "metric": "euclid",
})

shazam = Shazam(spec)

def candle(time, close):
    return {"time": time, "open": close, "high": close,
            "low": close, "close": close, "volume": 1.0}

# Index the asset's history.
history = [candle(t, 100.0 + t) for t in range(1, 11)]
shazam.command(json.dumps({"cmd": "index", "history": history}))

# Match the current state against the history.
response = shazam.command(json.dumps({
    "cmd": "match",
    "current": [candle(11, 105.0)],
    "k": 3,
}))

report = json.loads(response)
print([m["ts"] for m in report["matches"]])
```

### API

| Method | Description |
|--------|-------------|
| `Shazam(spec_json)` | Build a shazam from a spec JSON (raises `ValueError` if invalid). |
| `shazam.command(cmd_json) -> str` | Apply a command JSON, return the response JSON. Commands: `set_spec`, `index`, `label`, `match`, `reset`, `version`. |
| `Shazam.version() -> str` | The library version. |

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of PyO3, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-shazam/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-shazam>
- **Docs** (guides, spec reference, cookbook): <https://shazam.wickra.org>
- **Runnable example:** [`examples/python/`](https://github.com/wickra-lib/wickra-shazam/tree/main/examples/python)

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
