"use strict";

// Parity guard: the Node binding must expose the full public surface of the
// shazam, so an export dropped in a refactor fails loudly here (mirrors the
// completeness check in the main wickra repo).

const { test } = require("node:test");
const assert = require("node:assert");
const wickra = require("../index.js");

test("module exposes Shazam and version", () => {
  assert.strictEqual(typeof wickra.Shazam, "function");
  assert.strictEqual(typeof wickra.version, "function");
});

test("Shazam exposes command and version", () => {
  for (const name of ["command", "version"]) {
    assert.strictEqual(
      typeof wickra.Shazam.prototype[name],
      "function",
      `Shazam is missing ${name}`,
    );
  }
});

// Strict parity: pin the exact public surface so a stray export (or a dropped
// one) fails here, matching the exact-surface guards in the Python and R
// bindings.
test("module surface is exactly {Shazam, version}", () => {
  assert.deepStrictEqual(Object.keys(wickra).sort(), ["Shazam", "version"]);
});

test("Shazam surface is exactly {command, version}", () => {
  const methods = Object.getOwnPropertyNames(wickra.Shazam.prototype)
    .filter((name) => name !== "constructor")
    .sort();
  assert.deepStrictEqual(methods, ["command", "version"]);
});
