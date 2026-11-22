# 1. The papyrus, and a first page

Around 1550 BCE a scribe called Ahmose copied a book of arithmetic onto a
roll of papyrus five metres long. He says he copied it from a writing "made
in the likeness of writings of old", two or three centuries older. It was
bought in Luxor in 1858 by Alexander Henry Rhind, and most of it is in the
British Museum. It is the largest work of Egyptian mathematics that survives.

It opens with a table: two divided by every odd number from 3 to 101,
written as sums of unit fractions. Then come 87 problems: loaves shared among
men, a quantity and its parts, granaries, fields, pyramids, beer and bread.
Almost every problem shows its working, and the working is the point. It
shows how a person calculated with the operations they had: doubling,
halving, two-thirds, tenths, and the table.

`rhind` reads a page of that working, does every step again with the same
operations, and says where the scribe's figure and the working disagree.

```rmp
No. 26
"A quantity whose fourth part is added to it becomes 15."
  1   4
 '4   1
Total 5.
Reckon with 5 to find 15.
\ 1   5
\ 2   10
Result 3.
```

```
No. 26
"A quantity whose fourth part is added to it becomes 15."
  1   4                                 ✓
 4̅   1                                  ✓
Total 5.                                ✓
Reckon with 5 to find 15.
\ 1   5                                 ✓
\ 2   10                                ✓
Result 3.                               ✓

6 figures checked, 6 right, 0 wrong.
```

Each line it could check has a mark: ✓ where the scribe's figure is the one
the working gives. This is Problem 26, with its first step set out as a
column (the papyrus says it in words), and it is all right. The next chapters say what each line means. To follow along, put a page
in a file and run `rhind FILE`, or `rhind -` and type it in.

`rhind no 26` prints the papyrus's own Problem 26, as transcribed from the
editions of Chace (1929) and Peet (1923).
