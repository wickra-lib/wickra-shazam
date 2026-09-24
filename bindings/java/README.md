<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514-7" alt="Wickra Shazam — match an asset's current microstructure fingerprint against its entire history" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/ci.svg)](https://github.com/wickra-lib/wickra-shazam/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-shazam)
[![Maven Central](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/maven.svg)](https://central.sonatype.com/artifact/org.wickra/wickra-shazam)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-shazam/license.svg)](https://github.com/wickra-lib/wickra-shazam#license)

# Wickra Shazam — Java

---

> **▶ Live demo:** all 514 indicators over real Binance market data, computed live in your browser — **[live.wickra.org](https://live.wickra.org)** · zero backend, powered by `wickra-wasm`.

**Point at live data → "that's the May-2021 crash setup". Match the current microstructure fingerprint of an asset against its entire history — for Java. `org.wickra:wickra-shazam` — prebuilt native library inside the jar, no JNI, no system dependencies.**

JVM bindings for the `wickra-shazam` data-driven core over its C ABI hub
(FFM / Panama, `java.lang.foreign`). Build a `Shazam` from a spec JSON, drive it
with command JSON, read back match reports — the same protocol as every other
binding.

## Requirements

- Java 22+ (the Foreign Function & Memory API is stable since 22).
- Run with `--enable-native-access=ALL-UNNAMED`.
- The native library (`wickra_shazam`) must be resolvable — either on the
  library path or via the `native.lib.dir` system property pointing at the
  directory that holds `libwickra_shazam.{so,dylib}` / `wickra_shazam.dll`.

## Install

Maven:

```xml
<dependency>
  <groupId>org.wickra</groupId>
  <artifactId>wickra-shazam</artifactId>
  <version>0.1.3</version>
</dependency>
```

Gradle:

```kotlin
implementation("org.wickra:wickra-shazam:0.1.3")
```

The native library ships prebuilt per platform inside the jar and is
extracted automatically on first use. There is nothing to compile.

## Quick start

```java
import org.wickra.shazam.Shazam;

String spec = """
    {"features":[{"kind":"price","field":"close"}],"window":1,"metric":"euclid"}""";

try (Shazam shazam = new Shazam(spec)) {
    // Index the asset's history.
    shazam.command("""
        {"cmd":"index","history":[
        {"time":1,"open":101,"high":101,"low":101,"close":101,"volume":1},
        {"time":2,"open":102,"high":102,"low":102,"close":102,"volume":1}]}""");

    // Match the current state against the history.
    String report = shazam.command("""
        {"cmd":"match","current":[
        {"time":3,"open":102,"high":102,"low":102,"close":102,"volume":1}],"k":2}""");
    System.out.println(report); // {"indexed":2,"matches":[{"similarity":...,"ts":2},...]}
}
System.out.println(Shazam.version());
```

### API

| Member | Description |
|--------|-------------|
| `new Shazam(String specJson)` | Build a shazam from a spec JSON (throws `IllegalArgumentException` on an invalid spec). |
| `String command(String cmdJson)` | Apply a command JSON, return the response JSON. |
| `static String version()` | The library version. |
| `close()` | Free the native handle (via `AutoCloseable`). |

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of the Java Foreign Function & Memory API over the C ABI, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-shazam/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-shazam>
- **Docs** (guides, spec reference, cookbook): <https://shazam.wickra.org>
- **Runnable example:** [`examples/java/`](https://github.com/wickra-lib/wickra-shazam/tree/main/examples/java)

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
