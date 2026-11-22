// Builds the files the playground needs:  node web/build.mjs
//   web/rhind.wasm     rhind, compiled for WASI: the checker and the language server (`rhind lsp`)
//   web/papyrus.json   every page of the papyrus as transcribed, in papyrus order
//   web/examples.json  the examples written for this repository
import { execSync } from "node:child_process";
import { copyFileSync, existsSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const root = fileURLToPath(new URL("..", import.meta.url));
execSync("cargo build --release --target wasm32-wasip1", { cwd: root, stdio: "inherit" });
copyFileSync(`${root}target/wasm32-wasip1/release/rhind.wasm`, `${root}web/rhind.wasm`);

// papyrus order: the tables, then the problems by number
const key = (name) => {
  if (name.startsWith("table-")) return [0, name.startsWith("table-2n") ? 0 : 1, name];
  const id = name.replace(/^no-/, "");
  return [1, parseInt(id, 10), id.replace(/^\d+/, "")];
};
const cmp = (a, b) => {
  const [x, y] = [key(a), key(b)];
  for (let i = 0; i < 3; i++) if (x[i] !== y[i]) return x[i] < y[i] ? -1 : 1;
  return 0;
};

const read = (dir, ext) =>
  existsSync(`${root}${dir}`)
    ? readdirSync(`${root}${dir}`).filter((f) => f.endsWith(ext)).map((f) => f.slice(0, -ext.length))
    : [];

const label = (name) => {
  if (name === "table-2n-a") return "2 ÷ 3 to 2 ÷ 49";
  if (name === "table-2n-b") return "2 ÷ 51 to 2 ÷ 101";
  if (name === "table-n10") return "1 to 9 ÷ 10";
  return "No. " + name.replace(/^no-0?/, "").toUpperCase();
};

const pages = read("papyrus", ".rmp").sort(cmp).map((name) => {
  const text = readFileSync(`${root}papyrus/${name}.rmp`, "utf8");
  const words = /^"([^"]*)"/m.exec(text);
  const about = (/^% (.*)$/m.exec(text) || [, ""])[1];
  return { name, label: label(name), text, about, words: words ? words[1] : "" };
});
writeFileSync(`${root}web/papyrus.json`, JSON.stringify(pages));

const examples = read("examples", ".rmp").sort().map((name) => {
  const text = readFileSync(`${root}examples/${name}.rmp`, "utf8");
  const notes = [];
  for (const line of text.split("\n")) {
    if (!line.startsWith("%")) break;
    notes.push(line.replace(/^%\s?/, ""));
  }
  return { name, text, about: notes.join(" ").trim() };
});
writeFileSync(`${root}web/examples.json`, JSON.stringify(examples));
console.log(`web/rhind.wasm, web/papyrus.json (${pages.length} pages), web/examples.json (${examples.length}) written`);
