# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.3] - 2026-09-24

A follow-up release: the fingerprint matcher and its bindings are unchanged. It
pins wickra-exchange 0.1.8, the release that makes exchange's R package build on
r-universe's WebAssembly target and its release pipeline re-runnable.

### Changed

- **Built on wickra-exchange 0.1.8.** The exact pin on `wickra-exchange` moves
  from =0.1.7 to =0.1.8, and every tracked lockfile follows. Nothing in
  exchange's Rust API changed between the two; 0.1.8 fixes its R package's
  WebAssembly build and its Maven Central step.

## [0.1.2] - 2026-09-23

A maintenance release: the fingerprint matcher and its bindings are unchanged.
It publishes the refreshed dependency tree and toolchain pins.

### Added

- **The Node binding reports which artifact it loaded.** The loader generated
  by `@napi-rs/cli` 3.10.4 exports `__napiBindingTarget` -- `'native'` for the
  native addon, otherwise the WASI flavor it resolved -- and follows a
  `NAPI_RS_NATIVE_LIBRARY_PATH` override to a WASI loader instead of
  misreporting it as native. Typed in `index.d.ts`.

### Changed

- **Built on wickra-core 1.0.6.** The lock takes the indicator core's latest
  release; the `1.0` requirement already admitted it.
- **The family pins follow the owners' releases.** `wickra-backtest-core` =0.1.7
  -> =0.1.8, `wickra-exchange` =0.1.6 -> =0.1.7 -- the exact pins this
  repository keeps on its siblings move to the versions those repositories
  release in the same train, and every tracked lockfile follows.
- **Third-party dependencies refreshed.** `Cargo.lock` takes 38 crates to their
  newest versions compatible with the Rust floor (this repository resolves
  MSRV-aware), run across the family in one pass so every repository resolves
  the same day's versions. The refresh itself changes no manifest.
- **`@napi-rs/cli` 3.10.4** for the Node binding, the family's line.
- **uv 0.12.18** for the lockfile bootstrap in `scripts/update-lockfiles.sh`,
  with all four platform checksums moved together.
- **The README's static badges are served by the organization** rather than
  hot-linked from shields.io, so they no longer break when shields is down.

## [0.1.1] - 2026-09-18

### Fixed

- **The Java binding loads the library it ships.** The jar carries the native
  library under `native/<os>-<arch>/` -- the release pipeline stages every
  platform there -- but the loader only ever looked at `-Dnative.lib.dir` and
  the working directory, so a Maven Central consumer got a jar it could not
  load without pointing the JVM at a library it had to build itself. The loader
  now resolves in wickra's order: `-Dnative.lib.dir` when set, the bundled copy
  extracted to a temporary file, every `target/release` or `target/debug` up
  the tree from the working directory and the class's own location, then the
  bare name.

### Changed

- **Family pins follow the owners' releases:** wickra-backtest-core =0.1.6 -> =0.1.7, wickra-exchange =0.1.5 -> =0.1.6. No code of this repository changes; the engine it links is the one those releases ship.
- **Every README follows wickra's shape.** A cross-repo scan compared the
  heading skeleton of each README against wickra's and this repository's
  differed throughout. The root README opens as wickra's does (banner, badges,
  the one-liner, the live-demo and ecosystem lines, no separate H1), the
  License section carries wickra's wording and its `### Contribution` clause,
  and the shared sections run in wickra's order. Each binding README is
  `Install`, `Quick start`, `Benchmark`, `Documentation`, `Security`,
  `Disclaimer`, `License` with the product's own surface and protocol notes
  as subsections; the registry pages that render them now say how to report a
  vulnerability and under which licence the package ships.
  `examples/README.md` lists every language the way wickra's does, with the
  commands the CI examples job runs; the per-language example READMEs,
  `fuzz/README.md` and the `## Editing the docs` section of
  `docs/README.md` exist as they do in wickra.

### Changed

- **wickra-backtest-core 0.1.6 and wickra-exchange 0.1.5.** The pins move to the releases the family is on; the lock follows.
  A cross-repo scan lined the 24 wickra-lib repositories up, and the rest is
  what this one spelled differently: the fuzz job runs the family's pinned
  `nightly-2026-07-01` rather than a rolling nightly, and the example job's
  `dotnet-version` reads `8.0.x`.

### Changed

- **wickra-backtest-core 0.1.6 and wickra-exchange 0.1.5.** The pins move to the releases the family is on; the lock follows.
  A cross-repo scan lined the 24 wickra-lib repositories up, and the rest is
  what this one spelled differently: the fuzz job runs the family's pinned
  `nightly-2026-07-01` rather than a rolling nightly, and the example job's
  `dotnet-version` reads `8.0.x`.

### Changed

- **uv 0.12.15 for the lockfile script.** `scripts/update-lockfiles.sh`
  bootstraps 0.12.15 (was 0.12.13); the pin and all four release
  checksums move together, taken from the release's `.sha256` files.

## [0.1.0] - 2026-09-14

### Security

- **rustls 0.23.45.** RUSTSEC-2026-0285: rustls accepted TLS 1.3 handshake
  messages sent at the wrong encryption level. The lock moves to the
  patched release; nothing in the code changes.

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

[Unreleased]: https://github.com/wickra-lib/wickra-shazam/compare/v0.1.3...HEAD
[0.1.3]: https://github.com/wickra-lib/wickra-shazam/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/wickra-lib/wickra-shazam/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/wickra-lib/wickra-shazam/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/wickra-lib/wickra-shazam/releases/tag/v0.1.0
