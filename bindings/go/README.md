# Wickra Shazam — Go

Idiomatic Go bindings for the `wickra-shazam` data-driven core over its C ABI hub
(cgo). Build a `Shazam` from a spec JSON, drive it with command JSON, read back
match reports — the same protocol as every other binding.

## Install

```bash
go get github.com/wickra-lib/wickra-shazam-go
```

The binding links the prebuilt C ABI library, staged per platform under
`./lib/<goos>_<goarch>/`, with the header vendored under `./include`.

## Usage

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

## API

| Symbol | Description |
|--------|-------------|
| `New(specJSON string) (*Shazam, error)` | Build a shazam from a spec JSON. |
| `(*Shazam) Command(cmdJSON string) (string, error)` | Apply a command JSON, return the response JSON. |
| `(*Shazam) Close()` | Free the handle (idempotent; a finalizer also frees it). |
| `Version() string` | The library version. |

## License

`MIT OR Apache-2.0`.
