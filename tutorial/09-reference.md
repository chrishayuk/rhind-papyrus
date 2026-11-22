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
    A value is written as its whole part and then its fraction: the fewest distinct unit fractions that make it, with ⅔ allowed first, and among those the one whose smallest part is largest, so that 3/4 is 2̅ 4̅ (not ⅔ 1̅2̅) and 7/8 is 2̅ 4̅ 8̅ (not 2̅ 3̅ 2̅4̅). Twice an odd part uses the 2 ÷ n table. rhind compares values, not forms, so this rule only decides how rhind writes a figure, never whether the scribe's is right. The rule is ours.

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
    A row of fractions only (2̅, 1̅0̅, ⅔) is right if it is that part of the first row or that part of the row just above it: the papyrus writes `10  640` then `2̅  320` for "add its half" (No. 41), and `1̅0̅  150` under `20  1500` (No. 44). rhind says which it took. Allowing it can hide an error that happens to equal a part of the row above; ours.

outvote
    The number a column multiplies is chosen among the statement's, the row for 1 as written, that row with a part sign put back on one of its numbers, and the number each other row implies: the one that makes most rows right, the earlier on a tie. So one slip in the row for 1 (9 for 9̅, No. 12) shows as one wrong figure, not a column of them. Ours.

table-rows
    In the 2 ÷ n table a row `a  k  c` is the scribe's double statement: a times n is k, and k̅ of n is c. The part sign on k is optional, as the papyrus writes it both ways; a row `a  k̅` says the same without c. A `Remainder` in an entry is what is left of 2 once the parts at the head of the entry, one or more, are taken. Read from Chace's notes on 2 ÷ 7 and 2 ÷ 17.

bare
    A whole number standing among fractions (`7 ⅔ 39`) is read as written, a whole number, and its value counts as such. The papyrus means a part whose sign was left off; rhind finds that by checking, and names it, rather than by correcting the figure.

restored
    A figure an edition restores ([…]) is used to work out what follows, but a row containing one is never counted right or wrong: it tests the edition, not the scribe. Where a restoration disagrees with the working, rhind says so.
```
