# Examples

A runnable "index a history and match the current state" example in every
language. Each one builds a shazam from the same price spec, indexes a short
inline history (close 100, 101, 102) and matches a current bar (close 102) — so
the most recent bar (`ts 3`) is a perfect self-match. The per-language examples
are self-contained: the spec and candles are inline, so there is no shared data
to load (the golden fixtures live in [`../golden/`](../golden)).

| Language | Path | Run |
|----------|------|-----|
| Rust | [`rust/`](rust/) | `cargo run -p wickra-shazam-example` |
| Python | [`python/match.py`](python/match.py) | `pip install wickra-shazam && python examples/python/match.py` |
| Node.js | [`node/`](node/) | `cd examples/node && npm install && node match.js` |
| C / C++ | [`c/`](c/) | see below |
| Go | [`go/`](go/) | `cd examples/go && go run .` |
| .NET | [`csharp/Match/`](csharp/Match/) | `dotnet run --project examples/csharp/Match` |
| Java | [`java/Match.java`](java/Match.java) | see the header comment |
| R | [`r/match.R`](r/match.R) | `Rscript examples/r/match.R` |

The native bindings (Python, Node.js) load their own compiled library. The
bindings that go through the C ABI (Go, .NET, Java, R, and the C/C++ example
itself) need the C ABI library built first:

```bash
cargo build --release -p wickra-shazam-c
```

## C / C++

The C and C++ examples build with CMake and run under ctest:

```bash
cargo build --release -p wickra-shazam-c
cmake -S examples/c -B examples/c/build
cmake --build examples/c/build --config Release
ctest --test-dir examples/c/build -C Release --output-on-failure
```

On Windows the build copies `wickra_shazam.dll` next to each executable, since
there is no rpath.

## CLI over a data file

The [`data/`](data/) directory holds a single-symbol OHLCV history
(`BTCUSDT.csv`), a derived current-state window, and two specs. Point the CLI at
them to index a real-ish history and print the matches:

```bash
cargo run -p wickra-shazam -- \
  --spec examples/data/specs/crash_setup.json \
  --history examples/data/history/BTCUSDT.csv \
  --current examples/data/current/BTCUSDT.csv \
  --k 5 --format json
```

Swap in `examples/data/specs/price_euclid.json` for a plain close-price match, or
drop `--format json` for an aligned `rank | ts | similarity | label` table.

## Expected output

Every per-language example prints the version and the match report, for example:

```text
wickra-shazam 0.1.0
{"matches":[{"ts":3,"similarity":1.0},{"ts":2,"similarity":0.5}],"indexed":3}
```
