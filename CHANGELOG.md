# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.3] - 2026-11-27

A release of the build and release pipeline; the library code is unchanged.

### Changed

- **The Maven sources jar no longer carries the native libraries.** The release
  workflow stages every platform's library into the resources before the
  deploy, and the sources jar packed them a second time. A release's size on
  Maven Central roughly halves (measured on wickra-impact 0.2.2: the sources jar
  drops from 3.6 MB to 7 KB, the jar is unchanged).
- **The GitHub Release no longer waits on Maven Central.** Central can refuse a
  release over its monthly publishing limits after every other registry has
  published it, and the release page then never appeared. It now does, and the
  jar is attested in a job of its own once Central has taken it.
- **PyPI uploads go through PyPA's publishing action** instead of
  `maturin upload`, which maturin has deprecated. maturin still builds every
  wheel; token, files and the skip of versions already on PyPI are unchanged.
- **CI retries Maven Central's throttling** (429 and 5xx, eight times with a
  back-off of up to five minutes), and the link check skips r-universe's pages,
  which answer every non-browser client with a Cloudflare challenge.
- **Draft pull requests start no CI.** Every job of the workflows that run
  on pull requests skips a draft, and marking it ready for review starts
  the full run.
- **CI caches the Maven repository with restore-keys.** setup-java's own
  cache keyed on the pom alone, so every version bump started empty and
  fetched every plugin from Maven Central again; in a release train that ran
  into Central's 429 throttling. A changed pom now restores the previous
  cache and downloads only what is new.

## [0.2.2] - 2026-11-26

A maintenance release: it follows wickra-core 2.2.0, builds on Rust 1.100 and
Go 1.27, and ships the refreshed dependency tree.

### Changed

- **Built on wickra-core 2.2.0** (from 2.1.0). The lock takes the indicator
  core's release; the caret requirement already admitted it. Saturated `Rsi` and
  `StochRsi` values are now exactly 100 (and 0), and `TtmSqueeze` detrends its
  momentum per bar, as the reference does, with a warmup of `2 * period - 1`;
  anything here that runs those indicators can report different values with it.
- **The family pins follow the owners' releases.** `wickra-backtest-core` =0.2.1
  -> =0.2.2, `wickra-exchange` =0.2.1 -> =0.2.2 -- the exact pins this
  repository keeps on its siblings move to the versions those repositories
  release in the same train, and every tracked lockfile follows.
- **Third-party dependencies refreshed.** `Cargo.lock` takes 104 crates at their
  latest releases within the declared ranges. The npm and Python dev locks are
  re-resolved the same way. The refresh itself changes no manifest.
- **Dependabot's monthly updates** (GitHub Actions, Maven, npm): 4 pull
  requests, each checked against the versions the rest of the family holds.
- **CI and release workflow:** point osv-scanner at the Python lockfiles it was
  silently skipping; install the Node binding with npm ci now that the lock
  names every platform package; use setup-java's *-env-var inputs.

### Fixed

- **Builds on Rust 1.100.** Its `clippy::used_underscore_items` fires on the
  code `#[pymodule]` generates for an underscore-named module function. The
  function has a plain name now and `#[pyo3(name = "_...")]` keeps the extension
  module's import name unchanged.
- **The Go binding links on Windows with Go 1.27,** which rejects
  `-l:<name>.dll` in `#cgo LDFLAGS` as an invalid flag. It is `-l<name>` now;
  the MinGW linker resolves that to the same DLL.

### Security

- **url 2.5.9** in the lockfile, for RUSTSEC-2026-0592: a stack overflow in
  `Url::origin()` on nested `blob:` URLs.
- **rustix 1.1.6** in the lockfile, for RUSTSEC-2026-0486: xattr functions
  marked unwritten bytes as initialised when given a zero-length buffer.
- **quinn-proto 0.11.19** in the lockfile, for RUSTSEC-2026-0523 and
  RUSTSEC-2026-0524: a `MAX_STREAM_DATA` frame could open streams beyond the
  limit, and a long Retry token could panic or exhaust memory in a QUIC client.
- **quinn-udp 0.5.16** in the lockfile, for RUSTSEC-2026-0538: the receive loop
  could spin after a truncated datagram.

## [0.2.1] - 2026-10-28

A maintenance release: the pattern matcher and its bindings are unchanged. It
ships the security fixes in the dependency tree.

### Changed

- **Built on wickra-core 2.1.0.** The lock takes the indicator core's minor
  release; the `2.0` requirement already admitted it. It adds `RunningDrawdown`
  and updates `MaxDrawdown` in amortised O(1) with the same values; nothing here
  computes differently.
- **The family pins follow the owners' releases.** `wickra-backtest-core` =0.2.0
  -> =0.2.1, `wickra-exchange` =0.2.0 -> =0.2.1 -- the exact pins this
  repository keeps on its siblings move to the versions those repositories
  release in the same train, and every tracked lockfile follows.
