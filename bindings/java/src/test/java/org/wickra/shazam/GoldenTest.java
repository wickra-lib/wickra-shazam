package org.wickra.shazam;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;
import java.util.stream.Collectors;
import org.junit.jupiter.api.Test;

/**
 * Cross-language golden: index sym-01's history, match its current window with
 * k=5, and assert the response is byte-identical to golden/expected/&lt;spec&gt;.json.
 * Candle JSON is built from the raw CSV tokens so no per-language number
 * formatting (or locale) can drift.
 */
class GoldenTest {
    private static Path goldenDir() {
        Path dir = Paths.get("").toAbsolutePath();
        for (int i = 0; i < 8; i++) {
            Path g = dir.resolve("golden");
            if (Files.isDirectory(g.resolve("specs"))) {
                return g;
            }
            dir = dir.getParent();
            if (dir == null) {
                break;
            }
        }
        return null;
    }

    private static String candlesJson(Path path) throws IOException {
        List<String> rows = new ArrayList<>();
        for (String raw : Files.readAllLines(path, StandardCharsets.UTF_8)) {
            String line = raw.trim();
            if (line.isEmpty()) {
                continue;
            }
            String[] c = line.split(",");
            try {
                Long.parseLong(c[0]);
            } catch (NumberFormatException e) {
                continue; // header
            }
            rows.add("{\"time\":" + c[0] + ",\"open\":" + c[1] + ",\"high\":" + c[2]
                    + ",\"low\":" + c[3] + ",\"close\":" + c[4] + ",\"volume\":" + c[5] + "}");
        }
        return "[" + String.join(",", rows) + "]";
    }

    @Test
    void goldenMatchesAreByteIdentical() throws IOException {
        Path golden = goldenDir();
        assertTrue(golden != null, "golden fixtures not present");

        String history = candlesJson(golden.resolve("data/history/sym-01.csv"));
        String current = candlesJson(golden.resolve("data/current/sym-01.csv"));

        List<Path> specs;
        try (var stream = Files.list(golden.resolve("specs"))) {
            specs = stream.filter(p -> p.toString().endsWith(".json")).sorted()
                    .collect(Collectors.toList());
        }
        assertTrue(!specs.isEmpty(), "no golden specs found");
        for (Path specPath : specs) {
            String name = specPath.getFileName().toString();
            String spec = Files.readString(specPath, StandardCharsets.UTF_8);
            String expected =
                    Files.readString(golden.resolve("expected").resolve(name), StandardCharsets.UTF_8).trim();
            try (Shazam shazam = new Shazam(spec)) {
                shazam.command("{\"cmd\":\"index\",\"history\":" + history + "}");
                String response = shazam.command("{\"cmd\":\"match\",\"current\":" + current + ",\"k\":5}");
                assertEquals(expected, response.trim(), "mismatch for " + name);
            }
        }
    }
}
