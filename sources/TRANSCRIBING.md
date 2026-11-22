# How the papyrus was transcribed

This is the brief every transcriber worked to. It is kept so that the
transcriptions can be checked against it.

## The witnesses

The papyrus itself (British Museum EA 10057 and 10058) was not read. Two
editions were, both public domain and on the Internet Archive (see
`README.md` here):

1. **Chace** (primary): A. B. Chace, L. Bull, H. P. Manning, *The Rhind
   Mathematical Papyrus*, vol. II, 1929: `pdf/chace-1929-vol2.pdf`. For each
   problem an odd-numbered PDF page has the photograph's reference, the
   transliteration and a **literal translation that keeps the papyrus's
   layout, with every number in Arabic figures** (unit fractions with a bar
   over them, red ink noted). The facing even page is the **plate**: hieratic
   and a hieroglyphic transcription, with red ink printed in red and each
   number glossed in small Arabic figures beneath. Read both.
2. **Peet** (second witness): T. E. Peet, *The Rhind Mathematical Papyrus*,
   1923: `pdf/peet-1923.pdf` (a 1970 reprint; same pages). A translation with
   the working set out in columns, ticks as `✓`-like marks, and notes that
   say where the scribe went wrong (*sic*). PDF page = printed page + 4 (check
   it near your range; it can drift at plates).

Page maps: `pdf/` page numbers for Chace vol. II:

| PDF pages | content |
|---|---|
| 51-112 | 2 ÷ 3 to 2 ÷ 99 (odd page = literal translation, even = plate) |
| 113-114 | 2 ÷ 101; the numbers 1-9 divided by 10 |
| 115-266 | Problems 1-87 (each problem's literal translation, then its plate) |

Render a page with `pdftoppm -f N -l N -r 200 -png -singlefile FILE OUT` into
your own scratch directory, and crop or render at higher resolution (300) to
read small figures. Never guess a figure from OCR: the OCR destroys the
fraction bars. Read the images.

## What to write

One file per problem, `papyrus/no-NN.rmp` (`no-01.rmp`, `no-07b.rmp`,
`no-59b.rmp`), or for the tables `papyrus/table-2n.rmp` and
`papyrus/table-n10.rmp`, in the notation of `../docs/notation.md`. Read that
first, all of it.

The rules that matter most:

- **Write what the papyrus has, not what is right.** A wrong figure is
  copied as it stands. Never correct, never "normalise" a number.
- Keep the papyrus's layout: its order of rows, its ticks, its red ink
  (`{…}`), its totals. Where Chace marks a number red, put it in braces.
- Damage: `[…]` around what Chace restores, `[...]` where nothing is
  restored, `?` after a doubtful reading.
- Where Chace and Peet disagree about a figure, a tick, a restoration or the
  order, follow Chace in the file and add a `%` note on that line:
  `% Peet: 14`. List every such disagreement in your notes file too.
- Where Peet or Chace says the scribe erred (*sic*, "error", "should be"),
  add a `%` note quoting it briefly: `% Peet: (sic); Chace: "should be 14"`.
- Words: the papyrus's sentences go in double quotes, from Chace's literal
  translation. Turn a sentence into a statement (`Reckon with 5 to find 15.`,
  `Multiply 3 '2 by 5.`, `Total 21.`, `Result 3.`, `Proof.`) only where it
  says exactly that; otherwise keep it as quoted words.
- First line of each file: a `%` note with the problem, Chace's plate and PDF
  pages, and Peet's printed page(s).
- Geometry, hekat measures and the problems with several columns: do your
  best inside the notation, and say in the notes what did not fit. A row
  with three or more numbers is allowed.

## Notes

Write `sources/notes/<your range>.md`: for each problem, what was read with
confidence, what was not, every disagreement between Chace and Peet, every
error the editions point out, and anything the notation could not hold.

All reading was done by a coding agent's sub-agents. Say so; do not claim a
person checked anything.
