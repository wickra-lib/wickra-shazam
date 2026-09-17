<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514-7" alt="Wickra Shazam — match an asset's current microstructure fingerprint against its entire history" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/ci.svg)](https://github.com/wickra-lib/wickra-shazam/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-shazam)
[![NuGet](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/nuget.svg)](https://www.nuget.org/packages/Wickra.Shazam)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/license.svg)](https://github.com/wickra-lib/wickra-shazam#license)

# Wickra Shazam — C#

---

> **▶ Live demo:** all 514 indicators over real Binance market data, computed live in your browser — **[live.wickra.org](https://live.wickra.org)** · zero backend, powered by `wickra-wasm`.

**Point at live data → "that's the May-2021 crash setup". Match the current microstructure fingerprint of an asset against its entire history — for C#. `dotnet add package Wickra.Shazam` — prebuilt native library, no system dependencies.**

.NET bindings for [`wickra-shazam`](https://github.com/wickra-lib/wickra-shazam) over
the C ABI hub, via source-generated P/Invoke. Build a `Shazam` from a spec JSON,
index a history, label the windows you recognise, and match the current state
— the same protocol the CLI and every other binding speak, returning the same
bytes.

## Install

```bash
dotnet add package Wickra.Shazam
```

The native library ships prebuilt per platform under `runtimes/<rid>/native/`,
selected automatically. There is nothing to compile. Targets .NET 8 and later.

Requires .NET 8+. The native library (`wickra_shazam`) must be resolvable on the
loader path — `PATH` on Windows, `LD_LIBRARY_PATH` on Linux, `DYLD_LIBRARY_PATH`
on macOS — or beside the assembly, where the bundled resolver finds it.

## Quick start

```csharp
using Wickra.Shazam;

const string spec = """
{"features":[{"kind":"indicator","name":"Rsi","params":[14]},
             {"kind":"price","field":"close"}],
 "window":10,"normalize":"z_score","metric":"cosine"}
""";

using var shazam = new Shazam(spec);
shazam.Command("""{"cmd":"index","history":[ … ]}""");
shazam.Command("""{"cmd":"label","ts":1700226800,"label":"may_2021_crash"}""");
string report = shazam.Command("""{"cmd":"match","current":[ … ],"k":5}""");
```

A `label` sent before `index` is kept on the handle and applied when the index
is built; sent after `index` it goes onto the live index. Both yield the same
`match` report, and re-indexing the same history keeps it. The core pins this
in Rust and the binding's test suite checks it across the committed golden
corpus.

Every `Command` call is executed exactly once. The C ABI's length-then-fill
protocol needs two calls when the first buffer is too small, and the hub caches
the response of a mutating command (`index`, `label`, `set_spec`, `reset`)
until it has been delivered, so a retry never indexes twice.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of `[LibraryImport]` P/Invoke over the C ABI, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-shazam/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-shazam>
- **Docs** (guides, spec reference, cookbook): <https://shazam.wickra.org>
- **Runnable example:** [`examples/csharp/`](https://github.com/wickra-lib/wickra-shazam/tree/main/examples/csharp)

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
