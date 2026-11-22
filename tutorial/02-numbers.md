# 2. Numbers

An Egyptian number is a whole number and unit fractions: one part in 2, one
part in 15. The sign for "part", written over the number, is shown here as a
bar: 1̅5̅ is a fifteenth. Each fraction is used once, the largest first.
Two-thirds has a sign of its own, ⅔, and is the one fraction that is not
"one part".

You type a part as `'15`, and two-thirds as `2/3`. rhind writes the bars.

```sh
$ rhind double '7
4̅ 2̅8̅
$ rhind halve 7
3 2̅
$ rhind thirds '5
1̅0̅ 3̅0̅
```

Twice 7̅ is not 2/7: the Egyptians had no way to write that. It is 4̅ 2̅8̅,
and the scribe knew it from the table at the head of the papyrus. rhind
doubles an odd part only through that table. Halving is easy: an odd whole
number leaves a half, and half of n̅ is 2n̅. Two-thirds of an odd part is 2n̅
6n̅: the papyrus states the rule itself, in Problem 61B.

A number can be the sum of several, written in one figure:

```sh
$ rhind double 5 '3 '15
10 ⅔ 1̅0̅ 3̅0̅
```
