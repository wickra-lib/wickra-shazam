<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Shazam — match an asset's current microstructure fingerprint against its entire history" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/ci.svg)](https://github.com/wickra-lib/wickra-shazam/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-shazam)
[![Go module](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/go.svg)](https://pkg.go.dev/github.com/wickra-lib/wickra-shazam-go)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/license.svg)](https://github.com/wickra-lib/wickra-shazam#license)

# Wickra Shazam — Go

---

> **▶ Live demo:** all 514 indicators over real Binance market data, computed live in your browser — **[live.wickra.org](https://live.wickra.org)** · zero backend, powered by `wickra-wasm`.

**Point at live data → "that's the May-2021 crash setup". Match the current microstructure fingerprint of an asset against its entire history — for Go. `go get github.com/wickra-lib/wickra-shazam-go` — over the C ABI via cgo, prebuilt library bundled in the module.**

[Wickra Shazam](https://github.com/wickra-lib/wickra-shazam) turns an asset's whole history into a rolling index of fixed-dimension microstructure fingerprints and matches the current fingerprint against it. This package is the Go binding: it consumes the C ABI hub through cgo and exposes the `Shazam` handle with the same JSON protocol as every other binding.

## Install

Use the published **`wickra-shazam-go`** module, which bundles the prebuilt C ABI
library for every platform, so `go get` + `go build` works with no extra steps
(a C compiler is still required, as the binding uses cgo):

```bash
go get github.com/wickra-lib/wickra-shazam-go
```

`wickra-shazam-go` is generated from this directory by the release pipeline: it mirrors
the Go sources, the vendored C ABI header (`include/wickra_shazam.h`) and the prebuilt
libraries under `lib/<goos>_<goarch>/`. On Linux/macOS the library path is baked
in via rpath; on Windows the DLL must be discoverable at run time (next to the
executable or on `PATH`).

### Building from this repository (contributors)

This `bindings/go` directory is the development source. To build it directly,
compile the C ABI hub and stage the library into the per-platform directory cgo
links against:

```bash
cargo build -p wickra-shazam-c --release
mkdir -p bindings/go/lib/linux_amd64                    # match your GOOS_GOARCH
cp target/release/libwickra_shazam.so    bindings/go/lib/linux_amd64/   # Linux
cp target/release/libwickra_shazam.dylib bindings/go/lib/darwin_arm64/  # macOS (arm64)
cp target/release/wickra_shazam.dll      bindings/go/lib/windows_amd64/ # Windows
```

Then, with the library on the loader path, run `go test ./...` from this directory.

## Quick start

```go
package main

import (
	"fmt"

	wickra "github.com/wickra-lib/wickra-shazam-go"
)

func main() {
	spec := `{"features":[{"kind":"price","field":"close"}],"window":1,"metric":"euclid"}`

	s, err := wickra.New(spec)
	if err != nil {
		panic(err)
	}
	defer s.Close()

	// Index the asset's history.
	s.Command(`{"cmd":"index","history":[` +
		`{"time":1,"open":101,"high":101,"low":101,"close":101,"volume":1},` +
		`{"time":2,"open":102,"high":102,"low":102,"close":102,"volume":1}]}`)

	// Match the current state against the history.
	report, err := s.Command(`{"cmd":"match","current":[` +
		`{"time":3,"open":102,"high":102,"low":102,"close":102,"volume":1}],"k":2}`)
	if err != nil {
		panic(err)
	}
	fmt.Println(report) // {"indexed":2,"matches":[{"similarity":...,"ts":2},...]}
	fmt.Println(wickra.Version())
}
```

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of cgo over the C ABI, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-shazam/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-shazam>
- **Docs** (guides, spec reference, cookbook): <https://shazam.wickra.org>
- **Runnable example:** [`examples/go/`](https://github.com/wickra-lib/wickra-shazam/tree/main/examples/go)

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
