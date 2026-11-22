# The notation

A `.rmp` file is a page of the scribe's working, written down so that it can
be run. Most of it is what the papyrus has on it, in the papyrus's own
layout: a statement of the problem, columns of doubling with the scribe's
tick marks, totals, results and proofs. `rhind` reads it, does every
computation the way the scribe does it, and says where the scribe's figures
and its own disagree.

A file that transcribes the papyrus says only what the papyrus says. A file
written by a person may also leave the working out, and `rhind` writes it in
(see "Leaving the working out", below).

## Numbers

The Egyptians wrote a quantity as a whole number followed by distinct unit
fractions, largest first, with one exception, two-thirds.

| written | ASCII | value |
|---|---|---|
| `5` | `5` | five |
| `2̅` | `'2` | a half (the "part" sign over 2) |
| `1̅5̅` | `'15` | one fifteenth |
| `⅔` | `2/3` or `''3` | two-thirds |
| `5 3̅ 1̅5̅` | `5 '3 '15` | 5 + 1/3 + 1/15 |
| `⅔ 1̅0̅ 3̅0̅` | `2/3 '10 '30` | 2/3 + 1/10 + 1/30 |

Terms of a number are separated by single spaces. `rhind` always writes the
overbarred form (each digit followed by U+0305, COMBINING OVERLINE) and `⅔`;
either form is read.

A number may carry a unit: `3 '4 hekat`, `5 ro`, `10 khar`, `100 cubit`,
`2 setat`, `deben`, `kite`, `loaf`, `pesu` (a word, not a number, so
`pesu 20`). `rhind` keeps units on numbers and only checks a sum where the
units agree. Hekat fractions (the "Horus-eye" fractions 2̅ 4̅ 8̅ 1̅6̅ 3̅2̅ 6̅4̅ of a
hekat) are written as ordinary unit fractions with the unit:
`'4 '16 '64 hekat 1 '3 ro`. A hekat is 320 ro.

A whole number may follow fractions: `7 2/3 39`. That is not good Egyptian,
and it is there to be written when the papyrus has it: a part whose sign the
scribe left off. It is read as written, and `rhind` finds the slip.

On land, a cubit-strip is a hundredth of a setat: `'2 '8 setat 7 '2 cubit`.

## Ink

The papyrus is written in black, with some numbers and headings in red.
Anything in red is written in braces: `{1 '2}`, `{Total 21}`. The colour is
kept; it does not change the value.

## What is not in the papyrus

| written | means |
|---|---|
| `{Total} 15.` | a word of a statement in red |
| `[5]` | restored: the papyrus is damaged here, and the edition fills it in. Write what the edition restores, inside the brackets |
| `[...]` | lost, and not restored |
| `5?` | read, but uncertain |
| `% …` | a note, to the end of the line. Not in the papyrus |

A transcription never corrects the scribe. If the papyrus has 15 where 14 is
right, write 15. `rhind` finds it. If an edition prints *(sic)*, record that
in a `%` note on the same line: `\ 4  15     % Peet: (sic)`.

## The words

A line in double quotes is the papyrus's words, in translation: `"A quantity
whose fourth part is added to it becomes 15."` Numbers inside quotes are not
computed; they are part of the text. Use the literal translation (Chace's
second volume), modernised only as far as needed to read.

## The working

### Headings

```
No. 26                 % a problem; also No. 7B, No. 59B, No. 61B, No. 82B
2 ÷ 7                  % an entry of the table of two divided by an odd number (ASCII: 2 : 7)
```

Everything up to the next heading belongs to it.

### Statements

A statement is a sentence of the scribe's that `rhind` acts on. Each ends
with a full stop.

| statement | what it does |
|---|---|
| `Multiply 3 '2 by 5.` | the next column multiplies 3 2̅ by 5 |
| `Reckon with 5 to find 15.` | the next column divides 15 by 5: find what 5 must be multiplied by to make 15 |
| `Total 15.` | the sum of what precedes is 15 (see below) |
| `Result 12.` | the last multiplication or division gave 12 |
| `Remainder 9.` | the last subtraction left 9 |
| `Add 12 '4 to 3.` | an addition written as a sentence |
| `Take 9 from 10.` | a subtraction written as a sentence |
| `Proof.` | what follows proves the answer |

Most of these are optional: the papyrus often shows a column with no sentence
before it, and then the column says what it does. A statement is written in
a transcription only where the papyrus writes it (in words or as a heading),
in its own words as far as they fit one of these forms; otherwise the words
go in quotes and the column stands alone.

### Columns

A column is the scribe's table of multiplication: rows of two numbers, the
multiplier on the left and the product on the right, the first row usually
`1` and the number being multiplied. The two are separated by two or more
spaces (or a tab). A row the scribe ticks, to be added in, starts with `\`.

```
Reckon with 5 to find 15.
\ 1   5
\ 2   10
Result 3.
```

```
  1   3 '2
  2   7
\ 4   15
```

A column ends at a blank line, at a line that is not a row, and at a row
whose multiplier is 1 if the column already has rows: the papyrus often sets
small columns side by side, and each begins with its row for 1.

A row with more than two numbers is kept and printed, and its first two are
checked. In the 2 ÷ n table, `\ 4  28  '4` is the scribe's double statement,
4 times 7 is 28 and 2̅8̅ of 7 is 4̅, and all three are checked.

`Total X.` after a column says the ticked products add to X; if no row is
ticked, all the products. `Total X.` after a list of numbers, one to a line,
says they add to X.

### Leaving the working out

A statement with no column after it is done by `rhind`, and the column it
writes is the one a scribe would write: doubling, then halves, two-thirds,
tenths or a unit fraction of the number for what remains, with the 2 ÷ n
table for twice a unit fraction. Which of these it chooses, and why, is in
`rhind decisions`.

```
Multiply 9 by 9.
```

prints

```
Multiply 9 by 9.
\ 1   9
  2   18
  4   36
\ 8   72
Total 81.
```

## A whole file

This is Problem 26 set out in full. The papyrus itself writes the first step
in words only and gives the proof no heading; `papyrus/no-26.rmp` has it as
the papyrus does.

```
% After Problem 26 (Chace vol. II pl. 49, photograph XII; Peet p. 62).
No. 26
"A quantity whose fourth part is added to it becomes 15."
"Reckon with 4; you are to make their quarter, namely 1. Total 5."
  1   4
 '4   1
Total 5.
Reckon with 5 to find 15.
\ 1   5
\ 2   10
Result 3.
Multiply 3 by 4.
  1   3
  2   6
\ 4   12
Result 12.
Proof.
  1   12
 '4   3
Total 15.
"The quantity is 12, its quarter is 3, total 15."
```
