# Wickra Shazam examples

A runnable "index a history and match the current state" example in every
language. Each one builds a shazam from the same price spec, indexes a short
inline history (close 100, 101, 102) and matches a current bar (close 102) — so
the most recent bar (`ts 3`) is a perfect self-match. The per-language examples
are self-contained: the spec and candles are inline, so there is no shared data
to load (the golden fixtures live in [`../golden/`](../golden)).

## Rust — `examples/rust/`

As the CI examples job runs it, from the repository root:

```bash
cargo run -q --manifest-path examples/rust/Cargo.toml
```

| Example | What it does |
| --- | --- |
| `src/main.rs` | A runnable Rust example: index a short history with the native `build_index` API and match the current state. |

## C / C++ — `examples/c/`

Build the library first (`cargo build -p wickra-shazam-c --release`), then build and run
the examples via CMake, as the CI C ABI job does:

```bash
cmake -S examples/c -B examples/c/build
cmake --build examples/c/build --config Release
ctest --test-dir examples/c/build -C Release --output-on-failure
```

| Example | What it does |
| --- | --- |
| `match.c` | A minimal C example: index a history and match the current state through the |
| `match.cpp` | A minimal C++ example: index a history, label a window, match the current state -- and show that the label lands the same whether it was sent before or after the index was built -- all through the C++ |

## C# — `examples/csharp/`

As the CI examples job runs it, from the repository root:

```bash
dotnet run --project examples/csharp/Match
```

| Example | What it does |
| --- | --- |
| `Match/Program.cs` | A runnable .NET example: index a history and match the current state. |

## Go — `examples/go/`

As the CI examples job runs it, from the repository root:

```bash
cd examples/go && go run .
```

| Example | What it does |
| --- | --- |
| `match.go` | A runnable Go example: index a history and match the current state. |

## R — `examples/r/`

As the CI examples job runs it, from the repository root:

```bash
R CMD INSTALL bindings/r
Rscript examples/r/match.R
```

| Example | What it does |
| --- | --- |
| `match.R` | A runnable R example: index a history and match the current state. |

## Java — `examples/java/`

As the CI examples job runs it, from the repository root:

```bash
mvn -f bindings/java/pom.xml -q package -DskipTests
javac -cp bindings/java/target/classes examples/java/Match.java -d examples/java/out
java --enable-native-access=ALL-UNNAMED  -Dnative.lib.dir="$PWD/target/release"  -cp "bindings/java/target/classes:examples/java/out" Match
```

| Example | What it does |
| --- | --- |
| `Match.java` | A runnable Java example: index a history and match the current state. |

## Python — `examples/python/`

As the CI examples job runs it, from the repository root:

```bash
python -m pip install --require-hashes -r .github/requirements/ci-dev-py3.txt
( cd bindings/python && maturin build --release --out dist )
python -m pip install --no-index --find-links bindings/python/dist wickra-shazam
python examples/python/match.py
```

| Example | What it does |
| --- | --- |
| `match.py` | A runnable Python example: index a history and match the current state. |

## Node.js — `examples/node/`

As the CI examples job runs it, from the repository root:

```bash
( cd bindings/node && npm install --no-audit --no-fund && npx napi build --platform --release )
( cd examples/node && npm install --no-audit --no-fund )
node examples/node/match.js
```

| Example | What it does |
| --- | --- |
| `match.js` | A runnable Node.js example: index a history and match the current state. |

## WASM — `examples/wasm/`

Build the WASM package, serve the repository root, and open the page in a browser;
the module script inside it is what runs (CI parses it with `node --check`):

```bash
wasm-pack build bindings/wasm --target web
python -m http.server 8000     # then open http://localhost:8000/examples/wasm/
```

| Example | What it does |
| --- | --- |
| `match.html` | A runnable example against this binding. |

## Example datasets

The examples read from [`examples/data/`](data/): . The
cross-language golden fixtures, which every binding is checked against byte for
byte, live in [`../golden/`](../golden).

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
