// A runnable .NET example: index a history and match the current state.
//
//   cargo build --release -p wickra-shazam-c
//   dotnet run --project examples/csharp/Match

using System.Text.Json;
using Wickra.Shazam;

const string spec =
    "{\"features\":[{\"kind\":\"price\",\"field\":\"close\"}]," +
    "\"window\":1,\"metric\":\"euclid\"}";

static object Candle(int time, double close) =>
    new { time, open = close, high = close, low = close, close, volume = 1.0 };

using var shazam = new Shazam(spec);

shazam.Command(JsonSerializer.Serialize(new
{
    cmd = "index",
    history = new[] { Candle(1, 100), Candle(2, 101), Candle(3, 102) },
}));

string response = shazam.Command(JsonSerializer.Serialize(new
{
    cmd = "match",
    current = new[] { Candle(4, 102) },
    k = 2,
}));
using JsonDocument report = JsonDocument.Parse(response);

Console.WriteLine($"wickra-shazam {Shazam.Version()}");
Console.WriteLine(response);
foreach (JsonElement match in report.RootElement.GetProperty("matches").EnumerateArray())
{
    Console.WriteLine($"  match ts {match.GetProperty("ts").GetInt64()} similarity {match.GetProperty("similarity").GetDouble()}");
}
