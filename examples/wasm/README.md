# wickra-shazam WASM examples

Browser demos for the `wickra-shazam-wasm` binding.

The WASM build carries the whole matching core: the same fingerprint
features, the same metrics and the same report bytes the CLI and the other
nine bindings produce. A spec is data, not code, so the bytes on this page are
the same ones `examples/node/match.js` sends, and the matches are the same
matches.

## Build

The module ships as a `wasm-pack` `--target web` bundle. Build it once from the
repository root:

```bash
wasm-pack build bindings/wasm --target web --release
```

That writes `bindings/wasm/pkg/` with the `.wasm` binary, the JS loader and the
type declarations the page imports.

## Run

The page loads its module over `http://`, not `file://`, because ES module
imports and `WebAssembly.instantiateStreaming` both need a real origin. Serve the
repository root:

```bash
python -m http.server 8000
```

Then open `http://localhost:8000/examples/wasm/match.html`.

## Pages

| Page | What it does |
|------|--------------|
| `match.html` | Indexes a short history, labels the window the current state resembles and matches it, showing that the label lands the same whether it was sent before or after the index was built. The page counterpart of `examples/node/match.js`. |

## See also

- [examples/README.md](../README.md) — the same match in every other language.
- [bindings/wasm/README.md](../../bindings/wasm/README.md) — the binding itself.
