# Independent review

## Who reviewed, and what was checked

The repository at 1ea07b6 (release 0.1.0) was reviewed by OpenAI Codex CLI
(codex-cli 0.166.0) on 22-23 November 2026. It is an independent second
model, not a person. It changed no file in the repository. Its report, as
written, is [`independent-review-report.md`](independent-review-report.md).

It built the program and ran all 26 tests, which passed, and reran `verify`,
`errata` and `routes`, which matched. It recomputed the 2 ÷ n rule scores with
an independent program. It wrote eleven probe pages to make the checker fail,
and it read the code. And it checked **72 places** in the transcription
against the scans of both editions at 300 dpi, chosen to test the claims:
every one of the 45 places behind the "46 figures not pointed out by either
edition", 2 ÷ 59, the five 60-for-80 places, the README's examples, and four
ordinary rows drawn at random.

Each finding below was checked again here against the code, by running it,
and against the scans where it concerns them, before anything was changed.

## What the scans showed

No misread digit was found at any of the 72 places. The readings the README
rests on survive: Chace prints 2̅2̅6̅ in the answer of 2 ÷ 59 and 2̅3̅6̅ in its
working, and Peet 236 in both; the five 60-for-80 places are as transcribed.

But many of the 46 "not pointed out by either edition" are pointed out: Chace
comments on them, sometimes in the very note the transcriber copied into the
file, in words the search for "sic", "error", "mistake" and the like did not
catch ("carelessly added", "used here twice"), or in a general note (the
fractional dot "omitted in these places"). Others are checker mistakes, below.
One transcription was wrong in kind: No. 66 has a red **Total**, transcribed
as `Result`.

## The findings, and what was done

### 1. The part sign is not "the commonest slip": accepted, reworded

Confirmed. Chace (vol. II, printed p. xi) says the part sign is often
omitted where the context makes the fraction clear, and that its omission
need not be an error; Peet (p. 38) counts wrong dots as errors and corrects
them silently. The README called 71 of these "the commonest slip … about
half of everything that goes wrong".

`rhind verify` now sorts its records into three kinds: those that differ
only in a part sign (notation, which the editions treat differently), those
where the ticks do not make the total, and figures that differ in more than
their part sign. The README reports them apart and no longer calls the first
kind errors, nor uses it as a rate.

### 2. The "87 pointed out / 46 not" split is a keyword search: accepted, withdrawn

Confirmed: `flagged()` searched the transcriber's note on the line for a few
words. It missed comments and caught some about the editions themselves.

The number is withdrawn from the README. `rhind verify` now says what it is:
"a keyword search of the notes, not a census of what the editions say". A
structured record of each edition comment, which the review recommends, has
not been made.

### 3. The base vote put errors on correct figures: accepted, fixed

Confirmed by running it. Any row could propose the number a column
multiplies, and the majority won. Where the scribe's own error is halved down
a column, as in the 9̅-for-1̅4̅ sequence of Nos. 10-14, the wrong rows outvote
the right first row: No. 11's 7̅ was called wrong. No. 18's correct 1̅8̅ and
total were called wrong, and No. 33's correct 2̅1̅ got an absurd expected value.

Now a column multiplies the number the page states, or else its row for 1 as
written or with a part sign put back on one of its numbers, whichever makes
more of the other rows right; no other row can propose a number, and the row
for 1 does not vote for itself. A column with no row for 1 is read the same
way from its first row (which settles No. 33: `97  42` for 9̅7̅  4̅2̅). If there
is a choice and no candidate makes any other row right, the column is
reported as ambiguous and not checked (No. 12); with no choice, the row for 1
is what the page says, and No. 11 now shows what both editions say: 9̅
written for 1̅4̅, and 1̅8̅ for 2̅8̅ halved down from it. The `decisions` entry `outvote` is replaced by `base`, and its
claim about No. 12 is removed. The ancient corrections in Nos. 11-12 (a 1̅4̅
written over a 9̅ in lighter ink) are still flattened into a third figure on
the row; encoding both states, as the review recommends, is not done.

### 4. The carried test was wrong, and 35 of 38 did not show pen slips: accepted, fixed, withdrawn

Confirmed, and found here independently before the review arrived. The three
"carried" cases were all a part sign left off a multiplier (5 for 5̅ in
No. 22, 16 and 32 for 1̅6̅ and 3̅2̅ in No. 37): the product beside it was right
for the multiplier meant, but the checker added the literal multiplier into
the total it expected. Now the product as written is the true one when the
slip is in the multiplier. The review also found that a row was called "not
carried" whenever a later total matched, even if it was not ticked, or other
wrong rows stood behind the same total. Now a verdict is given only where one
wrong row, counted in the total, stands behind it.

