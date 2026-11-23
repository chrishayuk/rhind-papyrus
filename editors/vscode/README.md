# The Rhind Mathematical Papyrus for Visual Studio Code

Editor support for pages of the Rhind Mathematical Papyrus (c. 1550 BCE), the scribe Ahmose's copy of an older book of problems in arithmetic, written down so that they can be run. A `.rmp` file is a page of the scribe's working, in the papyrus's own layout: headings, the scribe's words, columns of doubling with the scribe's ticks, totals, results and proofs. `rhind` does every computation the way the scribe does it and says where the scribe's figures and its own disagree. The notation is described in [`docs/notation.md`](../../docs/notation.md).

Nothing else needs installing: `rhind`, which is both the checker and the language server, is built into the extension as WebAssembly.

## What you get

- **Colours** for headings (`No. 26`, `2 ÷ 7`), the scribe's words in quotes, the statements (`Multiply … by …`, `Reckon with … to find …`, `Total`, `Result`, `Remainder`, `Add … to …`, `Take … from …`, `Proof`), the tick `\`, figures in every spelling (`5`, `'15` or 1̅5̅, `2/3` or ⅔), units, red ink `{…}`, restorations `[…]`, `[...]` for what is lost, `?` for a doubtful reading, and `%` notes. The language server colours the page too, from what it has read, and its colours take the place of the grammar's.
- **Problems as you type.** A wrong figure of the scribe's is a warning that says what the working gives and which slips would explain it: `4 of 3 2̅ is 14; the papyrus has 15`, `could be one-sign: 5 written for 4`. A line that is not in the notation is an error.
- **Hover** on a figure to see its value (`4̅ = 1/4`), how its row is made from the rows before it, and whether the working agrees: ✓, or ✗ with the figure it gives and whether the error is carried into what follows. On a statement, what it does; on a tick, that the row is one of those added to make the answer; on a heading of the 2 ÷ n table, the table's entry.
- **The outline** of a page is its problems and table entries, and each folds from its heading to the next.
- **Completion** of the statements, with an example of each, and the units.
- **Snippets** `no`, `multiply`, `reckon`, `column`, `proof`, `table` and `twothirds` (⅔).
- **Run This Page:** the play button at the top of the editor, or Cmd+Enter (Ctrl+Enter on Windows and Linux). The page appears in a terminal with a ✓ or ✗ against every figure the scribe wrote, the working `rhind` writes in where the page leaves it out, and under it what is wrong. **Check This Page** prints a line for each problem, as `file:line:column: severity: message`, each a link to the place; **Show the Page in the Scribe's Figures** prints the page without marks, with every part overbarred.

## Settings

| Setting | What it does |
| --- | --- |
| `rhind.path` | Use an `rhind` program you built yourself, both as the language server (`rhind lsp`) and to run pages. |
| `rhind.trace.server` | Log the messages exchanged with the language server. |

## Building it

From this folder, with Rust (and its `wasm32-wasip1` target) and Node installed:

```sh
npm install
npm run build        # compiles rhind to WebAssembly, bundles the extension
npm test             # grammar, language server and runner tests
npm run test:vscode  # opens a real VS Code and drives the extension
npm run package      # writes rhind-language-0.1.2.vsix
code --install-extension rhind-language-0.1.2.vsix
```

To try it without installing, open this folder in VS Code and press F5.

## How it fits together

| File | What it is |
| --- | --- |
| `syntaxes/rhind.tmLanguage.json` | The highlighting grammar, used until the language server's colours arrive. |
| `language-configuration.json` | The note character, the braces of red ink and the brackets of restorations. |
| `snippets/rhind.code-snippets` | A problem, the statements, a column, a proof, a table entry and ⅔. |
| `src/extension.js` | Starts the language server and provides the run commands. |
| `server/rhind.wasm` | `rhind`, compiled for WASI: one program for running pages and, as `rhind lsp`, the language server. |
| `server/lsp.js` | Starts `rhind lsp`. |
| `server/run.js` | Runs a page: `rhind FILE`, `rhind --check FILE` or `rhind --plain FILE`, with the page given on standard input. |
| `server/wasi.js` | A small WASI host for Node, with no dependencies. |

## Licence

Apache License 2.0. See [LICENSE](LICENSE).