- **MSRV 1.87** (was 1.86) for the workspace; the Node binding stays at 1.88.
  wickra-exchange reaches `crypto-bigint` and `ctutils` through `p256`, and the
  releases that fix RUSTSEC-2026-0387 and RUSTSEC-2026-0385 require Rust 1.87.
  Every earlier crypto-bigint 0.7 release is yanked, so a fresh resolve could no
  longer build on 1.86.
- **napi 3.15.0 with napi-derive 3.6.13** for the Node binding, moved together
  with napi-derive-backend 6.1.6, napi-build 2.6.0 and napi-sys 3.4.1; the pair
  Dependabot proposed on its own did not compile. `@napi-rs/cli` 3.10.8 builds
  it. All five keep the binding's Rust 1.88 floor.
- **pyo3 0.30.1** for the Python binding, and serde_json 1.0.154. pyo3 0.30
  drops Python 3.8, which the wheels never supported: they still need Python 3.9
  or later.
- **central-publishing-maven-plugin 0.12.0** publishes the Java binding.
- **uv 0.13.5** for the lockfile bootstrap in `scripts/update-lockfiles.sh`.
- **The documentation quotes the indicator count this repository reaches:** the
  495 indicators the backtest registry resolves -- bar builders, histogram
  profiles and a few struct outputs are out of its scope -- rather than the
  core's 514 or a stale 497. The counts are now set by the family's count
  engine on every release bump.

### Security

- **crypto-bigint 0.7.9, ctutils 0.4.6 and tokio 1.53.3** in the lockfile, for
  RUSTSEC-2026-0387, RUSTSEC-2026-0385 and RUSTSEC-2026-0386.
- **rustls 0.23.46** in the lockfile, for RUSTSEC-2026-0346 to RUSTSEC-2026-0349:
  TLS secrets in the `Debug` output of client session values, application data
  sent to the outer server after an ECH rejection, Brotli certificate
  decompression allocating before authentication, and a ServerHello downgrade to
  classical key exchange.
- **rustls-webpki 0.103.16 and webpki-roots 1.0.10** in the lockfile, for
  RUSTSEC-2026-0382 and RUSTSEC-2026-0381: certificates chaining through the
  distrusted emSign intermediates are refused.

## [0.2.0] - 2026-10-06

A follow-up release on the wickra 2.0 family.

### Changed

- **Built on wickra 2.0, wickra-backtest 0.2.0 and wickra-exchange 0.2.0.**
  `wickra-core` and `wickra-data` move from 1.0 to 2.0, the formula-audit
  release of the indicator core; the exact pin on `wickra-backtest-core` moves
  from =0.1.9 to =0.2.0; the exact pin on `wickra-exchange` moves from =0.1.8
  to =0.2.0; every tracked lockfile follows. Indicators the audit corrected
  return the values of their published definitions; wickra's changelog lists
  them, with the warmup changes and the new defaults.

## [0.1.4] - 2026-09-27

A follow-up release: the fingerprint matcher and its bindings are unchanged. It
pins wickra-backtest 0.1.9, whose strategy-spec schema is now generated by
schemars 1.x -- the same schema, still draft 07 -- so schemars 0.8 leaves this
repository's dependency tree.

### Changed

- **Built on wickra-backtest 0.1.9.** The exact pin on `wickra-backtest-core`
  moves from =0.1.8 to =0.1.9, and every tracked lockfile follows. Besides the
  family crates the locks move `ref-cast` 1.0.27 (new), `ref-cast-impl` 1.0.27
  (new), `schemars` 0.8.22 -> 1.2.2, `schemars_derive` 0.8.22 -> 1.2.2,
  `serde_derive_internals` 0.29.1 -> 0.30.0.
- **The Node binding's build-tool tree matches the family.** The packages
  `bindings/node/package-lock.json` holds only for `@napi-rs/cli` 3.10.4 are
  resolved again, so every repository in the family builds with one identical
  tree; the CLI itself, which generates the committed loader, stays.
- **The READMEs show the organization's banner as vectors,** so its text stays
  sharp at any width and zoom.

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

[Unreleased]: https://github.com/wickra-lib/wickra-shazam/compare/v0.2.3...HEAD
[0.2.3]: https://github.com/wickra-lib/wickra-shazam/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/wickra-lib/wickra-shazam/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/wickra-lib/wickra-shazam/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/wickra-lib/wickra-shazam/compare/v0.1.4...v0.2.0
[0.1.4]: https://github.com/wickra-lib/wickra-shazam/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/wickra-lib/wickra-shazam/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/wickra-lib/wickra-shazam/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/wickra-lib/wickra-shazam/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/wickra-lib/wickra-shazam/releases/tag/v0.1.0
