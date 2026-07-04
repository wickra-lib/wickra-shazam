"use strict";

// Cross-language golden parity: build the shazam from each committed
// `golden/specs/*.json`, index `sym-01`'s history, match its current window with
// k=5, and assert the response equals `golden/expected/<spec>.json` byte-for-byte.
// Because every binding returns the core's compact `command_json` string
// verbatim, byte equality is the exact cross-language parity check. Candle JSON
// is built from the raw CSV tokens so no per-language number formatting can drift.

const { test } = require("node:test");
const assert = require("node:assert");
const fs = require("node:fs");
const path = require("node:path");
const { Shazam } = require("../index.js");

function findGolden() {
  let dir = __dirname;
  for (let i = 0; i < 8; i++) {
    const g = path.join(dir, "golden");
    if (fs.existsSync(path.join(g, "specs"))) {
      return g;
    }
    dir = path.dirname(dir);
  }
  return null;
}

function candlesJson(csvPath) {
  const out = [];
  for (const line of fs.readFileSync(csvPath, "utf8").split("\n")) {
    const t = line.trim();
    if (!t) continue;
    const c = t.split(",");
    if (Number.isNaN(parseInt(c[0], 10))) continue; // header
    out.push(
      `{"time":${c[0]},"open":${c[1]},"high":${c[2]},"low":${c[3]},"close":${c[4]},"volume":${c[5]}}`,
    );
  }
  return `[${out.join(",")}]`;
}

test("golden index+match are byte-identical", (t) => {
  const golden = findGolden();
  if (!golden) {
    t.skip("golden fixtures not present yet");
    return;
  }
  const history = candlesJson(path.join(golden, "data/history/sym-01.csv"));
  const current = candlesJson(path.join(golden, "data/current/sym-01.csv"));
  const specDir = path.join(golden, "specs");
  for (const file of fs.readdirSync(specDir).filter((f) => f.endsWith(".json"))) {
    const spec = fs.readFileSync(path.join(specDir, file), "utf8");
    const expected = fs
      .readFileSync(path.join(golden, "expected", file), "utf8")
      .trim();
    const shazam = new Shazam(spec);
    shazam.command(`{"cmd":"index","history":${history}}`);
    const response = shazam.command(`{"cmd":"match","current":${current},"k":5}`);
    assert.strictEqual(response.trim(), expected, `mismatch for ${file}`);
  }
});
