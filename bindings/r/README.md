# Wickra Shazam — R

R bindings for the `wickra-shazam` data-driven core, over its C ABI hub
(`.Call`). Build a shazam from a spec JSON, index an asset's history, match the
current fingerprint against it — the same protocol as the CLI and every other
binding.

## Usage

```r
library(wickrashazam)

spec <- paste0(
  '{"features":[{"kind":"price","field":"close"}],',
  '"window":1,"metric":"euclid"}'
)

shazam <- wkshzm_new(spec)

candle <- function(time, close) {
  paste0(
    '{"time":', time, ',"open":', close, ',"high":', close,
    ',"low":', close, ',"close":', close, ',"volume":1}'
  )
}

# Index the asset's history.
history <- paste0("[", paste(
  vapply(1:10, function(i) candle(i, 100 + i), character(1)), collapse = ","
), "]")
wkshzm_command(shazam, paste0('{"cmd":"index","history":', history, '}'))

# Match the current state against the history.
raw <- wkshzm_command(
  shazam, paste0('{"cmd":"match","current":[', candle(11, 110), '],"k":3}')
)
cat(raw, "\n")
cat(wkshzm_version(), "\n")
```

## Build and test from source

The package links the `wickra_shazam` C ABI, located out-of-tree via two
environment variables:

```bash
# Build the C ABI shared library first.
cargo build -p wickra-shazam-c --release

export WKSHZM_INC="$PWD/bindings/c/include"
export WKSHZM_LIB="$PWD/target/release"
# The loader must also find the shared library at run time:
export LD_LIBRARY_PATH="$WKSHZM_LIB:$LD_LIBRARY_PATH"   # PATH on Windows

R CMD INSTALL bindings/r
Rscript bindings/r/tests/run_tests.R
```

## API

| Function | Description |
|----------|-------------|
| `wkshzm_new(spec_json)` | Build a shazam from a spec JSON (errors on an invalid spec). |
| `wkshzm_command(shazam, cmd_json)` | Apply a command JSON, return the response JSON. |
| `wkshzm_version()` | The library version. |

## License

`MIT OR Apache-2.0`.
