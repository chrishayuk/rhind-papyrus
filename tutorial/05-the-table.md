# 5. The table of two

The papyrus begins with 2 divided by every odd number from 3 to 101. Each
entry gives the parts and the working that proves them. Here is the entry for
13 as the scribe wrote it. The red numbers at the head are the parts of 13;
the rows under them are his working. `\ 4  5̅2̅  4̅` is two statements in one
row: 4 times 13 is 52, and 5̅2̅ of 13 is 4̅.

```rmp
2 ÷ 13
{'8}  1 '2 '8
{'52}  '4
{'104}  '8
  1    13
  '2   6 '2
  '4   3 '4
\ '8   1 '2 '8
\ 4    '52   '4
\ 8    '104  '8
```

```
2 ÷ 13
{8̅}  1 2̅ 8̅                              ✓
{5̅2̅}  4̅                                 ✓
{1̅0̅4̅}  8̅                                ✓
  1    13                               ✓
  2̅   6 2̅                               ✓
  4̅   3 4̅                               ✓
\ 8̅   1 2̅ 8̅                             ✓
\ 4    5̅2̅   4̅                           ✓
\ 8    1̅0̅4̅  8̅                           ✓

9 figures checked, 9 right, 0 wrong.
```

Why 8̅ 5̅2̅ 1̅0̅4̅, when 7̅ 9̅1̅ has fewer parts? People have asked since the
1870s. `rhind 2/n` tries rules for choosing an entry against all 49 of them:

```sh
$ rhind 2/n
How were the entries of the 2 ÷ n table chosen? Each rule, run for every odd n from 5 to 101:

greedy           4 of 49   the largest part that fits, again and again
shortest        25 of 49   fewest parts (at most 4, none past 1000), then the smallest last part
shortest-even   29 of 49   fewest parts, then fewest odd parts, then the smallest last part
smallest-last   19 of 49   the smallest last part, then fewest parts
divisors        32 of 49   2/n = m̅ + (2m-n)/mn, 2m-n made of divisors of m; fewest parts, then the smallest last part
divisors-last   34 of 49   the divisor method, the smallest last part first
divisors-first  16 of 49   the divisor method, fewest parts, then the smallest first part

Where `divisors-last` and the papyrus differ:
  2 ÷ 7   papyrus 4̅ 2̅8̅                      rule 6̅ 1̅4̅ 2̅1̅
  2 ÷ 13  papyrus 8̅ 5̅2̅ 1̅0̅4̅               rule 1̅0̅ 2̅6̅ 6̅5̅
  2 ÷ 23  papyrus 1̅2̅ 2̅7̅6̅                  rule 2̅0̅ 4̅6̅ 9̅2̅ 2̅3̅0̅
  2 ÷ 35  papyrus 3̅0̅ 4̅2̅                    rule 2̅1̅ 1̅0̅5̅
  2 ÷ 47  papyrus 3̅0̅ 1̅4̅1̅ 4̅7̅0̅           rule 4̅0̅ 9̅4̅ 2̅3̅5̅ 3̅7̅6̅
  2 ÷ 49  papyrus 2̅8̅ 1̅9̅6̅                  rule 4̅2̅ 9̅8̅ 1̅4̅7̅
  2 ÷ 53  papyrus 3̅0̅ 3̅1̅8̅ 7̅9̅5̅           rule 3̅6̅ 2̅1̅2̅ 3̅1̅8̅ 4̅7̅7̅
  2 ÷ 55  papyrus 3̅0̅ 3̅3̅0̅                  rule 3̅3̅ 1̅6̅5̅
  2 ÷ 61  papyrus 4̅0̅ 2̅4̅4̅ 4̅8̅8̅ 6̅1̅0̅    rule 4̅5̅ 1̅8̅3̅ 3̅0̅5̅ 5̅4̅9̅
  2 ÷ 71  papyrus 4̅0̅ 5̅6̅8̅ 7̅1̅0̅           rule 4̅2̅ 4̅2̅6̅ 4̅9̅7̅
  2 ÷ 77  papyrus 4̅4̅ 3̅0̅8̅                  rule 6̅6̅ 1̅5̅4̅ 2̅3̅1̅
  2 ÷ 89  papyrus 6̅0̅ 3̅5̅6̅ 5̅3̅4̅ 8̅9̅0̅    rule 6̅3̅ 2̅6̅7̅ 6̅2̅3̅ 8̅0̅1̅
  2 ÷ 91  papyrus 7̅0̅ 1̅3̅0̅                  rule 7̅8̅ 1̅8̅2̅ 2̅7̅3̅
  2 ÷ 95  papyrus 6̅0̅ 3̅8̅0̅ 5̅7̅0̅           rule 5̅7̅ 2̅8̅5̅
  2 ÷ 101 papyrus 1̅0̅1̅ 2̅0̅2̅ 3̅0̅3̅ 6̅0̅6̅  rule 0

No rule here gives: 2 ÷ 13, 2 ÷ 95
```

The rule that does best writes 2/n as m̅ and then makes up the difference
with divisors of m, which is what the scribe's red auxiliary numbers look
like. It reproduces most entries, not all. Which rule, if any, the scribe's
source used is not settled here.
