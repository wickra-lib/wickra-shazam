using System.Text;
using Wickra.Shazam;
using Xunit;

namespace WickraShazam.Tests;

// Cross-language golden: index sym-01's history, match its current window with
// k=5, and assert the response is byte-identical to golden/expected/<spec>.json.
// Candle JSON is built from the raw CSV tokens so no per-language number
// formatting (or locale) can drift.
public class GoldenTests
{
    private static string? FindGolden()
    {
        string dir = AppContext.BaseDirectory;
        for (int i = 0; i < 12; i++)
        {
            string g = Path.Combine(dir, "golden");
            if (Directory.Exists(Path.Combine(g, "specs")))
            {
                return g;
            }
            DirectoryInfo? parent = Directory.GetParent(dir);
            if (parent is null)
            {
                break;
            }
            dir = parent.FullName;
        }
        return null;
    }

    private static string CandlesJson(string path)
    {
        var sb = new StringBuilder("[");
        bool first = true;
        foreach (string raw in File.ReadAllLines(path))
        {
            string line = raw.Trim();
            if (line.Length == 0)
            {
                continue;
            }
            string[] c = line.Split(',');
            if (!long.TryParse(c[0], out _))
            {
                continue; // header
            }
            if (!first)
            {
                sb.Append(',');
            }
            first = false;
            sb.Append($"{{\"time\":{c[0]},\"open\":{c[1]},\"high\":{c[2]},\"low\":{c[3]},\"close\":{c[4]},\"volume\":{c[5]}}}");
        }
        sb.Append(']');
        return sb.ToString();
    }

    [Fact]
    public void GoldenMatchesAreByteIdentical()
    {
        string? golden = FindGolden();
        Assert.NotNull(golden);

        string history = CandlesJson(Path.Combine(golden!, "data/history/sym-01.csv"));
        string current = CandlesJson(Path.Combine(golden!, "data/current/sym-01.csv"));

        string[] specs = Directory.GetFiles(Path.Combine(golden!, "specs"), "*.json");
        Assert.NotEmpty(specs);
        foreach (string specPath in specs)
        {
            string name = Path.GetFileName(specPath);
            string spec = File.ReadAllText(specPath);
            string expected = File.ReadAllText(Path.Combine(golden!, "expected", name)).Trim();
            using var shazam = new Shazam(spec);
            shazam.Command($"{{\"cmd\":\"index\",\"history\":{history}}}");
            string response = shazam.Command($"{{\"cmd\":\"match\",\"current\":{current},\"k\":5}}");
            Assert.Equal(expected, response.Trim());
        }
    }
}
