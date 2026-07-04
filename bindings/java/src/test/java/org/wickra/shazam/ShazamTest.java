package org.wickra.shazam;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import org.junit.jupiter.api.Test;

class ShazamTest {
    private static final String SPEC =
            "{\"features\":[{\"kind\":\"price\",\"field\":\"close\"}],"
                    + "\"window\":1,\"metric\":\"euclid\"}";

    private static String candle(int time, int close) {
        return "{\"time\":" + time + ",\"open\":" + close + ",\"high\":" + close
                + ",\"low\":" + close + ",\"close\":" + close + ",\"volume\":1}";
    }

    @Test
    void versionIsNonEmpty() {
        assertFalse(Shazam.version().isEmpty());
    }

    @Test
    void indexAndMatchReturnsNearestTimestamp() {
        try (Shazam shazam = new Shazam(SPEC)) {
            StringBuilder history = new StringBuilder("[");
            for (int i = 1; i <= 10; i++) {
                if (i > 1) {
                    history.append(',');
                }
                history.append(candle(i, 100 + i));
            }
            history.append(']');

            String indexed = shazam.command("{\"cmd\":\"index\",\"history\":" + history + "}");
            assertTrue(indexed.contains("\"indexed\":10"), indexed);

            String raw = shazam.command(
                    "{\"cmd\":\"match\",\"current\":[" + candle(11, 110) + "],\"k\":3}");
            assertTrue(raw.contains("\"indexed\":10"), raw);
            // The tail of a monotone history is nearest its latest bar (ts 10).
            assertTrue(raw.contains("\"ts\":10"), raw);
        }
    }

    @Test
    void invalidSpecThrows() {
        assertThrows(IllegalArgumentException.class, () -> new Shazam("not json"));
    }

    @Test
    void unknownCommandIsInBandError() {
        try (Shazam shazam = new Shazam(SPEC)) {
            // An unknown command is not a hard error: the ABI returns a length and
            // the error surfaces in-band as {"ok":false,...} JSON.
            String raw = shazam.command("{\"cmd\":\"nope\"}");
            assertEquals(true, raw.contains("\"ok\":false"), raw);
        }
    }
}
