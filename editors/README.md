# The Rhind Mathematical Papyrus in your editor

Two pieces give an editor an understanding of a page:

- **`rhind lsp`**, a language server. It is the `rhind` program itself, given the argument `lsp`, and it works with any editor that speaks the Language Server Protocol.
- **A Visual Studio Code extension** in [`vscode`](vscode), which bundles `rhind` and adds syntax highlighting and run buttons. Its [README](vscode/README.md) lists the features.

Both are built on the reconstruction itself: [`src/page.rs`](../src/page.rs) reads the page, [`src/scribe.rs`](../src/scribe.rs) does each computation the way the scribe does it, and [`src/check.rs`](../src/check.rs), the code behind `rhind FILE` and `rhind --check`, says where the scribe's figures and its own disagree. So the editor and `rhind` cannot disagree about what is on the page. The notation is in [`docs/notation.md`](../docs/notation.md).

A `.rmp` file is a page of the scribe's working: headings (`No. 26`, `2 ÷ 7`), the scribe's words in double quotes, statements (`Reckon with 5 to find 15.`), columns of doubling with the scribe's ticks, totals, results and proofs. `{…}` is red ink, `[…]` is restored by an edition, and `%` starts a note to the end of the line; notes are not in the papyrus.

## The language server

```sh
cargo install --git https://github.com/chrishayuk/rhind-papyrus rhind
```

or, in a clone of this repository, `cargo build --release`, which writes `target/release/rhind`.

`rhind lsp` talks on standard input and output. It provides diagnostics, hover, semantic tokens, document symbols, folding ranges and completion.

- **Diagnostics.** A wrong figure of the scribe's is a warning, with the figure the working gives and the slips that would explain it; a line that is not in the notation is an error. A row made in a way the working does not expect (a part taken of the row above, not of the first row) is information, and a figure that cannot be checked is a hint.
- **Hover** on a figure shows its value, how its row is made from the rows before it and whether the working agrees. On a statement, what it does; on a tick, what it means.
- **Semantic tokens** mean an editor with no grammar for the notation still gets colours. The legend is `comment` (notes), `string` (the scribe's words), `keyword` (statements), `number` (figures), `operator` (the tick), `namespace` (headings), `macro` (red ink) and `parameter` (restored, lost or doubtful).
- **Document symbols** and **folding ranges** are the problems and table entries, from each heading to the next.

It also builds for WASI (`cargo build --release --target wasm32-wasip1`, which writes `target/wasm32-wasip1/release/rhind.wasm`). That one file is both the checker and the server, and is how the VS Code extension and the web playground run it.

### Neovim (0.11 or later)

```lua
vim.filetype.add({ extension = { rmp = "rhind" } })
vim.lsp.config("rhind", {
  cmd = { "/path/to/rhind", "lsp" },
  filetypes = { "rhind" },
})
vim.lsp.enable("rhind")
```

Neovim colours from semantic tokens without a grammar. To have `gcc` comment a line, add `vim.bo.commentstring = "% %s"` in an `rhind` filetype autocommand.

### Helix

In `languages.toml`:

```toml
[language-server.rhind]
command = "/path/to/rhind"
args = ["lsp"]

[[language]]
name = "rhind"
scope = "source.rmp"
file-types = ["rmp"]
comment-token = "%"
language-servers = ["rhind"]
```

Helix does not colour from semantic tokens, so without a tree-sitter grammar the page is not highlighted; diagnostics, hover, symbols and completion work.

### Emacs (Eglot)

```elisp
(define-derived-mode rhind-mode text-mode "Rhind"
  (setq-local comment-start "% "))
(add-to-list 'auto-mode-alist '("\\.rmp\\'" . rhind-mode))
(with-eval-after-load 'eglot
  (add-to-list 'eglot-server-programs '(rhind-mode "/path/to/rhind" "lsp")))
```

### Any other LSP client

Start `rhind lsp` for files that end in `.rmp`, with the language id `rhind`. The server wants the whole text on every change (full document sync) and counts positions in UTF-16 code units.

These configurations follow each editor's documented format but have not been run here; the server itself is tested over standard input and output by `vscode/test/lsp.test.mjs`, against both the WebAssembly build and the native program.

## Typing the scribe's figures

A part, one in n, is written with a bar over the number: 1̅5̅ is one fifteenth. A keyboard does not type that, so `'15` is accepted for it, and `2/3` or `''3` for ⅔. `rhind` prints the overbarred form (each digit followed by U+0305, COMBINING OVERLINE) and reads either.

## Checking a page from the command line

```sh
rhind page.rmp
```

prints the page with a ✓ or ✗ against every figure the scribe wrote, and under it what is wrong and how the scribe could have come to write it.

```sh
rhind --check page.rmp
```

prints one line per problem, as `file:line:column: severity: message`, and exits with status 1 if a line is not in the notation. The severities are `wrong`, `form`, `unchecked` and `error`.
