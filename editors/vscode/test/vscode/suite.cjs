// Runs inside VS Code. Exercises the extension the way a person would: open
// a page of the papyrus, hover, see its problems, outline and colours, and
// run it.
"use strict";

const assert = require("node:assert/strict");
const path = require("node:path");
const vscode = require("vscode");

const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

async function until(what, probe, seconds = 20) {
  for (let i = 0; i < seconds * 10; i++) {
    const value = await probe();
    if (value) return value;
    await sleep(100);
  }
  throw new Error(`timed out waiting for ${what}`);
}

const hoverText = (hovers) =>
  hovers.flatMap((h) => h.contents).map((c) => (typeof c === "string" ? c : c.value)).join("\n");

/** Runs a command that starts an rhind task and returns the task's exit code. */
async function taskExit(command) {
  const finished = new Promise((resolve) => {
    const listener = vscode.tasks.onDidEndTaskProcess((event) => {
      if (event.execution.task.definition.type === "rhind") {
        listener.dispose();
        resolve(event.exitCode);
      }
    });
  });
  await vscode.commands.executeCommand(command);
  return Promise.race([finished, sleep(20000).then(() => "timed out")]);
}

exports.run = async function run() {
  const folder = vscode.workspace.workspaceFolders[0].uri.fsPath;
  const document = await vscode.workspace.openTextDocument(path.join(folder, "no-41.rmp"));
  await vscode.window.showTextDocument(document);
  assert.equal(document.languageId, "rhind", "the .rmp extension selects the language");

  const line = (text) => {
    for (let i = 0; i < document.lineCount; i++) if (document.lineAt(i).text.startsWith(text)) return i;
    throw new Error(`no line ${text}`);
  };

  // The server is ready when it answers a hover.
  const heading = new vscode.Position(line("No. 41"), 2);
  const hovers = await until("a hover from the language server", async () => {
    const found = await vscode.commands.executeCommand("vscode.executeHoverProvider", document.uri, heading);
    return found.length ? found : undefined;
  });
  assert.match(hoverText(hovers), /Problem 41/);
  const tick = await vscode.commands.executeCommand("vscode.executeHoverProvider", document.uri, new vscode.Position(line("\\ 8   64"), 0));
  assert.match(hoverText(tick), /tick/);
  console.log("hover ok");

  const legend = await vscode.commands.executeCommand("vscode.provideDocumentSemanticTokensLegend", document.uri);
  assert.deepEqual(legend.tokenTypes, ["comment", "string", "keyword", "number", "operator", "namespace", "macro", "parameter"]);
  const tokens = await vscode.commands.executeCommand("vscode.provideDocumentSemanticTokens", document.uri);
  assert.ok(tokens.data.length > 0);
  const symbols = await vscode.commands.executeCommand("vscode.executeDocumentSymbolProvider", document.uri);
  assert.deepEqual(symbols.map((s) => s.name), ["No. 41"]);
  const folds = await vscode.commands.executeCommand("vscode.executeFoldingRangeProvider", document.uri);
  assert.ok(folds.some((f) => f.start === line("No. 41")));
  console.log("symbols, folding and semantic tokens ok");

  // A page with a wrong figure and a line that is not in the notation.
  const broken = await vscode.workspace.openTextDocument({ language: "rhind", content: "No. 27\n\\ 1   3 '2\n  2   7\n\\ 4   15\nFrobnicate 3.\n" });
  await vscode.window.showTextDocument(broken);
  const problems = await until("diagnostics", () => {
    const found = vscode.languages.getDiagnostics(broken.uri);
    return found.length >= 2 ? found : undefined;
  });
  const wrong = problems.find((p) => p.severity === vscode.DiagnosticSeverity.Warning);
  assert.match(wrong.message, /the papyrus has 15/);
  assert.equal(wrong.range.start.line, 3);
  const error = problems.find((p) => p.severity === vscode.DiagnosticSeverity.Error);
  assert.equal(error.source, "rhind");
  assert.equal(error.range.start.line, 4);
  console.log("diagnostics ok");

  const completion = await vscode.commands.executeCommand("vscode.executeCompletionItemProvider", broken.uri, new vscode.Position(5, 0));
  const labels = completion.items.map((i) => (typeof i.label === "string" ? i.label : i.label.label));
  assert.ok(labels.includes("Reckon with 5 to find 15."));
  console.log("completion ok");

  // Run This Page and Check This Page: tasks that end with exit code 0.
  await vscode.window.showTextDocument(document);
  assert.equal(await taskExit("rhind.run"), 0);
  console.log("run ok");
  assert.equal(await taskExit("rhind.check"), 0);
  console.log("check ok");
};
