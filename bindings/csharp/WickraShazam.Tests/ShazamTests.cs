using System.Text.Json;
using Wickra.Shazam;
using Xunit;

namespace WickraShazam.Tests;

public class ShazamTests
{
    private const string Spec =
        "{\"features\":[{\"kind\":\"price\",\"field\":\"close\"}]," +
        "\"window\":1,\"metric\":\"euclid\"}";

    private static object Candle(int time, double close) =>
        new { time, open = close, high = close, low = close, close, volume = 1.0 };

    [Fact]
    public void Version_IsNonEmpty()
    {
        Assert.False(string.IsNullOrEmpty(Shazam.Version()));
    }

    [Fact]
    public void IndexAndMatch_ReturnsNearestTimestamp()
    {
        using var shazam = new Shazam(Spec);

        object[] history = Enumerable.Range(1, 10)
            .Select(i => Candle(i, 100 + i))
            .ToArray();
        string indexed = shazam.Command(JsonSerializer.Serialize(new
        {
            cmd = "index",
            history,
        }));
        using JsonDocument idx = JsonDocument.Parse(indexed);
        Assert.Equal(10, idx.RootElement.GetProperty("indexed").GetInt32());

        string raw = shazam.Command(JsonSerializer.Serialize(new
        {
            cmd = "match",
            current = new[] { Candle(11, 110) },
            k = 3,
        }));
        using JsonDocument report = JsonDocument.Parse(raw);
        Assert.Equal(10, report.RootElement.GetProperty("indexed").GetInt32());
        JsonElement matches = report.RootElement.GetProperty("matches");
        Assert.Equal(3, matches.GetArrayLength());
        Assert.Equal(10, matches[0].GetProperty("ts").GetInt64());
    }

    [Fact]
    public void InvalidSpec_Throws()
    {
        Assert.Throws<ArgumentException>(() => new Shazam("not json"));
    }

    [Fact]
    public void UnknownCommand_IsInBandError()
    {
        using var shazam = new Shazam(Spec);
        // An unknown command is not a hard error: the ABI returns a length and the
        // error surfaces in-band as {"ok":false,...} JSON.
        string raw = shazam.Command("{\"cmd\":\"nope\"}");
        Assert.Contains("\"ok\":false", raw);
    }
}
