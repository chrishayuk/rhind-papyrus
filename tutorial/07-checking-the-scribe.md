# 7. When the scribe is wrong

The papyrus has mistakes in it. rhind finds them by doing the working again,
and then asks what slip would turn the right figure into the one written.
This is the last column of Problem 27 as the papyrus has it:

```rmp
No. 27
\ 1   3 '2
  2   7
\ 4   15
Total 17 '2.
```

```
No. 27
\ 1   3 2̅                               ✓
  2   7                                 ✓
\ 4   15                                ✗ 14
Total 17 2̅.                             ✓

4 figures checked, 3 right, 1 wrong.
line 4: 4 of 3 2̅ is 14; the papyrus has 15
    what follows is right: the error is not carried
    could be one-sign: 5 written for 4
```

Four times 3 2̅ is 14, not 15. The total after it is right, so the scribe
worked from the right figure and wrote a wrong one: the error is "not
carried". And 15 is 14 with one sign changed: in hieratic, 4 and 5 are
different signs, and one was written for the other.

The explanations rhind tries. Each is a way a figure can come out as it
did; a fit is not a cause, and several can fit one figure:

| model | the slip |
|---|---|
| `one-sign` | one hieratic sign written for another: the numbers differ in one place |
| `part-sign` | the sign for "part" left off, so 3̅0̅ is written 30 |
| `one-part` | a fraction left out, added, or a different one |
| `neighbour` | the figure from the row above or below |
| `wrong-row` | the wrong row doubled or halved |
| `whole-only` | the whole number or the fractions of a figure, without the rest |
| `in-the-list` | a list's total is right and one figure in it is wrong |
| `multiplier` | the row is right for a multiplier one slip away from the one written |
| `ticks` | the total is right for other rows than those ticked |
| `carried` | a total computed from a wrong figure |

The difference rhind meets most often is a part sign left off:

```rmp
\ 1    '15
\ '2   30
Total '10.
```

```
\ 1    1̅5̅                               ✓
\ 2̅   30                                ✗ 3̅0̅
Total 1̅0̅.                               ✓

3 figures checked, 2 right, 1 wrong.
line 2: 2̅ of 1̅5̅ is 3̅0̅; the papyrus has 30
    what follows is right: the error is not carried
    could be part-sign: 30 written for 3̅0̅: the part sign left off
```

Whether this is a mistake at all is a question of reading. Chace says the
sign is often omitted where the context makes the fraction clear, and that
its omission need not be an error; Peet counts it among the scribe's
errors. rhind reports these as notation, apart from figures that differ in
more than their part sign.

`rhind errata` runs the whole papyrus and lists every disagreement, each with
the explanations that fit it, and whether the transcriber's note quotes an
edition on it.
