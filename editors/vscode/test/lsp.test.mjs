// Talks to the language server the way an editor does: over standard input
// and output. It tests the WebAssembly build the extension ships and, when
// it has been built, the native program other editors would use.
import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { existsSync } from "node:fs";
import test from "node:test";
import { fileURLToPath } from "node:url";

const here = fileURLToPath(new URL("..", import.meta.url));
const native = fileURLToPath(new URL("../../../target/release/rhind", import.meta.url));
const servers = [["WebAssembly", process.execPath, [`${here}server/lsp.js`, "--stdio"]]];
if (existsSync(native)) servers.push(["native", native, ["lsp"]]);

function connect(command, args) {
  const child = spawn(command, args, { stdio: ["pipe", "pipe", "inherit"] });
  let buffer = Buffer.alloc(0);
  const waiting = [];
  const arrived = [];
  child.stdout.on("data", (chunk) => {
    buffer = Buffer.concat([buffer, chunk]);
    for (;;) {
      const header = buffer.indexOf("\r\n\r\n");
      if (header === -1) return;
      const length = Number(/Content-Length: (\d+)/i.exec(buffer.subarray(0, header).toString())[1]);
      if (buffer.length < header + 4 + length) return;
      const message = JSON.parse(buffer.subarray(header + 4, header + 4 + length).toString());
      buffer = buffer.subarray(header + 4 + length);
      const next = waiting.shift();
      if (next) next(message);
      else arrived.push(message);
    }
  });
  const send = (message) => {
    const body = JSON.stringify({ jsonrpc: "2.0", ...message });
    child.stdin.write(`Content-Length: ${Buffer.byteLength(body)}\r\n\r\n${body}`);
  };
  const receive = () => new Promise((resolve) => (arrived.length ? resolve(arrived.shift()) : waiting.push(resolve)));
  let id = 0;
  const request = async (method, params) => {
    send({ id: ++id, method, params });
    const reply = await receive();
    assert.equal(reply.id, id);
    return reply.result;
  };
  const exited = new Promise((resolve) => child.on("exit", resolve));
  return { send, receive, request, exited, kill: () => child.kill() };
}

const uri = "file:///no-26.rmp";
// Problem 26, with a figure of the multiplication changed from 12 to 13.
const page = `% Problem 26.
No. 26
"A quantity whose fourth part is added to it becomes 15."
  1   4
 '4   1
Total 5.
Reckon with 5 to find 15.
\\ 1   5
\\ 2   10
Result 3.
Multiply 3 by 4.
  1   3
  2   6
\\ 4   13
Result 12.

  1   4
{2̅0̅}  [5]   % red, overbarred, restored
`;

/** Semantic tokens as [line, column, length, type name]. */
function decode(data, legend) {
  const out = [];
  let line = 0;
  let column = 0;
  for (let i = 0; i < data.length; i += 5) {
    line += data[i];
    column = data[i] ? data[i + 1] : column + data[i + 1];
    out.push([line, column, data[i + 2], legend[data[i + 3]]]);
  }
  return out;
}

for (const [name, command, args] of servers) {
  test(`${name} server: a session from initialize to exit`, async (t) => {
    const server = connect(command, args);
    // A failed assertion must not leave the server running, or the test never ends.
    t.after(() => server.kill());
    const initialised = await server.request("initialize", { processId: null, rootUri: null, capabilities: {} });
    assert.equal(initialised.serverInfo.name, "rhind");
    const capabilities = initialised.capabilities;
    assert.equal(capabilities.hoverProvider, true);
    assert.equal(capabilities.documentSymbolProvider, true);
    assert.equal(capabilities.foldingRangeProvider, true);
    const legend = capabilities.semanticTokensProvider.legend.tokenTypes;
    assert.deepEqual(legend, ["comment", "string", "keyword", "number", "operator", "namespace", "macro", "parameter"]);
    server.send({ method: "initialized", params: {} });

    server.send({ method: "textDocument/didOpen", params: { textDocument: { uri, languageId: "rhind", version: 1, text: page } } });
    const opened = await server.receive();
    assert.equal(opened.method, "textDocument/publishDiagnostics");
    // the 13 is a wrong figure: a warning, where it is, and how it could have happened
    const wrong = opened.params.diagnostics.filter((d) => d.severity === 2);
    assert.equal(wrong.length, 1);
    assert.deepEqual(wrong[0].range, { start: { line: 13, character: 6 }, end: { line: 13, character: 8 } });
    assert.match(wrong[0].message, /4 of 3 is 12; the papyrus has 13/);
    assert.match(wrong[0].message, /could be/);
    assert.equal(wrong[0].source, "rhind");
    assert.deepEqual(opened.params.diagnostics.filter((d) => d.severity === 1), []);

    const at = (line, character) => ({ textDocument: { uri }, position: { line, character } });
    const hover = async (line, character) => (await server.request("textDocument/hover", at(line, character))).contents.value;
    assert.match(await hover(1, 2), /Problem 26/);
    assert.match(await hover(2, 5), /scribe's words/);
    assert.match(await hover(6, 3), /`Reckon with A to find B\.`/);
    assert.match(await hover(7, 0), /tick/);
    assert.match(await hover(13, 6), /✗ the working gives \*\*12\*\*/);
    assert.match(await hover(4, 1), /1\/4/);
    assert.match(await hover(0, 3), /A note, not in the papyrus/);

    const { data } = await server.request("textDocument/semanticTokens/full", { textDocument: { uri } });
    const tokens = decode(data, legend);
    const kinds = (line) => tokens.filter((k) => k[0] === line).map((k) => k[3]);
    assert.deepEqual(kinds(0), ["comment"]);
    assert.deepEqual(kinds(1), ["namespace"]);
    assert.deepEqual(kinds(2), ["string"]);
    assert.deepEqual(kinds(6), ["keyword", "number", "keyword", "number"]);
    assert.deepEqual(kinds(7), ["operator", "number", "number"]);
    assert.deepEqual(kinds(17), ["macro", "parameter", "comment"]);
    // after the overbars (one UTF-16 unit each) the columns still line up
    assert.deepEqual(tokens.find((k) => k[0] === 17 && k[3] === "parameter").slice(1, 3), [8, 3]);

    const symbols = await server.request("textDocument/documentSymbol", { textDocument: { uri } });
    assert.deepEqual(symbols.map((s) => s.name), ["No. 26"]);
    const folds = await server.request("textDocument/foldingRange", { textDocument: { uri } });
    assert.equal(folds[0].startLine, 1);
    const completion = await server.request("textDocument/completion", at(18, 0));
    const labels = completion.map((c) => c.label);
    assert.ok(labels.includes("Reckon with 5 to find 15."));
    assert.ok(labels.includes("hekat"));

    server.send({ method: "textDocument/didChange", params: { textDocument: { uri, version: 2 }, contentChanges: [{ text: "No. 1\nFrobnicate 3.\n" }] } });
    const broken = await server.receive();
    assert.equal(broken.params.diagnostics[0].severity, 1);
    assert.equal(broken.params.diagnostics[0].range.start.line, 1);

    assert.equal(await server.request("shutdown", null), null);
    server.send({ method: "exit" });
    assert.equal(await server.exited, 0);
  });
}
