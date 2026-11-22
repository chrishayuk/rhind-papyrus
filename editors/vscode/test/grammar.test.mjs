// Tokenises pages with the same TextMate engine VS Code uses and checks the
// scopes the grammar assigns.
import assert from "node:assert/strict";
import { readFileSync, readdirSync } from "node:fs";
import { createRequire } from "node:module";
import test from "node:test";
import { fileURLToPath } from "node:url";
import oniguruma from "vscode-oniguruma";
import textmate from "vscode-textmate";

const require = createRequire(import.meta.url);
const grammarPath = fileURLToPath(new URL("../syntaxes/rhind.tmLanguage.json", import.meta.url));
const papyrus = fileURLToPath(new URL("../../../papyrus/", import.meta.url));

const wasm = readFileSync(require.resolve("vscode-oniguruma/release/onig.wasm"));
await oniguruma.loadWASM(wasm.buffer.slice(wasm.byteOffset, wasm.byteOffset + wasm.byteLength));
const registry = new textmate.Registry({
  onigLib: Promise.resolve({
    createOnigScanner: (patterns) => new oniguruma.OnigScanner(patterns),
    createOnigString: (text) => new oniguruma.OnigString(text),
  }),
  loadGrammar: async () => textmate.parseRawGrammar(readFileSync(grammarPath, "utf8"), grammarPath),
});
const grammar = await registry.loadGrammar("source.rmp");

/** Each non-blank piece of text with its most specific scope. */
function scopes(source) {
  const out = [];
  let state = textmate.INITIAL;
  for (const line of source.split("\n")) {
    const result = grammar.tokenizeLine(line, state);
    for (const token of result.tokens) {
      const text = line.slice(token.startIndex, token.endIndex);
      if (text.trim()) out.push([text.trim(), token.scopes.at(-1)]);
    }
    state = result.ruleStack;
  }
  return out;
}

/** Every scope on the stack of the token that starts with `text`. */
function stack(line, text) {
  const result = grammar.tokenizeLine(line, textmate.INITIAL);
  const token = result.tokens.find((t) => line.slice(t.startIndex, t.endIndex).trim().startsWith(text));
  assert.ok(token, `no token ${text}`);
  return token.scopes;
}

test("headings: a problem and an entry of the 2 ÷ n table", () => {
  assert.deepEqual(scopes("No. 26"), [["No. 26", "entity.name.section.problem.rhind"]]);
  assert.deepEqual(scopes("No. 7B   % the second half"), [
    ["No. 7B", "entity.name.section.problem.rhind"],
    ["% the second half", "comment.line.percentage.rhind"],
  ]);
  assert.deepEqual(scopes("2 ÷ 7"), [["2 ÷ 7", "entity.name.section.table.rhind"]]);
  assert.deepEqual(scopes("2 : 7"), [["2 : 7", "entity.name.section.table.rhind"]]);
});

test("statements: their words, figures and full stop", () => {
  assert.deepEqual(scopes("Reckon with 5 to find 15."), [
    ["Reckon with", "keyword.control.statement.rhind"],
    ["5", "constant.numeric.whole.rhind"],
    ["to find", "keyword.other.statement.rhind"],
    ["15", "constant.numeric.whole.rhind"],
    [".", "punctuation.terminator.statement.rhind"],
  ]);
  assert.deepEqual(scopes("Multiply 3 '2 by 5."), [
    ["Multiply", "keyword.control.statement.rhind"],
    ["3", "constant.numeric.whole.rhind"],
    ["'2", "constant.numeric.part.rhind"],
    ["by", "keyword.other.statement.rhind"],
    ["5", "constant.numeric.whole.rhind"],
    [".", "punctuation.terminator.statement.rhind"],
  ]);
  const words = ["Total", "Result", "Remainder", "Add", "to", "Take", "from", "Proof"];
  const s = scopes("Total 1. Result 2. Remainder 3. Add 4 to 5. Take 6 from 7. Proof.");
  for (const w of words) assert.ok(s.some(([t, k]) => t === w && k.startsWith("keyword.")), w);
  assert.deepEqual(scopes("Total 3 '4 hekat 1 '3 ro.").filter(([, k]) => k.startsWith("support.")), [
    ["hekat", "support.type.unit.rhind"],
    ["ro", "support.type.unit.rhind"],
  ]);
});

test("a column: the tick and the figures in every spelling", () => {
  assert.deepEqual(scopes("\\ 4   15"), [
    ["\\", "keyword.operator.tick.rhind"],
    ["4", "constant.numeric.whole.rhind"],
    ["15", "constant.numeric.whole.rhind"],
  ]);
  assert.deepEqual(scopes("  2/3  3 '3 '15"), [
    ["2/3", "constant.numeric.two-thirds.rhind"],
    ["3", "constant.numeric.whole.rhind"],
    ["'3", "constant.numeric.part.rhind"],
    ["'15", "constant.numeric.part.rhind"],
  ]);
  assert.deepEqual(scopes("\\ 8   ⅔ 1̅0̅ ''3"), [
    ["\\", "keyword.operator.tick.rhind"],
    ["8", "constant.numeric.whole.rhind"],
    ["⅔", "constant.numeric.two-thirds.rhind"],
    ["1̅0̅", "constant.numeric.part.rhind"],
    ["''3", "constant.numeric.two-thirds.rhind"],
  ]);
  // a backslash that is not at the start of a line is not a tick
  assert.ok(!scopes("  1  5 \\").some(([, k]) => k === "keyword.operator.tick.rhind"));
});

test("the scribe's words, red ink, restorations and notes", () => {
  assert.deepEqual(scopes('"A quantity whose fourth part is added to it becomes 15."'), [
    ['"', "punctuation.definition.string.begin.rhind"],
    ["A quantity whose fourth part is added to it becomes 15.", "string.quoted.double.rhind"],
    ['"', "punctuation.definition.string.end.rhind"],
  ]);
  assert.deepEqual(scopes("{Total 21}"), [
    ["{", "punctuation.definition.red.begin.rhind"],
    ["Total", "keyword.control.statement.rhind"],
    ["21", "constant.numeric.whole.rhind"],
    ["}", "punctuation.definition.red.end.rhind"],
  ]);
  assert.ok(stack("{1 '2}", "'2").includes("meta.ink.red.rhind"));
  assert.ok(stack('{"Call"} "2 out of 3"', "Call").includes("string.quoted.double.rhind"));
  assert.ok(stack("  [1]   2/3 ['30]", "'30").includes("meta.restored.rhind"));
  assert.deepEqual(scopes("  2   [...]"), [
    ["2", "constant.numeric.whole.rhind"],
    ["[...]", "comment.block.lost.rhind"],
  ]);
  assert.deepEqual(scopes("  2   10?"), [
    ["2", "constant.numeric.whole.rhind"],
    ["10", "constant.numeric.whole.rhind"],
    ["?", "keyword.operator.uncertain.rhind"],
  ]);
  assert.deepEqual(scopes("\\ 4  15     % Peet: (sic)").at(-1), ["% Peet: (sic)", "comment.line.percentage.rhind"]);
  assert.deepEqual(scopes("Frobnicate 3.")[0], ["Frobnicate", "invalid.illegal.rhind"]);
});

test("every page of the papyrus tokenises with nothing marked invalid", () => {
  const files = readdirSync(papyrus).filter((f) => f.endsWith(".rmp"));
  assert.ok(files.length >= 50);
  for (const file of files) {
    const bad = scopes(readFileSync(papyrus + file, "utf8")).filter(([, k]) => k === "invalid.illegal.rhind");
    assert.deepEqual(bad, [], file);
  }
});
