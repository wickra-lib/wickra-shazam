<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Shazam — match an asset's current microstructure fingerprint against its entire history" width="100%"></a>
</p>

[![Built on Wickra](https://img.shields.io/badge/built%20on-wickra-3b82f6)](https://github.com/wickra-lib/wickra)
[![Status](https://img.shields.io/badge/status-pre--release-orange)](https://github.com/wickra-lib/wickra-shazam)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](#license)

<!-- Skeleton README (P-SHZ-0.12). The full ~20-badge block (CI, CodeQL, codecov,
     crates.io/PyPI/npm/NuGet/Maven/Go/R-universe, Scorecard, Best-Practices,
     Provenance, Docs) and the finished sections are assembled in P-SHZ-9.1, once
     the per-product badge SVGs are generated in the .github repo (P-SHZ-9.2).
     Until then this stays link-clean (no 404s on the repo page). -->

---

# Wickra Shazam

**Point at live data → "that's the May-2021 crash setup". Match the current 514-dim microstructure fingerprint of an asset against its entire history.**

Wickra Shazam turns an asset's **whole history** into a rolling index of
fixed-dimension **microstructure fingerprints** — a vector built from the full
Wickra feature space (indicators, price and microstructure: order-book
imbalance, funding, open interest, liquidations, footprint) — and matches the
**current** fingerprint against that entire index to name the regime. It is
pattern/regime recognition over the **full feature space, not price alone**.

- **The fingerprint is data** — a serde `FingerprintSpec` (an ordered feature
  list + window + normalize + metric), not Rust closures, so it crosses the C ABI
  and WASM unchanged. A fixed dimension `N` is what makes it deterministic.
- **Deterministic core** — indexing and matching are byte-identical across all
  ten languages and between the parallel (rayon) and sequential (WASM) builds.
- **Three operations, one core** — `index(history, spec)` builds the rolling
  index, `match_current(index, current, k)` finds the `k` most similar historical
  fingerprints, and a label attaches a human name (`"may_2021_crash"`) to a match.

The core is one library (`shazam-core`), usable from **Rust, Python, Node.js,
WASM, C, C++, C#, Go, Java and R** over a JSON-over-C-ABI boundary, plus a
reference CLI.

## Status

**Pre-release — under active construction.** This repository is being built out
phase by phase (scaffold → core → CLI → ten language bindings → golden corpus →
tests → CI → docs). It is not yet published to any registry.

## Documentation

The full documentation — the `FingerprintSpec` / feature reference, the fingerprint
and index/match data-model, the similarity metrics, and per-binding quickstarts —
is finalized in this README and under `docs/` during the documentation phase.

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.

## Disclaimer

Wickra Shazam is analysis software: it computes similarity between market states.
A historical match is a statistical resemblance, **not a prediction** and **not
financial advice** — the past setup did not have to repeat, and neither does this
one. It places no orders. Trading carries risk; use at your own discretion.
