# Benchmarks

Shazam's cost splits in two: **indexing** — folding an asset's whole history into
a rolling fingerprint at every bar — and **matching** — computing one current
fingerprint and finding its `k` nearest historical neighbours under the chosen
metric. The benchmarks here measure that **core index + match work**, so
throughput scales predictably with the history length, the fingerprint dimension
and the metric.

## What is measured

The `shazam-bench` crate (criterion) covers:

- **index** — building the `FingerprintIndex` over a history of N bars.
- **match** — one `match_current` over an index of N fingerprints for a fixed `k`.
- **Fingerprint dimension** — specs of a few different dimensions `N`.
- **Execution path** — the default rayon `parallel` feature vs
  `--no-default-features` (the sequential / WASM path), which must produce a
  byte-identical report.

## Methodology

Run against fixed, in-process synthetic histories so the numbers are reproducible
and contain no I/O variance:

```bash
cargo bench -p shazam-bench
```

## Results

_To be filled in from the criterion run in the test-rigor / docs phase._ Figures
will be the median estimate on a single machine, default `parallel` (rayon) path;
treat them as orders of magnitude, not guarantees — they vary with CPU and
toolchain. The nightly `bench.yml` workflow reruns the suite on a clean runner for
tracking over time.

## Caveats

These figures bound shazam's own index + match overhead only. End-to-end time in
a real run also depends on loading the history from disk or a live feed, which
these in-process benchmarks do not capture.
