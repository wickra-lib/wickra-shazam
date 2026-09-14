# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-09-14

### Fixed

- **A mutating command through the C ABI executed twice.** `wickra_shazam_command`
  ran the command on every call, and every consumer of the length-then-fill
  protocol -- Go, C#, Java, R and the C examples -- calls it twice (once for the
  length, once for the bytes), so `set_spec`, `index`, `label` and `reset` ran
  twice through four of the ten bindings. The handle now caches the response it
  has computed but not yet delivered and reuses it for a repeated call with the
  same command bytes, so a logical command runs exactly once however many
  buffer-sizing retries it takes (the contract gym already documents). A C ABI
  test pins it.
- **Operating-mode equivalence is tested in the core and in every binding.**
  A `label` sent before `index` is kept on the handle and applied when the
  index is built; sent after `index` it goes onto the live index. Both must
  yield the same `match` report, and re-indexing keeps it; `operating_modes.rs`
  pins it in Rust over the golden corpus and each of the nine bindings checks
  it at its own boundary. The C suite
  (`examples/c/golden_test.c`, wired into ctest with a CMake-globbed spec list)
  checks golden parity and both modes without a JSON library. The golden tests
  fail on a missing corpus instead of skipping.
- **A C++ hull.** `bindings/c/include/wickra_shazam.hpp` -- header-only
  C++17, owns the handle, runs the length protocol, turns a negative return
  into an exception -- ships beside the C header and in the release archive;
  `examples/c/match.cpp` builds against it.
- **Every dependency comes from crates.io, and the sibling pins are exact.**
  `wickra-exchange` and `wickra-backtest-core` were git dependencies, which
  `cargo publish` refuses; they are the registry crates at `=0.1.4` and
  `=0.1.5`, as the released siblings pin them, and `wickra-core` and
  `wickra-data` follow their 1.0 lines. `deny.toml` no longer allows git
  sources.
- **The release front in the family shape.** The tag guard, the version gate,
  idempotent publishing of both crates (`wickra-shazam-core`, `wickra-shazam`)
  with CycloneDX SBOMs, the C ABI
  archives with the header and the hull, a Maven Central deploy that skips a
  version already on Central and waits as long as Central takes, the jar
  uploaded for the provenance job to attest, provenance over the nupkg, jar and
  C ABI archives, the Go mirror that builds before it pushes, and a
  `workflow_dispatch` that publishes nothing. The pom carries the release
  profile (sources, javadoc, GPG, the publishing plugin), `<scm>` and
  `<developers>` Central requires.
- **The Python 3.9 CI row runs without pytest.** pytest 9.x requires 3.10, so
  that row could only pin 8.4.2, below the fix for GHSA-6w46-j5rx-g56g with no
  backport. The 3.9 lock carries maturin only, and the row runs the same test
  modules through `bindings/python/tests/run_without_pytest.py`; 3.10 and up
  run them under pytest as before.
- **The R package builds the family way.** `configure` downloads the C ABI
  release archive (or builds it from the tag's source on r-universe's
  WebAssembly image), `configure.win` picks the architecture from
  `R.version$arch`, the exported functions carry generated `man/` pages, a
  shipped smoke test runs inside the tarball, `.Rbuildignore` is regex-safe,
  and `DESCRIPTION` states the R floor.
- `dtw_similarity` enumerates the band's window instead of ranging over it
  (clippy `needless_range_loop` on the current toolchain); same cells, same
  order, same bytes.
- CI in the family shape: the binding-surface, links and semver jobs, the
  wheel container smoke, an Examples job that holds every example to the
  version line, a WASM demo page (`examples/wasm/match.html`) whose module
  is parse-checked, CodeQL over C#, Java and C/C++ with a config that keeps
  generated code out, osv-scanner, timeouts on every job, patch-level pins,
  Dependabot over every manifest (the fuzz crate and the Go and Node examples
  included), the five repository-check scripts, `update-lockfiles.sh`, the
  detailed issue and PR templates, actionlint, CodSpeed with `criterion`
  aliased to it, zizmor's `self-repository` policy, and docs.rs metadata on
  both crates.
- Licence texts travel with every published package (`LICENSES/`, copies in
  each crate and the Python and npm packages); the README opens with the
  quickstart and states the toolchain floors the manifests declare;
  `SECURITY.md` names the first release.
- **The core crate carried a name the release could not upload.**
  `shazam-core` is outside the org's crates.io token scope, which
  creates new crates under the `wickra-` prefix only; `cargo publish` on it
  returns 403 at upload while `--dry-run` passes, and because the publish jobs
  run in parallel the release would have landed on PyPI, npm, NuGet, Maven
  Central and the Go mirror without ever reaching crates.io. The core is now
  `wickra-shazam-core`, the shape of every released sibling. The
  directory keeps its name; only the package and the
  `wickra_shazam_core` path moved. The same audit ran across the family
  (xray paid for this with its first tag).

### Added

- `wickra-shazam-core`: the deterministic fingerprint engine — a serde `FingerprintSpec`
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

[Unreleased]: https://github.com/wickra-lib/wickra-shazam/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/wickra-lib/wickra-shazam/releases/tag/v0.1.0
