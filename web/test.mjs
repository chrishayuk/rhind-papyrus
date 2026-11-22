// Runs every page of the papyrus and every example through the WebAssembly
// build, and compares with the native build:  node web/test.mjs
// (after `node web/build.mjs` and `cargo build --release`)
import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { rhind } from "./worker.js";
import { answer } from "./lsp-worker.js";

const root = fileURLToPath(new URL("..", import.meta.url));
const module = await WebAssembly.compile(readFileSync(`${root}web/rhind.wasm`));
const native = `${root}target/release/rhind`;
let failed = 0;
let n = 0;
const pages = [...JSON.parse(readFileSync(`${root}web/papyrus.json`, "utf8")), ...JSON.parse(readFileSync(`${root}web/examples.json`, "utf8"))];
for (const page of pages) {
  n++;
  const wasm = rhind(module, ["json", "-"], page.text);
  const want = execFileSync(native, ["json", "-"], { input: page.text }).toString();
  if (wasm.out !== want) {
    failed++;
    console.log(`FAIL ${page.name}: the wasm build and the native build differ`);
  }
  JSON.parse(wasm.out);
}
// the language server answers in the page as it does in an editor
const lsp = answer(module, "No. 27\n\\ 1   3 '2\n  2   7\n\\ 4   15\n", [
  { method: "textDocument/semanticTokens/full" },
  { method: "textDocument/hover", params: { position: { line: 3, character: 7 } } },
]);
if (!lsp.results[0]?.data?.length) (failed++, console.log("FAIL lsp: no semantic tokens"));
if (!/14/.test(lsp.results[1]?.contents?.value ?? "")) (failed++, console.log("FAIL lsp: hover does not give 14"));
if (lsp.diagnostics.length !== 1) (failed++, console.log(`FAIL lsp: ${lsp.diagnostics.length} diagnostics, wanted 1`));
// the commands
const w = rhind(module, ["multiply", "3", "'2", "by", "5"]);
if (!/Total 17 2̅\./.test(w.out)) (failed++, console.log("FAIL multiply"));
console.log(`${n} pages through the wasm build, ${failed ? failed + " failed" : "all agree with the native build"}; language server ${failed ? "" : "answers"}`);
process.exit(failed ? 1 : 0);
