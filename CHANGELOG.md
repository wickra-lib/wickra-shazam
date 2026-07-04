# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `shazam-core`: the deterministic fingerprint engine — a serde `FingerprintSpec`
  (an ordered feature list of `indicator` / `price` / `microstructure` axes plus
  `window`, `normalize` and `metric`) folded over an asset's history into a rolling
  index of fixed-dimension fingerprints, with `match_current` finding the `k`
  nearest historical fingerprints under cosine, Euclidean or DTW similarity, and
  labels attaching human names to historical bars. Index and match produce a
  byte-identical report on the parallel (rayon) and sequential (WASM) paths.
- `wickra-shazam` CLI: index a history CSV and match the current state against it,
  with `--spec` / `--history` / `--current` / `--k` / `--label` and text or JSON
  output.
- Language bindings exposing the same JSON-over-C-ABI data API in ten languages —
  native Rust, Python (PyO3), Node.js (napi) and WASM (wasm-bindgen), plus a C ABI
  hub for C, C++, C#, Go, Java and R.
- Byte-exact golden corpus, conformance / parallel-equals-sequential / property
  tests, cargo-fuzz targets, criterion benchmarks, and one runnable example per
  language.
- CI across all ten languages on three OSes, CodeQL, OpenSSF Scorecard, zizmor
  workflow auditing, a tag-triggered release pipeline, and the `docs/` guides.
- Repository scaffolding: Cargo workspace, supply-chain configuration
  (`deny.toml`, `osv-scanner.toml`, `lychee.toml`), lint configuration
  (`clippy.toml`), `repo-metadata.toml`, and dual `MIT OR Apache-2.0` licensing.

[Unreleased]: https://github.com/wickra-lib/wickra-shazam/commits/main
