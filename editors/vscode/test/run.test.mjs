// Runs every page of the papyrus through the runner the "Run This Page"
// command uses, and compares the page it prints with what the native rhind
// prints, when that has been built.
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { existsSync, mkdtempSync, readdirSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const runner = fileURLToPath(new URL("../server/run.js", import.meta.url));
const papyrus = fileURLToPath(new URL("../../../papyrus/", import.meta.url));
const native = fileURLToPath(new URL("../../../target/release/rhind", import.meta.url));

function run(text, options = []) {
  const file = join(mkdtempSync(join(tmpdir(), "rhind-")), "page.rmp");
  writeFileSync(file, text);
  return { file, ...spawnSync(process.execPath, [runner, ...options, file]) };
}

test("the built-in rhind runs every page of the papyrus", () => {
  const names = readdirSync(papyrus).filter((f) => f.endsWith(".rmp"));
  assert.ok(names.length >= 50);
  for (const file of names) {
    const result = spawnSync(process.execPath, [runner, papyrus + file]);
    assert.equal(result.status, 0, `${file}: ${result.stderr}`);
    const page = result.stdout.toString();
    assert.match(page, /\d+ figures checked, \d+ right, \d+ wrong\.\n/, file);
    if (existsSync(native)) {
      const want = spawnSync(native, [papyrus + file]).stdout.toString();
      assert.equal(page, want, file);
    }
  }
});

test("a page is run, and a wrong figure is marked with what the working gives", () => {
  const result = run("No. 27\n\\ 1   3 '2\n  2   7\n\\ 4   15\n");
  assert.equal(result.status, 0, result.stderr.toString());
  const page = result.stdout.toString();
  assert.match(page, /^\\ 1 {3}3 2̅ +✓$/m);
  assert.match(page, /^\\ 4 {3}15 +✗ 14$/m);
  assert.match(page, /3 figures checked, 2 right, 1 wrong\./);
  assert.match(page, /line 4: 4 of 3 2̅ is 14; the papyrus has 15/);
});

test("working left out is written in", () => {
  const page = run("Multiply 9 by 9.\n").stdout.toString();
  assert.match(page, /^\\ 8 {3}72 +· rhind$/m);
  assert.match(page, /^Total 81\. +· rhind$/m);
});

test("--plain prints the scribe's figures without marks", () => {
  const result = run("No. 27\n\\ 1   3 '2\n  2   7\n\\ 4   15\n", ["--plain"]);
  assert.equal(result.status, 0);
  assert.equal(result.stdout.toString(), "No. 27\n\\ 1   3 2̅\n  2   7\n\\ 4   15\n");
});

test("--check names the file, the line and the column", () => {
  const result = run("No. 27\n\\ 1   3 '2\n  2   7\n\\ 4   15\nFrobnicate 3.\n", ["--check"]);
  assert.equal(result.status, 1);
  const lines = result.stdout.toString().trimEnd().split("\n");
  assert.equal(lines[0], `${result.file}:4:7: wrong: 4 of 3 2̅ is 14; the papyrus has 15`);
  assert.match(lines[1], new RegExp(`^${result.file.replace(/[.\\]/g, "\\$&")}:5:1: error: `));
});

test("a line not in the notation is reported, and the exit status is 1", () => {
  const result = run("Frobnicate 3.\n");
  assert.equal(result.status, 1);
  assert.match(result.stdout.toString(), /line 1: not in the notation: `Frobnicate` is not a number/);
});
