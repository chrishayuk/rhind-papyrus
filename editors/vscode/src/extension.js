// The Rhind Mathematical Papyrus extension: starts the language server
// (`rhind lsp`) and adds commands that run the page in the editor and show
// it with every figure the scribe wrote checked.
"use strict";

const vscode = require("vscode");
const fs = require("fs");
const os = require("os");
const path = require("path");
const { LanguageClient, TransportKind } = require("vscode-languageclient/node");

/** @type {LanguageClient | undefined} */
let client;

function setting(name) {
  const value = vscode.workspace.getConfiguration("rhind").get(name);
  return typeof value === "string" ? value.trim() : value;
}

async function startServer(context) {
  const custom = setting("path");
  // Built in: rhind compiled to WebAssembly, run by the editor's own Node,
  // so nothing has to be installed.
  const builtIn = { module: context.asAbsolutePath(path.join("server", "lsp.js")), transport: TransportKind.stdio };
  const serverOptions = custom ? { command: custom, args: ["lsp"] } : { run: builtIn, debug: builtIn };
  client = new LanguageClient("rhind", "Rhind Mathematical Papyrus", serverOptions, {
    documentSelector: [{ language: "rhind" }],
  });
  await client.start();
}

async function restartServer(context) {
  if (client) await client.stop().catch(() => {});
  client = undefined;
  await startServer(context);
}

async function runPage(context, options, title) {
  const editor = vscode.window.activeTextEditor;
  if (!editor || editor.document.languageId !== "rhind") {
    vscode.window.showInformationMessage("Open a page of the papyrus (.rmp) to run it.");
    return;
  }
  const document = editor.document;
  let file = document.uri.fsPath;
  if (document.isUntitled || document.uri.scheme !== "file") {
    // Not on disk yet: run a copy.
    file = path.join(os.tmpdir(), "untitled.rmp");
    fs.writeFileSync(file, document.getText());
  } else if (document.isDirty) {
    await document.save();
  }

  const custom = setting("path");
  const execution = custom
    ? new vscode.ProcessExecution(custom, [...options, file])
    : new vscode.ProcessExecution(process.execPath, [context.asAbsolutePath(path.join("server", "run.js")), ...options, file], {
        env: { ELECTRON_RUN_AS_NODE: "1" },
      });
  const task = new vscode.Task({ type: "rhind" }, vscode.TaskScope.Workspace, `${title} ${path.basename(file)}`, "rhind", execution, []);
  task.presentationOptions = {
    reveal: vscode.TaskRevealKind.Always,
    panel: vscode.TaskPanelKind.Dedicated,
    focus: true,
    clear: true,
    // Without this the terminal first prints the whole command line.
    echo: false,
    showReuseMessage: false,
  };
  await vscode.tasks.executeTask(task);
}

async function activate(context) {
  context.subscriptions.push(
    vscode.commands.registerCommand("rhind.run", () => runPage(context, [], "Run")),
    vscode.commands.registerCommand("rhind.check", () => runPage(context, ["--check"], "Check")),
    vscode.commands.registerCommand("rhind.plain", () => runPage(context, ["--plain"], "Page")),
    vscode.commands.registerCommand("rhind.restartServer", () => restartServer(context)),
    vscode.workspace.onDidChangeConfiguration((event) => {
      if (event.affectsConfiguration("rhind.path")) restartServer(context);
    })
  );
  await startServer(context);
}

function deactivate() {
  return client ? client.stop() : undefined;
}

module.exports = { activate, deactivate };
