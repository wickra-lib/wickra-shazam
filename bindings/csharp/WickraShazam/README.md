# Wickra Shazam — .NET

.NET bindings for the `wickra-shazam` data-driven core over its C ABI hub
(`[LibraryImport]`). Build a `Shazam` from a spec JSON, drive it with command
JSON, read back match reports — the same protocol as every other binding.

## Install

```sh
dotnet add package Wickra.Shazam
```

## Usage

```csharp
using System.Text.Json;
using Wickra.Shazam;

const string spec =
    "{\"features\":[{\"kind\":\"price\",\"field\":\"close\"}],\"window\":1,\"metric\":\"euclid\"}";

using var shazam = new Shazam(spec);

static object Candle(int time, double close) =>
    new { time, open = close, high = close, low = close, close, volume = 1.0 };

// Index the asset's history.
var history = Enumerable.Range(1, 10).Select(i => Candle(i, 100 + i)).ToArray();
shazam.Command(JsonSerializer.Serialize(new { cmd = "index", history }));

// Match the current state against the history.
string raw = shazam.Command(JsonSerializer.Serialize(new
{
    cmd = "match",
    current = new[] { Candle(11, 110) },
    k = 3,
}));

using JsonDocument report = JsonDocument.Parse(raw);
Console.WriteLine(report.RootElement.GetProperty("indexed").GetInt32());
Console.WriteLine(Shazam.Version());
```

## API

| Member | Description |
|--------|-------------|
| `new Shazam(specJson)` | Build a shazam from a spec JSON (throws `ArgumentException` if invalid). |
| `shazam.Command(cmdJson)` | Apply a command JSON, return the response JSON. |
| `Shazam.Version()` | The library version. |
| `shazam.Dispose()` | Free the native handle. |

## License

`MIT OR Apache-2.0`.
