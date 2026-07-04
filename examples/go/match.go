// A runnable Go example: index a history and match the current state.
//
//	cargo build --release -p wickra-shazam-c
//	# stage the library under bindings/go/lib/<goos>_<goarch>/ (CI does this)
//	cd examples/go && go run .
package main

import (
	"fmt"

	wickra "github.com/wickra-lib/wickra-shazam-go"
)

const spec = `{"features":[{"kind":"price","field":"close"}],` +
	`"window":1,"metric":"euclid"}`

const indexCmd = `{"cmd":"index","history":[` +
	`{"time":1,"open":100,"high":100,"low":100,"close":100,"volume":1},` +
	`{"time":2,"open":101,"high":101,"low":101,"close":101,"volume":1},` +
	`{"time":3,"open":102,"high":102,"low":102,"close":102,"volume":1}]}`

const matchCmd = `{"cmd":"match","current":[` +
	`{"time":4,"open":102,"high":102,"low":102,"close":102,"volume":1}],"k":2}`

func main() {
	shazam, err := wickra.New(spec)
	if err != nil {
		panic(err)
	}
	defer shazam.Close()

	if _, err := shazam.Command(indexCmd); err != nil {
		panic(err)
	}
	report, err := shazam.Command(matchCmd)
	if err != nil {
		panic(err)
	}

	fmt.Println("wickra-shazam", wickra.Version())
	fmt.Println(report)
}
