// A runnable Java example: index a history and match the current state.
//
//   cargo build -p wickra-shazam-c
//   mvn -f bindings/java/pom.xml -q package -DskipTests
//   javac -cp bindings/java/target/classes examples/java/Match.java -d examples/java/out
//   java --enable-native-access=ALL-UNNAMED \
//        -Dnative.lib.dir=target/debug \
//        -cp "bindings/java/target/classes;examples/java/out" Match
import org.wickra.shazam.Shazam;

public final class Match {
    private static final String SPEC =
            "{\"features\":[{\"kind\":\"price\",\"field\":\"close\"}],"
                    + "\"window\":1,\"metric\":\"euclid\"}";

    private static String candle(int time, int close) {
        return "{\"time\":" + time + ",\"open\":" + close + ",\"high\":" + close
                + ",\"low\":" + close + ",\"close\":" + close + ",\"volume\":1}";
    }

    public static void main(String[] args) {
        try (Shazam shazam = new Shazam(SPEC)) {
            String history = "[" + candle(1, 100) + "," + candle(2, 101) + "," + candle(3, 102) + "]";
            shazam.command("{\"cmd\":\"index\",\"history\":" + history + "}");
            String response =
                    shazam.command("{\"cmd\":\"match\",\"current\":[" + candle(4, 102) + "],\"k\":2}");
            System.out.println("wickra-shazam " + Shazam.version());
            System.out.println(response);
        }
    }
}
