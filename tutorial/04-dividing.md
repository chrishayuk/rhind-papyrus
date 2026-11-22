# 4. Dividing

There is no division in the papyrus. The scribe asks: what must 5 be
multiplied by to make 15? "Reckon with 5 to find 15." He doubles 5 until the
rows can make 15, ticks them, and the answer is the sum of the ticked left
sides.

```sh
$ rhind reckon with 5 to find 15
Reckon with 5 to find 15.
\ 1   5
\ 2   10
Result 3.
```

When the rows cannot make the number exactly, he goes on with halves, or
with two-thirds, or with tenths:

```sh
$ rhind reckon with 6 to find 21
Reckon with 6 to find 21.
\  1   6
\  2   12
\ 2̅   3
Result 3 2̅.
```

And when nothing else will do, with parts of the divisor itself. 7̅ of 7 is
1, so what is left over can be made of parts of 7:

```sh
$ rhind reckon with 7 to find 2
Reckon with 7 to find 2.
     1   7
\   4̅   1 2̅ 4̅
\ 2̅8̅   4̅
Result 4̅ 2̅8̅.
```

That is exactly the first entry of the table at the head of the papyrus: two
divided by seven is 4̅ 2̅8̅. The next chapter is about that table.
