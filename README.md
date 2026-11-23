# rhind

The Rhind Mathematical Papyrus, run. The scribe Ahmose's working, about
1550 BCE, transcribed so that it can be computed, and every figure in it that
the notation can hold worked again and compared, exactly, with what he wrote.

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

The papyrus writes fractions only as parts of one, with two-thirds the one
exception, and it divides by multiplying: to divide, the scribe doubles the
divisor until the rows can be picked to make the dividend. To double an odd
part, he looks it up in the table of 2 divided by odd numbers at the head of
the papyrus.

`rhind` does two separate things. It **checks**: each figure the scribe wrote
is compared, as an exact fraction, with what the working above it gives. And
it **works**: where a page asks for a multiplication or a division, it writes
the working the scribe's way, by doubling, halving, two-thirds and the table
(`rhind routes` compares that with the scribe's columns). The check is exact
arithmetic under the interpretive choices listed by `rhind decisions`; it is
not a trace of the scribe's operations, and a disagreement can come from the
figure, the transcription or one of those choices. For each disagreement it
lists the explanations that fit: a sign left off, one sign written for
another, the wrong rows ticked, and others.

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
rhind errata         # every disagreement, and the explanations that fit it
```

For Visual Studio Code, download `rhind-language-0.1.2.vsix` from the
[latest release](https://github.com/chrishayuk/rhind-papyrus/releases/latest)
and run `code --install-extension rhind-language-0.1.2.vsix`: colours, the
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
rhind routes --derive      is each row of a column made from one before it, by operations the papyrus shows?
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

These are counts against this project's transcription, under the rules in
`rhind decisions`. The transcription was made by a coding agent and its
sub-agents and no person has checked it; an independent review by a second
model checked 72 places against the scans
([`docs/independent-review.md`](docs/independent-review.md)). Nothing here is
claimed as new.

`rhind verify` checks 956 lines of working across 91 problem sections and the
two tables: 845 agree with the working, 111 do not. Those 111 lines give 113
diagnostic records (one written figure can give two). They are of three kinds:

| | records | what they are |
|---|---|---|
| a part sign | 75 | the figure differs from the working only by the sign for "part": 30 where 3̅0̅ is meant, or the reverse |
| the ticks | 4 | the total written is right for other rows than those ticked |
| a figure | 34 | a figure that differs in more than its part sign |

The first kind is not a count of errors. Chace (vol. II, p. xi) says the part
sign is often left off where the context makes the fraction clear, and that
its omission need not be an error; Peet (p. 38) counts wrong dots among the
scribe's errors and corrects them silently. The editions disagree about what
these are, and `rhind` reports them apart.

Of the 113, the transcribers' notes quote an edition commenting at 97. That
is a keyword search of the notes, not a census of what Chace and Peet say:
the review found edition comments the search misses, so the 16 left over are
not "unnoted". It also found no misreading of a digit at any of them.

### What the 34 are

Most of the 34 are known: 2 ÷ 79 (2̅3̅3̅ and 4̅1̅6̅ for 2̅3̅7̅ and 3̅1̅6̅),
No. 27 (4 times 3 2̅ written 15), No. 31 (total 99 for 97), No. 43
(4̅5̅ for 1̅8̅0̅), No. 58 (1 4̅ for 1 2̅ 4̅), No. 63 (113 3̅ for 133 3̅ in a
list totalled 700), No. 79 (2301 for 2401), and the 9̅-for-1̅4̅ sequence of
Nos. 10-14, all pointed out by Chace or Peet.

For each, `rhind errata` lists the explanations that fit, not the cause.

| explanation | fits | the only fit |
|---|---|---|
| a part sign left off or added | 77 | 75 |
| one hieratic number sign for another (the figures differ in one decimal place) | 8 | 8 |
| the ticks | 5 | 4 |
| the multiplier one sign away from the one written | 4 | 3 |
| the row is twice or half another row | 6 | 0 |
| one figure of a list, by one of the above | 3 | 3 |
| a fraction left out or added | 4 | 1 |
| none of these | 13 | |

"One sign for another" is inferred from the decimal places of the numbers,
not from the shapes of the hieratic signs. Among the candidates it emits, the
pair that recurs is 60 for 80: 6̅6̅ for 8̅6̅ in 2 ÷ 43, 63 for 83 in 2 ÷ 83 and
1̅6̅6̅ for 1̅8̅6̅ in 2 ÷ 93, five places. Chace saw this: his note to 2 ÷ 43
names all five. No other pair is emitted more than twice.

### Was the error carried?

Where one wrong row stands behind a total or result, `rhind` can say whether
the total was made from the figure as written or from the right one. After
the review tightened this test, it applies at 12 places: 11 are a part sign,
and 1 is a figure. At all 12 the total is the right one. With 11 of them
notational, this says nothing general about how the scribe worked; an
earlier version of this README drew a conclusion from it that it does not
support (see the review, finding 4).

### 2 ÷ 59

The red answer at the head of the entry has 2̅2̅6̅; the working has 2̅3̅6̅, and
4 × 59 is 236. Chace prints 226 and 236 as described; Peet prints 236 in
both, without remark on this page. The transcriber read 226 on Chace's plate
before `rhind` could run, and the review read it again. This is a
discrepancy between the editions, and a question for the British Museum
photograph. It is not shown to be an error of the scribe's.

### The routes

`rhind routes` asks whether the scribe sets out a column the way `rhind`
would. The problems have 139 columns of two rows or more. 128 can be read in
full, and 65 of those begin with a row for 1, say what they multiply and tick
at least one row; only those 65 are compared, by their multipliers and ticks.
A multiplier written without its part sign, where the check finds that, is
read as the part, and the listing says so:

| | columns |
|---|---|
| the same multipliers and ticks, by doubling | 41 |
| the same, with the tens done by a row for 10 | 4 |
| the same, taking 3̅ straight from the first row | 2 |
| the same, read as a division | 2 |
| `rhind`'s rows, with others written besides | 5 |
| the scribe uses 10 another way | 4 |
| the scribe goes by ⅔ where `rhind` halves, or the reverse | 4 |
| another route | 3 |

So 49 of the 65 eligible columns match after trying these three
conventions. The targets are often taken from the scribe's own ticks, so this
describes the subset, and does not validate the procedure. `rhind routes
--all` lists the columns that differ.

`rhind routes --derive` asks a question with no target and no choice of
route. Is every row of the scribe's column made from one row already written,
by a fixed list of operations? The list is twice, half, ⅔ or 3̅ of a row; 10
times or 1̅0̅ of the first row; and n̅ of the first row where that makes 1 or a
single part. Is every row also ticked or used to make another? 55 of the 65
are. `rhind`'s own routes for the same columns pass the same test in 51.
Where the two differ (16 columns), `rhind`'s route takes a part no listed
operation makes in 10, and the scribe's in 3. The scribe reaches a part
through a row he has already written (2̅ then 4̅; ⅔, 3̅, 6̅), or takes the part
of the first row that makes a unit (1̅0̅6̅ of 106 is 1). `rhind` takes the part
directly. [docs/routes.md](docs/routes.md) goes through the 18 columns 0.1.1
left unmatched, the exceptions, and how the counts move if the list is
loosened.

### The table of 2 ÷ n

Why does the papyrus write 2/13 as 8̅ 5̅2̅ 1̅0̅4̅, when 7̅ 9̅1̅ has fewer
parts? `rhind 2/n` runs candidate rules for every odd n from 5 to 101 against
the 49 entries, with at most four parts and none past 1000:

| rule | entries |
|---|---|
| the largest part that fits, again and again | 4 of 49 |
| fewest parts, then the smallest last part | 25 |
| fewest parts, then fewest odd parts | 29 |
| 2/n = m̅ + (2m − n)/mn, 2m − n made of up to three divisors of m (n/2 < m ≤ n), fewest parts | 32 |
| the same divisor method, smallest last part first | 34 |

The scores depend on the bounds: with parts up to 10,000 the fewest-parts
divisor rule falls to 25 (the review reproduced this with an independent
program, and so did this project). The targets take 2 ÷ 43, 59 and 93 as the
working, not the answers, gives them. No rule here gives 2 ÷ 13 or 2 ÷ 95.
Agreement describes a rule; it does not show the scribe used it. Rules of
these kinds have been proposed for a long time, and none here is claimed to
be any one scholar's, or new.

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

Every figure was read from page images, not from OCR: the OCR destroys the
fraction bars, and the bars are the point. For the problems, both Chace's
literal translation and his plates were read; for most of the 2 ÷ n table
the transcriber read Chace's translation pages, which print the red ink in
red, and read the plates glyph by glyph only at a few entries (see
`sources/notes/table-2n-a.md`). Where the editions disagree, the
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
are compared by value), how a column is told from the next, what number a
column multiplies (the statement's, or its row for 1 with or without a part
sign put back, and otherwise the column is left unchecked), that a row of
fractions may be taken of the row above ("add its half") where no statement
says otherwise, how the table's three-figure rows are read, how a division
stopped short with a remainder is followed, which units can be compared, and
that a restored figure is used but never counted for or against the scribe.
Several of these were changed after the independent review.

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

## Independent review

A second model, OpenAI's Codex, reviewed 0.1.0 independently: it checked 72
places in the transcription against the scans, wrote probe pages to break the
checker, and read the code. It found no misread digit, confirmed 2 ÷ 59 and
the five 60-for-80 places, and found that the census and several of the
checker's rules were unsound. Every finding was checked here and acted on;
what was done about each is in
[`docs/independent-review.md`](docs/independent-review.md), and its report is
[`docs/independent-review-report.md`](docs/independent-review-report.md).
The numbers in this README are after those changes.

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
*Ägyptische Algorithmen* (2003) reads the problems as algorithms, the idea
this project runs with; this project has not read it, and so does not say how
close it is. Work on the problems continues: Luca Miatello's reconstructions
of the geometric problems, for one. An outside reader pointed to a 2025
paper of his on Problem 43, which this project has not been able to confirm
or read. The 2 ÷ n table's rules have a literature of
their own. Nothing here is claimed as new until it has been checked against
that literature and the photographs.

It follows the method of the other reconstructions in this collection
(JOSS, Turing's ACE, Böhm's compiler and others): rebuild from the documents
that survive, run what they printed, and treat each place the run and the
page disagree as evidence.

## Licence

Apache License 2.0 - see [LICENSE](LICENSE).
