# The routes: what the unmatched columns were

0.1.1 reported that 47 of the 65 columns `rhind routes` compares match
`rhind`'s own route, and listed 18 that do not. This note follows those 18:
what each one is, what was wrong in `rhind`, and a second test, `rhind routes
--derive`, that asks a different question of the scribe's working.

The short answer: the 18 hold no unknown operation. Six were caused by
`rhind`, three are ticking slips the editions note, and nine come from two
habits that the editions' own notes describe. Those habits are the main
finding. When the scribe's route differs from `rhind`'s, the scribe's route
is usually made row by row from rows already written, and `rhind`'s usually
is not.

## The 18, one by one

Each was read against its transcription and the Chace and Peet notes in the
`.rmp` file.

**Caused by `rhind` (6)**

| column | what it was |
|---|---|
| No. 22 l.7 | `5` written for 5̅ (Chace fn 2). `routes` compared the figure as written. Read as 5̅, the column is `rhind`'s route exactly. |
| No. 36 l.10 | `express` could not write 15/53. Its greedy form, 4̅ 3̅1̅ 1̅3̅1̅5̅ 2̅8̅8̅0̅7̅2̅7̅ …, ran past a part in a million million. It then stopped, **returning a value that was not 15/53**. A debug build panicked on `rhind routes`; release printed the wrong value. The same wrong value was in the published errata, against No. 36 l.18. |
| No. 76 l.10 | The tens convention was tried only on multiplications; `scribe::reckon` took no `Style`. |
| No. 44 l.19 | A `1̅0̅` row from another reckoning is set in the column. Peet: "strayed hither from No. 45". |
| No. 33 l.9 | Not a reckoning: the parts of 42 that make 1 ⅔ 2̅ 7̅, listed (the 42nds working). |
| No. 70 l.6 | The division stops at 99 2̅ 4̅ on purpose, and the remainder is worked apart (review finding 6). |

**Ticking slips the editions note (3)**: No. 35 l.15 (Peet: "a tick is
omitted before 3̅"), No. 39 l.5 (the row for 2 unticked, Peet *sic*), No. 53
l.7 (Chace and Peet: inaccurate ticking). `routes` takes the target from the
ticks, so the target is wrong in each.

**Another route (9)**, by two habits:

1. *A part reached through a row already written.* No. 31 writes 2̅ to
   halve it to 4̅. No. 43 writes 1̅0̅ for 2̅0̅. No. 21 writes 1̅0̅ for 5̅. No. 70
   l.28 halves its row for 10 to get 5. Nos. 32, 38 and 42 go ⅔, 3̅, 6̅ and on.
   `rhind` took any part of the first row directly.
2. *The part of the first row that makes a unit.* 1̅5̅ of 15 is 1 (No. 21).
   7̅ of 1 2̅ 4̅ is 4̅, then doubled (No. 34). 1̅0̅6̅ of 106 is 1, then doubled and
   halved (No. 36). 7̅ of 3 2̅ is 2̅ (No. 69). 6̅3̅ of 7 2̅ 4̅ 8̅ is 8̅, then
   "double the fraction" (No. 70). This is the working of the 2 ÷ n table, and
   `scribe::reckon` already used it as a last resort.

None of these is new. Both habits can be seen in the editions' own notes. This
project has not read the wider literature on them (Gillings, Imhausen and
others), so it claims nothing about who described them first.

## What was changed

- **`express` is exact.** When the greedy form is long, `rhind` now looks for
  one of at most four parts, none past 10,000. For 15/53 that is 4̅ 5̅3̅ 1̅0̅6̅
  2̅1̅2̅, the scribe's own form in No. 36. A value with no form within the
  bounds is never written inexactly:
  - `try_express` returns nothing, and `slips` then skips that candidate;
  - `Quantity` marks the form approximate, as before, and it is shown as a
    fraction.
- **Release builds check overflow** (`overflow-checks = true`), so a wrapped
  fraction fails rather than printing.
- **`routes` reads omitted part signs.** It uses the readings the check has
  already made (its `part-sign` hint), and prints each with its reason. The
  transcription keeps the figure as written.
- **A division can be set out with a row for 10.** `scribe::reckon_with`
  makes the whole of the quotient by 1, 10, 20 … and the remainder as before.
  It works from the stated target where there is one. It does not just
  multiply by the quotient the ticks give, since that would copy the scribe.
- **"Rows written besides" is tested as a subsequence**; it had been a prefix
  test that could not match.
- **`routes` and `routes --derive` are golden tests.** The tests run a debug
  build, so the overflow would now have failed them.

Two lines of `errata` changed:
- No. 36 l.18 now gives the right value (4̅ 5̅3̅ 1̅0̅6̅ 2̅1̅2̅, with `53` written
  for 5̅3̅).
- No. 15's expected total is now written in parts (2̅0̅ 1̅6̅0̅ 2̅8̅5̅ 3̅8̅4̅, which
  is 455/7296) rather than as a fraction.

Review finding 14 said no value in the papyrus was affected by inexact
writing. That was not so for No. 36.

## `routes`, before and after

