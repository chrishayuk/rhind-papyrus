# rhind

The Rhind Mathematical Papyrus, run. Every figure the scribe Ahmose wrote in
its working, about 1550 BCE, worked again with the operations he had, and
compared, exactly, with what he wrote.

```
$ rhind no 27
% Problem 27. Chace vol. II pl. 50 (photograph XII, register 4), PDF pp. 147-148; Peet pp. 62-63 (Pl. J).
No. 27
"{A quantity, '5 of it added to it,} becomes it: 21."
  1      5                              ✓
  5̅     1                               ✓
Total 6.                                ✓
% 6 is then reckoned with to find 21, with no sentence
\ 1      6                              ✓
\ 2      12                             ✓
\ 2̅     3                               ✓
Total 21.                               ✓
% the ticked multipliers give 3 '2, which is then multiplied by 5
\ 1      3 2̅                            ✓
  2      7                              ✓
\ 4      15             % Peet: "15 (sic)", "15 has been erroneously written for 14"; Chace prints 15 without comment  ✗ 14
% no total written
"{The quantity is 17 '2,} '5 of it is 3 '2, {the total is 21.}"

10 figures checked, 9 right, 1 wrong.
line 15: 4 of 3 2̅ is 14; the papyrus has 15
    could be one-sign: 5 written for 4
```

The papyrus has no division and no fractions but parts of one. To divide,
the scribe doubles the divisor until the rows can be picked to make the
dividend. To double an odd part, he looks it up in the table of 2 divided by
odd numbers at the head of the papyrus. `rhind` does the same, and has no
other way to do it. So when it reaches a different figure from the scribe's,
the difference is in the figure, not in the method. It then asks what slip
of the pen would turn the right figure into the one written.

This is not a calculator for Egyptian fractions. It is a reading of one
document: what the scribe wrote, what his working gives, and where they part.

## Install

