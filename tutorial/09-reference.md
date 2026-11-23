# 9. Reference

## Commands

```sh
$ rhind --help
rhind: the Rhind Mathematical Papyrus, run

  rhind FILE                 the page, with every figure the scribe wrote checked
  rhind no 27                a problem of the papyrus, run (also: rhind no 7B, rhind table)
  rhind --plain FILE         the page in the scribe's figures, without marks
  rhind --check FILE         what is wrong with it, a line each
  rhind verify               run the whole papyrus and compare with the editions
  rhind errata               every wrong figure in the papyrus, and the slips that would make it
  rhind multiply 3 '2 by 5   the working, as the scribe would write it
  rhind reckon with 6 to find 21
  rhind double '7            (also halve, thirds: two-thirds of)
  rhind routes               does the scribe work each column the way rhind does?
  rhind routes --derive      is each row of a column made from one before it, by operations the papyrus shows?
  rhind 2/n                  how were the entries of the 2 ÷ n table chosen? rules tested against the papyrus
  rhind decisions            the choices this reconstruction makes that the papyrus does not
  rhind lsp                  a language server on standard input and output
  rhind json FILE            the run, as JSON, for the playground

  FILE may be - for standard input. In a figure, 'n is one part in n and 2/3 is two-thirds.
```

## Statements

| statement | what it does |
|---|---|
| `Multiply A by B.` | the next column multiplies A by B; with no column, rhind writes it |
| `Reckon with A to find B.` | the next column divides B by A; with no column, rhind writes it |
| `Total X.` | the ticked products of the column above, or all of them if none is ticked, or the figures listed above, add to X |
| `Result X.` | the last multiplication or division gave X |
| `Remainder X.` | what the last subtraction left; in the table, what is left of 2 |
| `Add A to B.` `Take A from B.` | an addition or a subtraction |
| `Proof.` | what follows proves the answer |

## Figures

`5`, `'15` (1̅5̅), `2/3` (⅔), `5 '3 '15`, `'4 '64 hekat 1 '3 ro`. Units:
hekat and ro (320 ro to the hekat), khar, cubit, setat (100 cubit-strips to
the setat), deben, kite, loaf, pesu, hinu, day, khet, palm, finger.

## Layout

A row is two or more figures separated by two or more spaces, `\` in front
if ticked. A column ends at a blank line, at a line that is not a row, or at
a row for 1 after other rows. `No. 26` and `2 ÷ 7` begin a section.

## The decisions

```sh
$ rhind decisions
express
    A value is written as its whole part and then its fraction: the fewest distinct unit fractions that make it, with ⅔ allowed first, and among those the one whose smallest part is largest, so that 3/4 is 2̅ 4̅ (not ⅔ 1̅2̅) and 7/8 is 2̅ 4̅ 8̅ (not 2̅ 3̅ 2̅4̅). Twice an odd part uses the 2 ÷ n table. Where the largest-part-first form runs past four parts or a part past 10,000, the shortest form of at most four parts, none past 10,000, is used instead (15/53 is 4̅ 5̅3̅ 1̅0̅6̅ 2̅1̅2̅, as No. 36 writes it); a value with no exact form within the bounds is never written approximately. rhind compares values, not forms, so this rule only decides how rhind writes a figure, never whether the scribe's is right. The rule is ours.

double
    Twice a number is made term by term: n̅ for even n becomes (n/2)̅; for odd n the 2 ÷ n table is used, as the scribe would have; ⅔ becomes 1 3̅; 2̅ becomes 1. Repeated terms are then rewritten by value. The term-by-term part is the papyrus's (the table exists for it); the rewriting is ours.

two-thirds
    ⅔ of an odd part n̅ is 2n̅ 6n̅, the rule stated in Problem 61B; of ⅔ it is 3̅ 9̅, as in Problem 61. ⅔ of a whole number not divisible by 3 is written by value. 3̅ of a number is half of its ⅔, as the papyrus finds it everywhere it shows the step.

multiply
    A multiplication doubles the first row until the next doubling would pass the whole multiplier, ticks the doublings that make it, then adds one row for each fraction of the multiplier, ⅔ before 3̅. The papyrus also multiplies by 10 directly and by other routes; rhind checks any route the scribe wrote, but writes only this one.

reckon
    A division doubles the divisor while the double does not pass the dividend, ticks the largest rows that fit, then makes the remainder with the halving chain (2̅ 4̅ 8̅ …), the ⅔ chain (⅔ 3̅ 6̅ …) or tenths (1̅0̅ 5̅), whichever finishes in fewest rows; failing those, with parts of the divisor itself, n̅ of the divisor being 1. The order of preference is ours.

columns
    A column ends at a blank line, at a line that is not a row, or at a row whose multiplier is 1 when the column already has rows: the papyrus sets small columns side by side, and each starts with its row for 1. In an entry of the 2 ÷ n table every column multiplies the divisor. This is a reading of the layout, ours.

relative
    In a column the page gives no statement for, a row of fractions only (2̅, 1̅0̅, ⅔) is right if it is that part of the first row, or that part of the row just above it as written: the papyrus writes `10  640` then `2̅  320` for "add its half" (No. 41). rhind says which it took. Where the page states the multiplication, this is not allowed: the row must be that part of the number stated (independent review, finding 8). Ours.

base
    A column multiplies the number the page states, if it states one. Otherwise the row for 1 as written, or that row with a part sign put back on one of its numbers (9 for 9̅): whichever makes more of the other rows right, the written one on a tie; the row for 1 does not vote for itself, and no other row can propose a number. A column with no row for 1 is read the same way from its first row. If there is a choice and no candidate makes any other row right, the column is ambiguous and is not checked; with no choice, the row for 1 is what the page says the column multiplies. This replaces a vote in which any row could propose the number, which put the error on correct figures where the scribe's own error had been carried down a column (Nos. 11, 12, 18; independent review, finding 3). Ours.

table-rows
    In the 2 ÷ n table a row `a  k  c` is the scribe's double statement: a times n is k, and k̅ of n is c. The part sign on k is optional, as the papyrus writes it both ways; a row `a  k̅` says the same without c. c is checked against 1/a, so one wrong k is one finding, not two (independent review, finding 10). A `Remainder` in an entry is what is left of 2 once the parts at the head of the entry, one or more, are taken. Read from Chace's notes on 2 ÷ 7 and 2 ÷ 17.

partial
    A division may stop short of the number to find: the ticked rows make less, the page writes the remainder, and the division is finished in further steps (No. 70). Falling short is not reported; a `Remainder` is checked against what is left; the result the page then gives is checked only if it is the result of a column rhind has, and otherwise reported as not checked. A multiplication or division with nothing ticked does not say which rows make the answer, so its result is not checked.

units
    Figures are compared, and added in lists, only in units that agree: a bare number goes with anything, hekat and ro convert (320 ro to the hekat), and any other pair is not checked. Inside one figure, setat and cubit-strips convert (100 to the setat); a cubit on its own is a length and is not converted.

bare
    A whole number standing among fractions (`7 ⅔ 39`) is read as written, a whole number, and its value counts as such. The papyrus usually means a part whose sign was left off; rhind finds that by checking, and names it, rather than by correcting the figure. Chace (vol. II, p. xi) says such an omission, where the context makes it clear, need not be an error; rhind reports these as notation, apart from other differences.

restored
    A figure an edition restores ([…]) is used to work out what follows, but a row, total or result containing one is never counted right or wrong, and a list with a restored member is not summed: it tests the edition, not the scribe. Where a restoration disagrees with the working, rhind says so.
```