| | 0.1.1 | now |
|---|---|---|
| the same rows and ticks, by doubling | 40 | 41 |
| the same, with the tens done by a row for 10 | 3 | 4 |
| the same, taking 3̅ straight from the first row | 2 | 2 |
| the same, read as a division | 2 | 2 |
| `rhind`'s rows, with others written besides | 1 | 5 |
| the scribe uses 10 another way | 5 | 4 |
| the scribe goes by ⅔ where `rhind` halves, or the reverse | 6 | 4 |
| another route | 6 | 3 |
| **matching** | **47** | **49** |

Two columns came to match: No. 22, by the reading of 5̅, and No. 76, by the
tens division. None ceased to match. Four moved from "another route" or "⅔"
to "rows written besides" (Nos. 31, 33, 35 l.15, 43 l.23), because that test
now works. The count went up because `rhind` was fixed, not because a rule
was fitted.

## `routes --derive`: is each row made from one before it?

`routes` asks whether the scribe's route is the one `rhind` would choose.
That depends on how `rhind` breaks ties, and its routes can take any part of
a number directly. `routes --derive` asks something else, with no target and
no tie-breaking. Is every row of the scribe's column made from one row
already written, by one of these operations? And is every row used, either
ticked or the source of a later row?

| operation | from |
|---|---|
| twice, half, ⅔ of a row | any row above |
| 3̅ of a row, its ⅔ not written | any row above (Nos. 35, 39) |
| 10 times, 1̅0̅ of | the first row |
| n̅ of, where that makes 1 or a single part | the first row |
| a part the page's statement names | the first row |

The list was fixed before the columns were run, and it was not changed after.

Not all of this comes from the papyrus. It shows each operation in use:
doubling, halving, ⅔, 3̅, a row for 10, 1̅0̅, and n̅ of a number. But two parts
of the test are this project's choices:
- that ×10 and 1̅0̅ are taken only of the first row;
- that a faithful column has every row ticked or used to make another.

The papyrus does not say the scribe worked under either rule.
Its exceptions are reported below, along with what loosening it would do.

| | faithful | no ticks | unused row | underived | all |
|---|---|---|---|---|---|
| the 65 columns `routes` compares | 55 | 0 | 5 | 5 | 65 |
|   where `rhind`'s route matches | 47 | 0 | 0 | 2 | 49 |
|   where it does not | 8 | 0 | 5 | 3 | 16 |
| every column read in full that begins with 1 | 55 | 43 | 5 | 15 | 118 |
| **`rhind`'s own routes**, for the 65 | 51 | 0 | 0 | 14 | 65 |
|   where they differ from the scribe's | 6 | 0 | 0 | 10 | 16 |

Each row of the table adds up: for the scribe's 65 columns, 55 + 0 + 5 + 5;
for `rhind`'s, 51 + 0 + 0 + 14.

This is the result. In the 16 columns where the scribe and `rhind` part
ways:
- the scribe's route is made row from row in 8, and `rhind`'s in 6;
- `rhind`'s takes a part no listed operation makes in 10, and the scribe's in 3.

The test therefore separates the two: it is not a filter that every route
passes. The 8 are Nos. 21, 31, 34, 36, 42, 43 l.23, 69 l.9 and 70 l.6, the
two habits above. The constraint matches the scribe's routes better than
`rhind`'s choice of route does.

Only two columns are a choice between two routes that both derive:
- No. 34: 7̅ of the first row, where `rhind` goes by ⅔.
- No. 69 l.9: tens, where `rhind` doubles.

The other four where both derive are the ticking slips, plus No. 44's stray row.

**The exceptions.** Five of the 65 have a row no listed operation makes:
- No. 33 l.9: 2̅8̅ of 42, in the 42nds list.
- No. 38 l.22: 1̅1̅, a part of a multiplier carried from earlier working.
- No. 38 l.31: 7̅ of the first row.
- No. 54 l.5: 5̅ of the first row.
- No. 70 l.28: 5̅ of 100.

Five rows are "unused": No. 32's 1̅1̅4̅ (its ticks are missing), Nos. 35 l.15,
39 l.5 and 53 l.7 (the ticking slips), and No. 44's stray row.

**Sensitivity.** Each change below was run once, and none is kept:

| change to the list | faithful of 65 | underived of 65 | underived of 118 |
|---|---|---|---|
| as above | 55 | 5 | 15 |
| ×10 of any row, not only the first | 55 | 5 | 13 |
| 5̅ of the first row, as 1̅0̅ is | 57 | 3 | 12 |
| any part of the first row (no constraint) | 59 | 0 | 2 |

Of the changes tried, taking 5̅ straight from the first row accounts for the
most exceptions. It occurs with round numbers (5̅ of 100 is 20), as 1̅0̅ does.

## What this does not show

- That the scribe was bound by these operations. A stepping-stone row shows
  what he wrote, not that a direct part was forbidden. The constrained model
  is one discipline that fits 55 of 65 columns, not a rule he followed.
- That the scribe's routes are the shortest such. `--derive` checks each row
  locally (made from one row above, and used). It does not search for a
  shorter route under the same operations.
- Anything about other papyri. Moscow, Lahun and the Leather Roll are not
  transcribed here. A test of the operation list on independent material
  would need them.
- The 65 are still a selected set (review finding 11). `--derive` also runs on
  the 118 columns that begin with 1, but 43 of those tick nothing (most are
  lists of parts), and for those, whether a row is used cannot be said.
