// Starts the language server: rhind compiled for WASI, run as `rhind lsp`,
// talking the Language Server Protocol on standard input and output.
"use strict";

const path = require("path");
const { run } = require("./wasi.js");

process.exitCode = run(path.join(__dirname, "rhind.wasm"), { args: ["lsp"] });
