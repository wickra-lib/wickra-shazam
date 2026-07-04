# Wickra Shazam — Java

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

## Usage

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

## API

| Member | Description |
|--------|-------------|
| `new Shazam(String specJson)` | Build a shazam from a spec JSON (throws `IllegalArgumentException` on an invalid spec). |
| `String command(String cmdJson)` | Apply a command JSON, return the response JSON. |
| `static String version()` | The library version. |
| `close()` | Free the native handle (via `AutoCloseable`). |

## License

`MIT OR Apache-2.0`.
