# wickra-shazam (C#)

.NET bindings for [`wickra-shazam`](https://github.com/wickra-lib/wickra-shazam) over
the C ABI hub, via source-generated P/Invoke. Build a `Shazam` from a spec JSON,
index a history, label the windows you recognise, and match the current state
— the same protocol the CLI and every other binding speak, returning the same
bytes.

```csharp
using Wickra.Shazam;

const string spec = """
{"features":[{"kind":"indicator","name":"Rsi","params":[14]},
             {"kind":"price","field":"close"}],
 "window":10,"normalize":"z_score","metric":"cosine"}
""";

using var shazam = new Shazam(spec);
shazam.Command("""{"cmd":"index","history":[ … ]}""");
shazam.Command("""{"cmd":"label","ts":1700226800,"label":"may_2021_crash"}""");
string report = shazam.Command("""{"cmd":"match","current":[ … ],"k":5}""");
```

A `label` sent before `index` is kept on the handle and applied when the index
is built; sent after `index` it goes onto the live index. Both yield the same
`match` report, and re-indexing the same history keeps it. The core pins this
in Rust and the binding's test suite checks it across the committed golden
corpus.

Every `Command` call is executed exactly once. The C ABI's length-then-fill
protocol needs two calls when the first buffer is too small, and the hub caches
the response of a mutating command (`index`, `label`, `set_spec`, `reset`)
until it has been delivered, so a retry never indexes twice.

Requires .NET 8+. The native library (`wickra_shazam`) must be resolvable on the
loader path — `PATH` on Windows, `LD_LIBRARY_PATH` on Linux, `DYLD_LIBRARY_PATH`
on macOS — or beside the assembly, where the bundled resolver finds it.

Licensed under either of [MIT](https://github.com/wickra-lib/wickra-shazam/blob/main/LICENSE-MIT) or
[Apache-2.0](https://github.com/wickra-lib/wickra-shazam/blob/main/LICENSE-APACHE) at your option.
