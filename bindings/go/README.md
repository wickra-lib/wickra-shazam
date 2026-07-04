<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Shazam — match an asset's microstructure fingerprint against its history, for Go" width="100%"></a>
</p>

[![Built on Wickra](https://img.shields.io/badge/built%20on-wickra-3b82f6)](https://github.com/wickra-lib/wickra)
[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/ci.svg)](https://github.com/wickra-lib/wickra-shazam/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-shazam)
[![Go module](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/go.svg)](https://pkg.go.dev/github.com/wickra-lib/wickra-shazam-go)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/license.svg)](https://github.com/wickra-lib/wickra-shazam#license)

# Wickra Shazam — Go

---

**The deterministic market-fingerprint similarity-search core for Go, over the Wickra C ABI hub via cgo.**

[Wickra Shazam](https://github.com/wickra-lib/wickra-shazam) turns an asset's whole history into a rolling index of fixed-dimension microstructure fingerprints and matches the current fingerprint against it. This package is the Go binding: it consumes the C ABI hub through cgo and exposes the `Shazam` handle with the same JSON protocol as every other binding.

## Install

Use the published **`wickra-shazam-go`** module, which bundles the prebuilt C ABI library
for every platform, so `go get` + `go build` works with no extra steps (a C
compiler is still required, as the binding uses cgo):

```bash
go get github.com/wickra-lib/wickra-shazam-go
```

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


`wickra-shazam-go` is generated from this directory by the release pipeline: it mirrors the
Go sources, the vendored C ABI header (`include/wickra_shazam.h`) and the prebuilt
libraries under `lib/<goos>_<goarch>/`. On Windows the DLL must be discoverable at
run time (next to the executable or on `PATH`).

## Building from this repository (contributors)

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

## License

Dual-licensed under [MIT](https://github.com/wickra-lib/wickra-shazam/blob/main/LICENSE-MIT)
or [Apache-2.0](https://github.com/wickra-lib/wickra-shazam/blob/main/LICENSE-APACHE), at your option.
