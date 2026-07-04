# Threat Model

`wickra-shazam` is analysis software. It fingerprints market states and computes
similarity over historical data, places no orders, opens no authenticated
connections on its default path, and holds no secret key material. The attack
surface is correspondingly narrow: it is dominated by the parsing of **untrusted
input** — a `FingerprintSpec` and the history/current data supplied by the caller
— as it crosses the C ABI and WASM boundary.

## Assets

- The **`FingerprintSpec` and history data** a caller supplies. These are inputs,
  not secrets, but a malformed or hostile one must never crash or corrupt the host.
- The **integrity and determinism** of the `MatchReport`: the same spec and data
  must always produce the same result, in every language and in both the parallel
  and sequential builds.
- The **host process** embedding a binding. Indexing or matching must not be able
  to take it down (panic across FFI, unbounded allocation) or read memory it
  should not.

There is intentionally **no secret asset** on the default path — no API keys, no
credentials, no order flow.

## Trust boundaries

- **Caller → core.** Everything arriving through `Shazam::command` (spec, data,
  command) is untrusted and validated (`FingerprintSpec::validate`) before use.
- **Binding → C ABI hub.** The hub is the one place `unsafe` is allowed. It wraps
  every call in `catch_unwind`, guards null pointers, and uses a length-out
  buffer protocol so no panic or invalid pointer crosses into C / Go / C# / Java
  / R.
- **Optional `live` feature.** Only this pulls `wickra-exchange` to read the
  current public market state; it adds a network read but still no credentials or
  orders.

## Guarantees the code is held to

- `unsafe_code = "forbid"` workspace-wide; only `bindings/c` re-allows it locally.
- No panic crosses the FFI boundary; errors are returned as JSON, never as an
  abort.
- Parsing is bounded and total — a hostile spec or dataset yields an error, not
  an unbounded allocation or a hang. In particular the fingerprint dimension is
  capped (`MAX_DIM`): a spec whose feature list would exceed it is rejected at
  validation, so it cannot drive an out-of-memory allocation of the index.
- The parallel (rayon) and sequential (WASM) paths produce a byte-identical
  report, so parallelism introduces no nondeterminism.
- Degenerate numeric cases are clamped, so `NaN`/`±inf` never reach the output
  and a hostile input cannot poison a similarity score.

## Out of scope

- Incorrect indicator or metric mathematics — a functional bug, handled through
  normal issues and tests, not a vulnerability.
- Vulnerabilities in third-party crates, which are tracked and triaged through
  `deny.toml` and `osv-scanner.toml`.
- Resource exhaustion a caller inflicts on **their own** process by deliberately
  feeding an enormous history; the core bounds its own per-fingerprint allocation
  (`MAX_DIM`) but cannot bound the sheer number of bars the caller supplies.
