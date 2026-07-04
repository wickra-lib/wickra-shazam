"use strict";

const { test } = require("node:test");
const assert = require("node:assert");
const { Shazam, version } = require("../index.js");

const SPEC = JSON.stringify({
  features: [{ kind: "price", field: "close" }],
  window: 1,
  metric: "euclid",
});

function candle(time, close) {
  return { time, open: close, high: close, low: close, close, volume: 1.0 };
}

test("index and match returns the nearest historical timestamp", () => {
  const shazam = new Shazam(SPEC);
  const history = Array.from({ length: 10 }, (_, i) =>
    candle(i + 1, 100 + i + 1),
  );
  const indexed = JSON.parse(
    shazam.command(JSON.stringify({ cmd: "index", history })),
  );
  assert.strictEqual(indexed.indexed, 10);

  const response = shazam.command(
    JSON.stringify({ cmd: "match", current: [candle(11, 110.0)], k: 3 }),
  );
  const report = JSON.parse(response);
  assert.strictEqual(report.indexed, 10);
  assert.strictEqual(report.matches.length, 3);
  assert.strictEqual(report.matches[0].ts, 10);
});

test("version matches the module-level function", () => {
  const shazam = new Shazam(SPEC);
  assert.strictEqual(shazam.version(), version());
});

test("a malformed spec throws", () => {
  assert.throws(() => new Shazam("not json"));
});
