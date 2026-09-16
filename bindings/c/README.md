<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Shazam — match an asset's current microstructure fingerprint against its entire history" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/ci.svg)](https://github.com/wickra-lib/wickra-shazam/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-shazam)
[![GitHub release](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/release.svg)](https://github.com/wickra-lib/wickra-shazam/releases/latest)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/license.svg)](https://github.com/wickra-lib/wickra-shazam#license)

# Wickra Shazam — C / C++

---

> **▶ Live demo:** all 514 indicators over real Binance market data, computed live in your browser — **[live.wickra.org](https://live.wickra.org)** · zero backend, powered by `wickra-wasm`.

**Point at live data → "that's the May-2021 crash setup". Match the current microstructure fingerprint of an asset against its entire history — for C / C++. `cargo build -p wickra-shazam-c --release` — a prebuilt shared/static library plus a generated `wickra_shazam.h`, no system dependencies.**

The C ABI is the hub every C-capable language (C, C++, C#, Go, Java, R) links
against. It exposes `wickra-shazam-core` as a tiny, JSON-shaped surface built as both a
`cdylib` (dynamic library) and a `staticlib`.

## Install

Grab the prebuilt header + library for your platform from the
[GitHub releases](https://github.com/wickra-lib/wickra-shazam/releases) — each archive
has `wickra_shazam.h`, the C++ wrapper where the binding ships one, and the shared/static
library — or build from source:

```bash
cargo build -p wickra-shazam-c --release
# -> target/release/libwickra_shazam.{so,dylib} or wickra_shazam.dll (+ import lib) + a staticlib
```

Then compile against the header and link the library.

## Quick start

[`examples/c/match.c`](https://github.com/wickra-lib/wickra-shazam/blob/main/examples/c/match.c) is the runnable example the CI smoke job executes; in full:

```c
/* A minimal C example: index a history and match the current state through the
   wickra-shazam C ABI. */
#include <stdio.h>
#include <stdlib.h>

#include "wickra_shazam.h"

static const char *SPEC =
    "{\"features\":[{\"kind\":\"price\",\"field\":\"close\"}],"
    "\"window\":1,\"metric\":\"euclid\"}";

static const char *INDEX =
    "{\"cmd\":\"index\",\"history\":["
    "{\"time\":1,\"open\":100,\"high\":100,\"low\":100,\"close\":100,\"volume\":1},"
    "{\"time\":2,\"open\":101,\"high\":101,\"low\":101,\"close\":101,\"volume\":1},"
    "{\"time\":3,\"open\":102,\"high\":102,\"low\":102,\"close\":102,\"volume\":1}]}";

static const char *MATCH =
    "{\"cmd\":\"match\",\"current\":["
    "{\"time\":4,\"open\":102,\"high\":102,\"low\":102,\"close\":102,\"volume\":1}],"
    "\"k\":2}";

/* Run a command with the length-out protocol; returns a malloc'd response the
   caller must free, or NULL on error. */
static char *run(WickraShazam *shazam, const char *cmd) {
    int len = wickra_shazam_command(shazam, cmd, NULL, 0);
    if (len < 0) {
        return NULL;
    }
    char *buf = (char *)malloc((size_t)len + 1);
    if (!buf) {
        return NULL;
    }
    wickra_shazam_command(shazam, cmd, buf, (size_t)len + 1);
    return buf;
}

int main(void) {
    WickraShazam *shazam = wickra_shazam_new(SPEC);
    if (!shazam) {
        fprintf(stderr, "failed to build shazam\n");
        return 1;
    }

    char *indexed = run(shazam, INDEX);
    char *report = indexed ? run(shazam, MATCH) : NULL;
    if (!indexed || !report) {
        fprintf(stderr, "command failed\n");
        free(indexed);
        free(report);
        wickra_shazam_free(shazam);
        return 1;
    }

    printf("wickra-shazam %s\n", wickra_shazam_version());
    printf("indexed: %s\n", indexed);
    printf("match: %s\n", report);

    free(indexed);
    free(report);
    wickra_shazam_free(shazam);
    return 0;
}
```

### Surface

```c
#include "wickra_shazam.h"

WickraShazam *wickra_shazam_new(const char *spec_json);
void          wickra_shazam_free(WickraShazam *handle);
int32_t       wickra_shazam_command(WickraShazam *handle,
                                    const char *cmd_json,
                                    char *out, size_t cap);
const char   *wickra_shazam_version(void);
```

- **`wickra_shazam_new`** builds a shazam from a spec JSON. Returns `NULL` if the
  argument is null, not UTF-8, or not a valid spec.
- **`wickra_shazam_free`** destroys a handle (null is a no-op).
- **`wickra_shazam_command`** applies a command JSON and writes the response JSON
  into the caller's buffer using a length-out protocol (below).
- **`wickra_shazam_version`** returns a static, NUL-terminated version string (do
  not free).

### Command / response protocol

Everything after construction goes through `wickra_shazam_command`. Commands are
JSON objects with a `"cmd"` field: `set_spec`, `index`, `label`, `match`,
`reset`, `version`. Responses are JSON, e.g.
`{"matches":[...],"indexed":N}` for a match or `{"ok":true}` for a mutation.

The response is returned via a caller-owned buffer with a length-out protocol —
the callee never allocates memory the caller must free:

1. Call with `out = NULL`, `cap = 0` to learn the response length `len`
   (excluding the terminating NUL).
2. Allocate `len + 1` bytes and call again; the response plus a NUL is written.

Whenever `len < cap`, the response is written on that call, so a
sufficiently-large buffer needs only one call.

A mutating command (`set_spec`, `index`, `label`, `reset`) is executed exactly
once across those calls: the handle caches the response it has computed but not
yet delivered, and a repeated call with the same command bytes reuses it
instead of re-executing. Once the response has been written to a buffer the
cache is cleared, so the next identical command executes freshly.

Return codes:

| Return   | Meaning                                             |
|----------|-----------------------------------------------------|
| `>= 0`   | Response length in bytes (excluding the NUL).       |
| `-1`     | A required pointer (`handle` or `cmd_json`) is null. |
| `-2`     | `cmd_json` is not valid UTF-8.                       |
| `-3`     | A panic was caught at the boundary.                 |

Domain errors (a bad spec, an unknown command) are **not** negative — they come
back in-band as `{"ok":false,"error":...}` JSON in the buffer.

### C++

`include/wickra_shazam.hpp` is a header-only C++17 hull over the same four
functions: `wickra::Shazam` owns and frees the handle, `command` runs the
length-out protocol for you, and a negative return becomes a
`wickra::ShazamError`. In-band refusals (`{"ok":false,...}`) are returned as
strings, not thrown. `examples/c/match.cpp` builds against it.

### Header generation

`include/wickra_shazam.h` is generated with [cbindgen] and committed; CI fails
if it drifts from the source. Regenerate after changing the ABI:

```sh
cbindgen --config cbindgen.toml --crate wickra-shazam-c --output include/wickra_shazam.h
```

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of the C ABI itself, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-shazam/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-shazam>
- **Docs** (guides, spec reference, cookbook): <https://shazam.wickra.org>
- **Runnable example:** [`examples/c/`](https://github.com/wickra-lib/wickra-shazam/tree/main/examples/c)

Wickra Shazam ships native bindings for Python, Node.js, WASM and Rust, plus a C ABI hub that any
C-capable language (C, C++, C#, Go, Java, R) links against — all forwarding to the
same data-driven, `unsafe`-forbidden Rust core.

## Security

Found a security issue? **Please don't open a public issue.** Report it privately
via the repository's *Security* tab (*"Report a vulnerability"*) or email
**support@wickra.org** with a subject line starting `[wickra security]`. Full
policy: <https://github.com/wickra-lib/wickra-shazam/blob/main/SECURITY.md>.

## Disclaimer

Wickra Shazam is analysis software: it computes similarity between market states.
A historical match is a statistical resemblance, **not a prediction** and **not
financial advice** — the past setup did not have to repeat, and neither does this
one. It places no orders. Trading carries risk of loss; review the code and use
at your own discretion.

## License

Licensed under either of [Apache-2.0](https://github.com/wickra-lib/wickra-shazam/blob/main/LICENSE-APACHE)
or [MIT](https://github.com/wickra-lib/wickra-shazam/blob/main/LICENSE-MIT) at your option.

[cbindgen]: https://github.com/mozilla/cbindgen
