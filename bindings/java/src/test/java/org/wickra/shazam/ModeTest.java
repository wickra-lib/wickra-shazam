package org.wickra.shazam;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.util.stream.Collectors;
import org.junit.jupiter.api.Test;

/**
 * Operating-mode equivalence through the binding, over the golden corpus. The
 * command protocol offers two ways to attach a label to a historical window,
 * and both must yield the same {@code match} report bytes: a {@code label}
 * sent before {@code index} is kept on the handle and applied when the index
 * is built; a {@code label} sent after {@code index} goes onto the live index.
 * Re-indexing the same history keeps the labels and the report. The core pins
 * this in Rust (operating_modes.rs); this checks the boundary the Java binding
 * crosses. A missing corpus is a failure, not a skip.
 */
class ModeTest {
    private static final String LABEL = "golden_top";
    private static final Pattern TOP_TS = Pattern.compile("\"ts\":(\\d+)");

    @Test
    void aLabelBeforeIndexMatchesALabelAfterIndex() throws IOException {
        Path golden = GoldenTest.goldenDir();
        assertNotNull(golden, "golden corpus not found");

        String history = GoldenTest.candlesJson(golden.resolve("data/history/sym-01.csv"));
        String current = GoldenTest.candlesJson(golden.resolve("data/current/sym-01.csv"));
        String indexCmd = "{\"cmd\":\"index\",\"history\":" + history + "}";
        String matchCmd = "{\"cmd\":\"match\",\"current\":" + current + ",\"k\":5}";

        List<Path> specs;
        try (var stream = Files.list(golden.resolve("specs"))) {
            specs = stream.filter(p -> p.toString().endsWith(".json")).sorted()
                    .collect(Collectors.toList());
        }
        assertTrue(!specs.isEmpty(), "golden corpus not found");
        for (Path specPath : specs) {
            String name = specPath.getFileName().toString();
            String spec = Files.readString(specPath, StandardCharsets.UTF_8);
            String expected =
                    Files.readString(golden.resolve("expected").resolve(name), StandardCharsets.UTF_8).trim();

            String unlabelled;
            try (Shazam plain = new Shazam(spec)) {
                plain.command(indexCmd);
                unlabelled = plain.command(matchCmd).trim();
            }
            assertEquals(expected, unlabelled, name);
            Matcher ts = TOP_TS.matcher(unlabelled);
            assertTrue(ts.find(), name + ": no match in the report");
            String labelCmd = "{\"cmd\":\"label\",\"ts\":" + ts.group(1) + ",\"label\":\"" + LABEL + "\"}";

            String labelledBefore;
            try (Shazam before = new Shazam(spec)) {
                before.command(labelCmd);
                before.command(indexCmd);
                labelledBefore = before.command(matchCmd).trim();
            }
            try (Shazam after = new Shazam(spec)) {
                after.command(indexCmd);
                after.command(labelCmd);
                String labelledAfter = after.command(matchCmd).trim();
                assertEquals(labelledBefore, labelledAfter, name + ": a label before index differs from a label after index");
                assertTrue(labelledAfter.contains(LABEL), name + ": the label did not ride along");
                assertNotEquals(unlabelled, labelledAfter, name + ": the label changed nothing");

                after.command(indexCmd);
                assertEquals(labelledAfter, after.command(matchCmd).trim(), name + ": re-indexing changed the report");
            }
        }
    }
}