The test now applies at 12 places, 11 of them a part sign. The README's
conclusion that the scribe "worked from the right figure and wrote another"
is withdrawn.

### 5. Restored totals were counted against the scribe: accepted, fixed

Confirmed: No. 2's `Total [2].`, restored by both editions, was one of the
"unnoted errors". A restored total or result is now never counted, a list with
a restored member is not summed, and a restoration that disagrees with the
working is reported as a remark on the edition. The review's probes are kept
in `tests/review/` and their output is a test.

### 6. Partial divisions, remainders and unticked columns: accepted, fixed

Confirmed. No. 70 stops its division at 99 2̅ 4̅, writes the remainder 4̅, and
finishes in further steps; the checker called all three wrong. A division
short of its target is no longer reported; `Remainder` is checked against what
is left of the target; a result reached in steps after a remainder is reported
as not checked. A column with no ticks gave a quotient of 0 (Nos. 55 and 66):
it now gives no result. No. 66's red Total, which adds the multipliers and not
the products, is kept as the scribe's words, with a note.

### 7. Units: accepted, fixed

Confirmed by its probes: hekat against cubit passed, and a list of 1 hekat and
320 ro was totalled as 321 hekat. Figures are now compared and listed only in
units that agree, with hekat and ro converted; any other pair is not checked.

### 8. "Relative" rows could certify a wrong explicit multiplication: accepted, fixed

Confirmed: `Multiply 10 by 2̅` with the row `2̅  10` was marked right, as half of
the row above. A row taken of the row above is now allowed only where the
page states no multiplication, and it is taken of that row as written.

### 9. The method claim went beyond the checker: accepted, reworded

Confirmed. The check compares exact fractions; it is not a trace of doubling,
halving and the table. The README said `rhind` "has no other way to do it" and
that a disagreement "is in the figure, not in the method". It now describes
the two things separately: an exact check under the stated choices, and a
generator that works the scribe's way. `rhind double` of an odd part past the
table (it stops at 101) now says it was written by value. The table `rhind`
uses takes 2 ÷ 43, 59 and 93 from the working, not from the answers, and the
README says so.

### 10. One wrong figure could be counted twice: accepted, fixed in part

Confirmed: in the table's three-figure rows (`2  6̅6̅  2̅`), a wrong middle
figure also failed the check of the third. The third is now checked against
the multiplier, so one wrong figure is one record. The counts are now called
"diagnostic records", not wrong figures; stable identities for figures, which
the review recommends, are not made.

### 11. 47 of 65 was a fit on a selected subset: accepted, reworded

Confirmed. `rhind routes` now prints its inclusion rule and the counts (139
columns, 128 readable, 65 eligible), and the README says the 65 are compared by
multipliers and ticks only, with targets often taken from the scribe's own
ticks, so that this describes the subset and does not validate the procedure.

### 12. The 2 ÷ n scores depend on bounds: accepted, disclosed

Confirmed, and reproduced here with a separate program: with parts up to
10,000 the fewest-parts divisor rule falls from 32 to 25 while the other stays
at 34. `rhind 2/n` now prints its bounds, the sensitivity, the normalized
targets, and "no candidate within the bounds" where it printed `rule 0`.

### 13. Not every explanation was reported: accepted, fixed

Confirmed: `explain` stopped at the first part sign or checker hint. It now
collects every model that fits, and `errata` counts both the figures each fits
and those it is the only fit for. The list explanation still keeps only the
candidates the page itself attests where there are any (No. 63), and the
README says so. "One sign for another" is inferred from decimal places, not
from glyph shapes, and the README says that too.

### 14. A value too fine to write came out as 0: accepted, fixed

Confirmed: `rhind double '1000000000001` printed 0. The commands now refuse
a value they cannot write exactly, and a message shows such a value as a
fraction. No value in the papyrus was affected.

### 15. The coverage claims were too broad: accepted, reworded and fixed

Confirmed. The README began "Every figure"; it now says the figures the
notation can hold. The table of tenths had no checked line; each entry is now
checked against a ÷ 10 (8 of 9 agree; 9 ÷ 10 has 30 for 3̅0̅, as Chace notes).
The README said every page of both editions was read; it now says how the
table was read. A stale note on cubit-strips is corrected. Imhausen is no
longer called "closest", since this project has not read her.

## What the review did not cover

It did not run the VS Code extension or the playground, decipher the
papyrus's photographs themselves (Chace's plates are his drawing of them), or
search the literature. It is a model's review, like the transcription; no
person or palaeographer has checked either.
