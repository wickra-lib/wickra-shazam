// A runnable Node.js example: index a history and match the current state.
//
//   ( cd bindings/node && npm install && npm run build )
//   ( cd examples/node && npm install && node match.js )

"use strict";

const { Shazam, version } = require("wickra-shazam");

const SPEC = JSON.stringify({
  features: [{ kind: "price", field: "close" }],
  window: 1,
  metric: "euclid",
});

const candle = (time, close) => ({
  time,
  open: close,
  high: close,
  low: close,
  close,
  volume: 1.0,
});

const shazam = new Shazam(SPEC);
shazam.command(
  JSON.stringify({
    cmd: "index",
    history: [candle(1, 100.0), candle(2, 101.0), candle(3, 102.0)],
  }),
);
const response = shazam.command(
  JSON.stringify({ cmd: "match", current: [candle(4, 102.0)], k: 2 }),
);
const report = JSON.parse(response);

console.log("wickra-shazam", version());
console.log(response);
for (const match of report.matches) {
  console.log(`  match ts ${match.ts} similarity ${match.similarity}`);
}
