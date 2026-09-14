using System.Text.RegularExpressions;
using Wickra.Shazam;
using Xunit;

namespace WickraShazam.Tests;

// Operating-mode equivalence through the binding, over the golden corpus. The
// command protocol offers two ways to attach a label to a historical window,
// and both must yield the same `match` report bytes: a `label` sent before
// `index` is kept on the handle and applied when the index is built; a `label`
// sent after `index` goes onto the live index. Re-indexing the same history
// keeps the labels and the report. The core pins this in Rust
// (operating_modes.rs); this checks the boundary the C# binding crosses. A
// missing corpus is a failure, not a skip.
public class ModeTests
{
    private const string Label = "golden_top";

    [Fact]
    public void ALabelBeforeIndex_MatchesALabelAfterIndex()
    {
        string? golden = GoldenTests.FindGolden();
        Assert.NotNull(golden);

        string history = GoldenTests.CandlesJson(Path.Combine(golden!, "data/history/sym-01.csv"));
        string current = GoldenTests.CandlesJson(Path.Combine(golden!, "data/current/sym-01.csv"));
        string indexCmd = $"{{\"cmd\":\"index\",\"history\":{history}}}";
        string matchCmd = $"{{\"cmd\":\"match\",\"current\":{current},\"k\":5}}";

        string[] specs = Directory.GetFiles(Path.Combine(golden!, "specs"), "*.json");
        Array.Sort(specs, StringComparer.Ordinal);
        Assert.NotEmpty(specs);
        foreach (string specPath in specs)
        {
            string name = Path.GetFileName(specPath);
            string spec = File.ReadAllText(specPath);
            string expected = File.ReadAllText(Path.Combine(golden!, "expected", name)).Trim();

            using var plain = new Shazam(spec);
            plain.Command(indexCmd);
            string unlabelled = plain.Command(matchCmd).Trim();
            Assert.True(expected == unlabelled, $"{name}: match does not match the blessed report");
            Match ts = Regex.Match(unlabelled, "\"ts\":(\\d+)");
            Assert.True(ts.Success, $"{name}: no match in the report");
            string labelCmd = $"{{\"cmd\":\"label\",\"ts\":{ts.Groups[1].Value},\"label\":\"{Label}\"}}";

            using var before = new Shazam(spec);
            before.Command(labelCmd);
            before.Command(indexCmd);
            string labelledBefore = before.Command(matchCmd).Trim();

            using var after = new Shazam(spec);
            after.Command(indexCmd);
            after.Command(labelCmd);
            string labelledAfter = after.Command(matchCmd).Trim();
            Assert.True(labelledBefore == labelledAfter, $"{name}: a label before index differs from a label after index");
            Assert.Contains(Label, labelledAfter);
            Assert.True(labelledAfter != unlabelled, $"{name}: the label changed nothing");

            after.Command(indexCmd);
            Assert.True(after.Command(matchCmd).Trim() == labelledAfter, $"{name}: re-indexing changed the report");
        }
    }
}
