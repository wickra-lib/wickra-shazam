# Fuzzing Wickra Shazam

[`cargo-fuzz`](https://rust-fuzz.github.io/book/cargo-fuzz.html) harnesses for the parsing and stateful entry points of Wickra Shazam. Fuzzing requires a nightly Rust toolchain; CI runs every target for 30 seconds on the family's pinned `nightly-2026-07-01`.

## Setup

```bash
cargo install cargo-fuzz
rustup toolchain install nightly-2026-07-01
```

The date is the family's fuzz nightly, pinned in `ci.yml`: a rolling `nightly`
regressed with a codegen ICE unrelated to this code, so every repository moves
the date together, on purpose.

## Targets

| Target | What it exercises |
| --- | --- |
| `spec_parse` | Spec parsing: arbitrary bytes are fed to `FingerprintSpec::from_json`. |
| `build_index` | Index construction: an attacker-controlled `{spec, history}` object is parsed and indexed. |
| `match_index` | The full index + match pipeline: an attacker-controlled `{spec, history, current, k}` object is indexed and matched. |
| `normalize_metric` | The normalize + metric paths: an arbitrary (validated) spec — driving any `Normalize` × `Metric` combination — is indexed and matched over a fixed, bounded history. |

## Run

```bash
# From the repository root:
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu spec_parse
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu build_index
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu match_index
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu normalize_metric
```

Each run continues until a crash is found or it is interrupted. A short
time-boxed smoke run is what CI does:

```bash
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu spec_parse -- -max_total_time=30
```

The expectation for every target is that it never panics: malformed or
adversarial input must surface as an `Err` or an in-band error, never a crash.
