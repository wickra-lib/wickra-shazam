"use strict";

// Operating-mode equivalence through the binding, over the golden corpus. The
// command protocol offers two ways to attach a label to a historical window,
// and both must yield the same `match` report bytes: a `label` sent before
// `index` is kept on the handle and applied when the index is built; a `label`
// sent after `index` goes onto the live index. Re-indexing the same history
// keeps the labels and the report. The core pins this in Rust
// (operating_modes.rs); this checks the boundary the Node binding crosses. A
// missing corpus is a failure, not a skip.

const { test } = require("node:test");
const assert = require("node:assert");
const fs = require("node:fs");
const path = require("node:path");
const { Shazam } = require("../index.js");

const LABEL = "golden_top";

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

test("a label before index matches a label after index", () => {
  const golden = findGolden();
  assert.ok(golden, "golden corpus not found");
  const history = candlesJson(path.join(golden, "data/history/sym-01.csv"));
  const current = candlesJson(path.join(golden, "data/current/sym-01.csv"));
  const indexCmd = `{"cmd":"index","history":${history}}`;
  const matchCmd = `{"cmd":"match","current":${current},"k":5}`;
  const specDir = path.join(golden, "specs");
  const files = fs.readdirSync(specDir).filter((f) => f.endsWith(".json")).sort();
  assert.ok(files.length > 0, "golden corpus not found");
  for (const file of files) {
    const spec = fs.readFileSync(path.join(specDir, file), "utf8");
    const expected = fs.readFileSync(path.join(golden, "expected", file), "utf8").trim();

    const plain = new Shazam(spec);
    plain.command(indexCmd);
    const unlabelled = plain.command(matchCmd);
    assert.strictEqual(unlabelled.trim(), expected, file);
    const ts = /"ts":(\d+)/.exec(unlabelled)[1];
    const labelCmd = `{"cmd":"label","ts":${ts},"label":"${LABEL}"}`;

    const before = new Shazam(spec);
    before.command(labelCmd);
    before.command(indexCmd);
    const labelledBefore = before.command(matchCmd);

    const after = new Shazam(spec);
    after.command(indexCmd);
    after.command(labelCmd);
    const labelledAfter = after.command(matchCmd);
    assert.strictEqual(labelledBefore, labelledAfter, file);
    assert.ok(labelledAfter.includes(LABEL), file);
    assert.notStrictEqual(labelledAfter, unlabelled, file);

    after.command(indexCmd);
    assert.strictEqual(after.command(matchCmd), labelledAfter, file);
  }
});
