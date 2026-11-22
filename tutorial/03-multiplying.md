# 3. Multiplying

The scribe multiplies by doubling. To multiply 9 by 9 he writes 1 against 9,
and doubles both sides until the left would pass 9; then he ticks the rows
whose left sides make 9, and adds their right sides.

```sh
$ rhind multiply 9 by 9
Multiply 9 by 9.
\ 1   9
  2   18
  4   36
\ 8   72
Total 81.
```

`\` is the scribe's tick. A page can say `Multiply 9 by 9.` and leave the
working out; rhind writes it in, marked `· rhind`, and checks what comes
after it:

```rmp
Multiply 3 '2 by 5.
Result 17 '2.
```

```
Multiply 3 2̅ by 5.
\ 1   3 2̅                               · rhind
  2   7                                 · rhind
\ 4   14                                · rhind
Total 17 2̅.                             · rhind
Result 17 2̅.                            ✓

1 figures checked, 1 right, 0 wrong.
```

Fractions of the multiplier get rows of their own: a half of the first row,
two-thirds of it, a tenth.

```sh
$ rhind multiply 12 by 2/3 '4
Multiply 12 by ⅔ 4̅.
   1   12
\  ⅔   8
\ 4̅   3
Total 11.
```