You need [Rust](https://rustup.rs):

```bash
cargo install --git https://github.com/chrishayuk/rhind-papyrus
```

That puts `rhind` in `~/.cargo/bin`. The whole papyrus, as transcribed, is
compiled into it:

```bash
rhind no 26          # a problem of the papyrus, checked
rhind verify         # the whole papyrus
rhind errata         # every wrong figure, and the slips that would make it
```

For Visual Studio Code, download `rhind-language-0.1.0.vsix` from the
[latest release](https://github.com/chrishayuk/rhind-papyrus/releases/latest)
and run `code --install-extension rhind-language-0.1.0.vsix`: colours, the
scribe's wrong figures as problems while you type, hover help that says how
each row is made, and a run button. The extension carries its own copy of
`rhind`. For Neovim, Helix and
Emacs see [`editors/README.md`](editors/README.md). Or open the
[playground](https://chrishayuk.com/demos/rhind-papyrus), which runs the same
program in the page.

## Using it

```
rhind FILE                 the page, with every figure the scribe wrote checked
rhind no 27                a problem of the papyrus (also: rhind no 7B, rhind table)
rhind --plain FILE         the page in the scribe's figures, without marks
rhind --check FILE         what is wrong with it, a line each
rhind verify               run the whole papyrus and compare with the editions
rhind errata               every wrong figure, the slips that fit it, and the editions' notes
rhind routes               does the scribe set out each column the way rhind does?
rhind 2/n                  how were the entries of the 2 ÷ n table chosen? rules tested against the papyrus
rhind multiply 3 '2 by 5   the working, as the scribe would write it
rhind reckon with 6 to find 21
rhind double '7            (also halve, thirds)
rhind decisions            the choices this reconstruction makes that the papyrus does not
rhind lsp                  a language server on standard input and output
```

## The notation

A `.rmp` file is a page of the scribe's working, in the papyrus's own
layout. [`docs/notation.md`](docs/notation.md) has all of it. This is
Problem 26, with its first step set out as a column:

```
No. 26
"A quantity whose fourth part is added to it becomes 15."
  1   4
 '4   1
Total 5.
Reckon with 5 to find 15.
\ 1   5
\ 2   10
Result 3.
```

`'4` is a quarter, written 4̅: the Egyptian sign for "part" over the number.
`2/3` is two-thirds, ⅔, the one fraction with a sign of its own. A row is the
multiplier and the product; `\` is the scribe's tick against the rows to be
added. `{…}` is red ink, `[…]` a restoration, `%` a note. A `Multiply` or
`Reckon` with no working under it is worked by `rhind`, the scribe's way.

A transcription never corrects the scribe. Where the papyrus has 15, the
file has 15. Where the scribe left the sign for "part" off a number, so that
3̅0̅ stands as 30, the file has 30. Finding these is `rhind`'s job, not the
transcriber's.

## What running it found

The papyrus as transcribed has 91 problem sections and the two tables.
`rhind verify` checks 965 lines of working: 838 are right and 127 have
something wrong. In those 127 lines are 133 wrong figures. Chace or Peet
points out 87 of them; 46 are not pointed out by either edition, as
transcribed here. Each of those 46 is a question for the photographs. Most
will be things the editions corrected silently, or misreadings by this
project, before any is something new.

These counts are against this project's transcription, not the papyrus, and
the transcription was made by a coding agent and its sub-agents (see
"Reading the papyrus").

### The slips

For each wrong figure `rhind errata` tries models of how a figure goes
wrong, and reports every one that turns the right figure into the written
one. The first that fits:

| slip | figures | what it is |
|---|---|---|
| `part-sign` | 71 | the sign for "part" left off, so 3̅0̅ is written 30 |
| `one-sign` | 9 | one hieratic number sign written for another: 15 for 14 |
| `ticks` | 6 | the total is right for other rows than those ticked |
| `multiplier` | 4 | the row is right for a multiplier one sign away from the one written |
| `in-the-list` | 3 | a list's total is right and one figure in it is wrong |
| `wrong-row`, `neighbour`, `one-part` | 7 | the wrong row doubled or halved; the next row's figure copied; a fraction dropped |
| none | 33 | no single slip of these kinds |

The commonest slip, by far, is the sign for "part" left off a number. The
editions say so case by case ("fractional dot omitted"); run as a whole, it
is about half of everything that goes wrong.

The second is which signs are confused. In hieratic each of 1-9, 10-90 and
100-900 has a sign of its own, so a one-sign slip is one sign written for
another. The pair that recurs is 60 for 80: 6̅6̅ for 8̅6̅ in 2 ÷ 43, 63 for 83
in 2 ÷ 83, and 1̅6̅6̅ for 1̅8̅6̅ in 2 ÷ 93, five places in all. Chace noticed
this: his note to 2 ÷ 83 refers to the one on 2 ÷ 43. What `rhind` adds is
that it finds all of them by working the rows, with no other help, and that
the tally sets them beside every other slip in the papyrus: no other pair of
signs is confused more than twice.

Whether an error is carried tells which figure the scribe was working from.
Where a wrong row has a total or result after it, `rhind` can say whether the
total was made from the wrong figure or from the right one. Of the 38 wrong
rows where it can tell, 35 are not carried: the total is right, so the
working was right and the figure was miswritten. In Problem 27 no total is
written, but the answer the scribe gives in words, 17 2̅, is 3 2̅ and 14, not
15. These are slips of the pen more than of the arithmetic, which fits
Ahmose's own description of the papyrus as a copy.

Some places where the slip model and the editions meet:

- **Problem 63.** Four shares are listed and totalled, 700. They add to 680.
  `rhind` asks which one figure, changed by one slip, makes the total right.
  Four could, but only one correction is written elsewhere on the page:
  113 3̅ for 133 3̅, which is the result of `Multiply 400 by 3̅` a few lines
  up. Chace: "Mistake for 133."
- **Problem 65.** Ten shares totalled 100. The one that is wrong is 7 ⅔ 39
  for 7 ⅔ 3̅9̅, which is again on the page, two lines up. Chace: "The scribe
  omitted the fractional dot."
- **Problem 35.** The scribe's total 1 is right for two rows he did not tick.
  Peet: *(sic)*. `rhind` names the two rows.
- **2 ÷ 59.** The red answer has 2̅2̅6̅; the working has 2̅3̅6̅, and 4 × 59 is
  236. `rhind` says the figure is right for a multiplier one sign away: 20
  written for 30. Neither edition comments. Chace prints 226; Peet prints
  236, perhaps silently corrected. The transcriber of that range read 226 on
  Chace's plate independently, at 400 dpi, before `rhind` could run. This is
  the place most worth checking against the British Museum photograph.

### The routes

Doing the arithmetic right is one thing; setting it out the scribe's way is
another. For every column the problems write in full, `rhind routes`
works the same multiplication or division by its own procedure and compares
the rows:

| | columns |
|---|---|
| the same rows and ticks, by doubling | 40 |
| the same, with the tens done by a row for 10 | 3 |
| the same, taking 3̅ straight from the first row | 2 |
| the same, read as a division (halving down to the remainder) | 2 |
| the same, with rows written that are not used | 1 |
| the scribe uses 10 another way | 5 |
| the scribe goes by ⅔ where `rhind` halves, or the reverse | 6 |
| another route | 6 |
| **in all** | **65** |

Doubling alone gives 40 of 65 columns row for row. Three more conventions,
each visible in the papyrus, bring it to 47. The rest are where the scribe
chose differently, and they are listed by `rhind routes --all`.

### The table of 2 ÷ n

Why does the papyrus write 2/13 as 8̅ 5̅2̅ 1̅0̅4̅, when 7̅ 9̅1̅ has fewer
parts? This has been asked since the 1870s. `rhind 2/n` runs candidate rules
for every odd n from 5 to 101 and compares them with all 49 entries:

| rule | entries |
|---|---|
| the largest part that fits, again and again | 4 of 49 |
| fewest parts, then the smallest last part | 25 |
| fewest parts, then fewest odd parts | 29 |
| 2/n = m̅ + (2m − n)/mn, with 2m − n made of divisors of m, fewest parts | 32 |
| the same divisor method, smallest last part first | 34 |

The divisor method is what the scribe's red auxiliary numbers look like, and
it reproduces most entries. No rule tried here gives 2 ÷ 13 or 2 ÷ 95. The
rules are named for what they do. Scholars have proposed rules of this kind
since the 1870s, and none here is claimed to be any one scholar's, or new.

## Reading the papyrus

The papyrus itself was not read. Two editions were, both public domain and
on the Internet Archive:

- A. B. Chace, L. Bull, H. P. Manning, *The Rhind Mathematical Papyrus*,
  vol. II, Mathematical Association of America, 1929: photographs, the
  hieratic, a hieroglyphic transcription with red ink in red and every
  number glossed, and a literal translation in the papyrus's layout. This is
  the primary witness.
- T. E. Peet, *The Rhind Mathematical Papyrus, British Museum 10057 and
  10058*, 1923: translation and commentary, with the working in columns and
  the scribe's errors marked *(sic)*. The second witness.

Every page was read from images of both, not from OCR: the OCR destroys the
fraction bars, and the bars are the point. Where the editions disagree, the
file follows Chace and has a note on the line. The notes on each range,
with every disagreement, are in [`sources/notes/`](sources/notes/). The
brief the transcribers worked to is
[`sources/TRANSCRIBING.md`](sources/TRANSCRIBING.md).

All of this reading was done by a coding agent and its sub-agents. No person
has checked it against the pages. Two things guard it a little. The
transcribers of the table could not run `rhind`, and read 2 ÷ 59 as 2̅2̅6̅
before anything flagged it. And `rhind verify` lists every place an edition
says the scribe erred and `rhind` finds the line right. Several of those were
transcription errors (a dot silently restored) and have been put back.

What does not fit the notation is kept as the scribe's words in quotes and
not computed: the hundred-hekat notation of Problems 68-86, ditto marks,
drawings, and some columns set side by side.

## The decisions

`rhind decisions` lists every choice this reconstruction makes that the
papyrus does not fix: how a value is written (only for printing; figures
are compared by value), how a column is told from the next, that a row of
fractions may be taken of the row above ("add its half"), that a column's
own rows outvote one slip in its first row, how the table's three-figure
rows are read, and that a restored figure is used but never counted for or
against the scribe.

## In the browser

`web/` is a playground that runs `rhind`, compiled unchanged for WASI
(`wasm32-wasip1`), in the page: every page of the papyrus, checked, with the
wrong figures marked and their slips; the transcription in an editor whose
colours, problems and hover come from `rhind lsp` running in the page too;
and a box that works any multiplication or division the scribe's way, a row
at a time.

```bash
node web/build.mjs                  # builds web/rhind.wasm and web/papyrus.json
node web/test.mjs                   # every page through the wasm build, compared with the native one
python3 -m http.server -d web 8000  # then open http://localhost:8000
```

## Tutorial

[`tutorial/`](tutorial/01-introduction.md) has nine chapters: numbers,
multiplying, dividing, the table, false position, the scribe's mistakes,
reading the papyrus, and a reference. Every page and command in it is run by
`cargo test` and compared with what the chapter says it prints.

## Tests

```bash
cargo test
```

Unit tests for the arithmetic, the scribe's operations, the reader, the
checker, the slips and the language server. The integration tests check
that every page of the papyrus is in the notation; that `verify`, `errata`
and `2/n` print what they printed when last accepted (`RHIND_BLESS=1 cargo
test` accepts a change, which then shows in the diff); every example; and
the tutorial.

## Earlier work

The papyrus has been edited and studied for 150 years: Eisenlohr (1877),
Peet (1923), Chace (1927-29), Gillings (*Mathematics in the Time of the
Pharaohs*, 1972), Robins and Shute (1987), and much since. Annette Imhausen's
*Ägyptische Algorithmen* (2003) reads the problems as algorithms, which is
the idea this project runs with, and is the scholarly work closest to it.
This project has not read it. The 2 ÷ n table's rules have a literature of
their own. Nothing here is claimed as new until it has been checked against
that literature and the photographs.

It follows the method of the other reconstructions in this collection
(JOSS, Turing's ACE, Böhm's compiler and others): rebuild from the documents
that survive, run what they printed, and treat each place the run and the
page disagree as evidence.

## Licence

Apache License 2.0 - see [LICENSE](LICENSE).
