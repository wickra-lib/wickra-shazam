# Wickra Shazam examples — R

Runnable R examples for the [Wickra Shazam R binding](../../bindings/r). The package compiles a thin
`.Call` glue layer against the C ABI library, so build the library and install
the package first (the CI examples job does exactly this):

```bash
cargo build -p wickra-shazam-c --release
R CMD INSTALL bindings/r
```

## Run

```bash
Rscript examples/r/match.R
```

## The examples

| Example | What it does |
|---------|--------------|
| `match.R` | A runnable R example: index a history and match the current state. |
