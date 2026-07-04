# Roadmap

`wickra-shazam` is built out in phases, mirroring the proven structure of the
Wickra exchange, backtester, terminal and screener repos. Each phase lands as
reviewed, CI-green pull requests. Status below is updated as phases complete.

## Phases

0. **Scaffold** — workspace, governance, supply-chain config, `.github`
   scaffolding. *In progress.*
1. **`shazam-core`** — the `FingerprintSpec`, the rolling per-bar fingerprint
   fold, normalization, the distance metrics, the `FingerprintIndex` and
   `match_current`, with near-total coverage via inline tests.
2. **`shazam-cli`** — the reference `wickra-shazam` binary: load a spec and a
   history, index it, match the current fingerprint, render the report as text or
   JSON.
3. **Bindings** — the C ABI hub first, then native Python, Node and WASM, then C,
   C++, C#, Go, Java and R over the hub; each exposes the `Shazam` handle +
   `command` + `version`, with a completeness guard.
4. **Golden harness** — a fixed deterministic history and current state, canonical
   specs, and blessed match reports that are the byte-exact, cross-language parity
   corpus.
5. **Test rigor** — conformance, golden, parallel-equals-sequential equivalence,
   property tests, fuzz targets and a criterion benchmark suite.
6. **ABI harness + examples** — cbindgen header sync-check and one runnable
   example per language, with a C/C++ CMake harness.
7. **CI/CD** — the full workflow matrix (all languages), OpenSSF Scorecard, Best
   Practices, link check, and the release workflow.
8. **README, badges, docs** — the banner + badge treatment and the docs guides
   (features, fingerprints, metrics, cookbook).

## Beyond 1.0

- A **live cross-section** — matching the current fingerprint of an asset against
  the fingerprints of **every other asset** at the same instant, not just against
  one asset's own history. This is the sibling idea "THE GENOME"; shazam lays the
  fingerprint machinery it builds on.
- Richer feature axes and distance metrics as the corpus grows.
- Sourcing the current fingerprint from a live exchange feed (the optional `live`
  feature), still read-only.

## Non-goals

- **Indicator code in this repository.** Indicators come from the `wickra-core`
  registry; shazam composes them into fingerprints, it does not reimplement them.
- **Fingerprints as code.** A fingerprint is a serde `FingerprintSpec`, never a
  Rust closure, so it crosses the C ABI and WASM unchanged.
- **A hosted service or stored credentials.** Shazam runs locally; it holds no
  order-secret material and places no orders. A historical match is a statistical
  resemblance, not a prediction.
