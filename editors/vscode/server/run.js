// Runs a page of the papyrus:  node run.js [options] page.rmp
// The page is given to rhind on standard input (as the file `-`), so it needs
// no access to files, and it prints the page with every figure checked.
// Options are those of `rhind` (--check, --plain).
"use strict";

const fs = require("fs");
const path = require("path");
const { run } = require("./wasi.js");

const args = process.argv.slice(2);
const file = args.pop();
if (!file || file.startsWith("--")) {
  console.error("usage: run.js [options] page.rmp");
  process.exit(2);
}
let page;
try {
  page = fs.readFileSync(file);
} catch (error) {
  console.error(`rhind: cannot read ${file}: ${error.message}`);
  process.exit(2);
}

const wasm = path.join(__dirname, "rhind.wasm");
if (args.includes("--check")) {
  // rhind names the file it read, and here that is `-`: put the real name
  // back, so each line reads path:line:column and the terminal links it.
  const out = [];
  const code = run(wasm, {
    args: [...args, "-"],
    prefix: page,
    keyboard: false,
    output: (fd, bytes) => (fd === 1 ? out.push(Buffer.from(bytes)) : process.stderr.write(bytes)),
  });
  process.stdout.write(Buffer.concat(out).toString().replace(/^-:/gm, `${file}:`));
  process.exitCode = code;
} else {
  process.exitCode = run(wasm, { args: [...args, "-"], prefix: page, keyboard: false });
}
