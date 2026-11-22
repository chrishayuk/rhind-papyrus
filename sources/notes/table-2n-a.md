# Notes: table of 2 ÷ n, entries 2 ÷ 3 to 2 ÷ 49

File: `papyrus/table-2n-a.rmp`. Chace vol. II, PDF pp. 51-78 (literal translation on odd pages, plates 2-15 on even pages).
Peet 1923, printed pp. 37-43 (PDF pp. 41-47): p. 37 is a summary table of all resolutions; pp. 38-43 give each sum's working.

All reading was done by a coding agent's sub-agents (one agent for this range). No person has checked it.

## How it was read

- Chace's literal-translation pages were read from the page photographs embedded in the PDF (3264 × 2448 pixels),
  cropped and enlarged. Every figure, dot (unit-fraction mark), double dot (2/3), bracket, tick and red/black colour
  was read off the images, not from OCR.
- Peet's pages were rendered at 150 dpi and read whole.
- Chace's plates (hieratic + hieroglyphic) were looked at for 2 ÷ 3 to 2 ÷ 11 (plates 2-3) and 2 ÷ 37/39 (plate 11),
  mainly to understand the restorations in 2 ÷ 11 and the layout. For the other entries the translation page, which
  prints red ink in red, was taken as Chace's reading; the plates were not read glyph by glyph.

## Conventions used in the file (what the notation could not hold directly)

- **Summary line.** Each entry opens with a line of pairs: red unit fraction, then in black what it gives of n. It is
  written one pair to a row (`{'4}  1 '2 '4`), in the papyrus's order. In the papyrus they are all on one line.
- **Double statements.** Rows such as `\ 4  28  '4` (2 ÷ 7) or `  1  '17` (2 ÷ 17) are the scribe's compressed form:
  "4 times n is 28, and '28 of n is '4". Chace explains this in his note to 2 ÷ 7 and 2 ÷ 17; the middle figure is
  sometimes dotted, sometimes not (Chace: about 50 dotted, 30 not, in the table). Written as Chace prints them; a
  checker reading the first two figures as multiplier/product of n will flag the dotted ones (e.g. `2  '34` in 2 ÷ 17)
  and the "1 '24" rows of 2 ÷ 29/31; these are the scribe's notation, not necessarily arithmetic errors.
- **Divisor at the head.** From 2 ÷ 31 on (and over the working of 2 ÷ 5) the divisor stands alone in black at the head
  of the entry. Written as a quoted line (`"31"`). Peet renders it as the first row `1  31` (p. 38: "the number by which
  2 is divided has the unity dot in front of it ... we have inserted it").
- **"Remainder" (ḏꜣ·t)** lines are written as `Remainder X.` statements. In 2 ÷ 17 this row is ticked in the papyrus; the
  notation cannot tick a statement, so it is said in a `%` note. The remainder there is 2 less what has been found,
  not the result of a written subtraction.
- **"Total" (dmd)** in 2 ÷ 19, 23, 41 stands on the same line as a ticked row, in front of it. Written as a quoted line
  `"Total"` before the row. In 2 ÷ 23 the line has no multiplier: `\ '276  '12` stands in the product column with the
  tick in front of '276; it is written as a row with '276 on the left, which is checkable ('276 of 23 is '12) but
  moves the figure into the multiplier column.
- **"Find" (gm)** in 2 ÷ 43, 47, 49 stands in the multiplier column; written as a quoted line `"Find"` before the row.
- **Partial restorations** of a number are written at digit level as Chace prints them: `[1]8` (2 ÷ 9), `1[3]`
  (2 ÷ 13), `[2]2`, `[4]4` (2 ÷ 11). The notation's `[...]` is defined for whole figures; this may need parser support.
- **Restored tick.** In 2 ÷ 11 the whole left column, tick included, is Chace's restoration; the notation cannot mark a
  tick as restored, so the figures are bracketed and a note says so.
- **Red ticks.** Chace prints the ticks of 2 ÷ 5, 7 and 9 in red (from 2 ÷ 11 on they are black). Not representable.
- **Red words.** "Call" / "Call 2" (nyś) and "Working-out:" (šsm·t) are red: `{"Call 2"} "out of 17"`,
  `{"Working-out:"}`.
- **Missing dots** that Chace notes (the scribe wrote a whole number where a unit fraction is meant) are copied as
  whole numbers: `{6}` in 2 ÷ 9, `6` in 2 ÷ 11 and 2 ÷ 17, `{42}` in 2 ÷ 21, `{30}`, `{42}`, `42` in 2 ÷ 35, `{42}  1 42`
  in 2 ÷ 43, `174` (2 ÷ 29), `124` (2 ÷ 31), and the undotted middles `28`, `22`, `26`, `78`, `30`, `76`, `296`. A checker
  will flag them.
- **2 ÷ 35 auxiliary figures** (red 6, black 7 and 5) are written as quoted words, since they are counts of sixths,
  not a multiplication: `{"6"} "7" "5"`.

## Entries

All 24 entries written: 2 ÷ 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45, 47, 49.

Read with confidence: all figures in every entry. The points below are the only places with any doubt.

### Uncertain readings

- **2 ÷ 37**: Chace prints a small stroke above and below the '2 of the summary "1 '2 '24", and above/below the '3 of
  the remainder "'3 '8". Read as '2 and '3 (Peet agrees); the strokes may be type marks or flecks on the photograph.
- **2 ÷ 11**: the extent of Chace's bracket over the left column is "[1 ... 1 2/3 '6]", read as the whole column restored
  (the plate shows nothing there, and Peet prints the same bracket). The right-hand "11" in the first row is printed
  unbracketed by Chace though the plate shows nothing there either; followed as printed.
- **2 ÷ 9**: "[2  1]8  '2": read as the multiplier 2 and the first digit of 18 restored.
- **2 ÷ 35**: the position of the red 6 (Chace: under the 35; Peet: at the start of the line).

## Chace / Peet disagreements

Peet says (p. 38) he corrected the fractional dots "without further comment" throughout these sums, so where Chace
has an undotted figure and Peet a dotted one (or the reverse) this is Peet's normalisation, not a different reading.
Those are listed last.

1. **2 ÷ 5**: Peet's footnote to the '3 row: "Wrongly given as 1 '3 in the plate" (Peet's own Plate A). Both editions
   read 1 2/3.
2. **2 ÷ 9**: last row. Chace `\ [2]  [1]8  '2` (two multiplications); Peet `\ '18  '2`, "The papyrus is damaged, but
   the reading certain."
3. **2 ÷ 11**: right column. Chace `[1]  11`, `\ 6  66  6`; Peet `[1  1]1`, `\ 6  [6]6  '6` (Peet restores the first digit
   of 66, and the first digit of 11 in row 1; Peet dots the last 6).
4. **2 ÷ 13**: Chace `1  1[3]`; Peet `1  13`, no restoration.
5. **2 ÷ 19**: Chace has "dmd" (Total) in front of `\ 6  '114  '6`; Peet has no "Total" there.
6. **2 ÷ 23**: Chace ticks the total line `\'276  '12`; Peet "Total '276 '12", no tick.
7. **2 ÷ 29**: Peet prints a first row `1  29` that Chace does not have (Peet's inserted "unity" line).
8. **2 ÷ 31, 33, 35, 39, 43, 45, 47, 49**: Chace has the divisor alone at the head of the entry; Peet sets it as the first
   row `1  n` of the working. (Same thing, different layout.)
9. **2 ÷ 35**: the red 6 of the auxiliary line: Chace under the 35, Peet "to be read not under the 35 but at the
   beginning of the line containing the 7 and 5".
10. **2 ÷ 41**: Peet prints a first row `1  41` in the left column that Chace does not have.
11. **2 ÷ 43**: Chace `{'66}  '2` and `\ 2  '66  '2` (the scribe's sign for 60 where 80 was meant; Chace: "intended to
    be '86"); Peet prints '86 in both places, without comment. Chace `"Find" \ 42  1 '42` (tick before the 42, in the
    middle column); Peet `Found \ '42  1 '42`.
12. Dots (Peet's normalisation): 2 ÷ 7 `28` (Peet '28, footnote "Actually 28"); 2 ÷ 9 `{6}` (Peet '6); 2 ÷ 11 third `6`
    (Peet '6); 2 ÷ 17 `6` (Peet '6); 2 ÷ 19 `'19 '38 '76` in the second column and `'19 '38 76 '114` in the third (Peet 19,
    38, 76, 114); 2 ÷ 21 `'14`, `{42}` (Peet 14, '42); 2 ÷ 29 `174` (Peet '174); 2 ÷ 31 `124` (Peet '124); 2 ÷ 33 `22`
    (Peet '22); 2 ÷ 35 `{30}`, `42` (Peet '30, '42); 2 ÷ 39 `26`, `78` (Peet '26, '78); 2 ÷ 41 `'27` (Peet 27);
    2 ÷ 43 `{42} 1 42` (Peet '42, 1 '42); 2 ÷ 45 `30` (Peet '30).

No disagreement in any tick other than 2 ÷ 23 (item 6). Both editions leave unticked: the whole left column of 2 ÷ 19;
`2/3 22` in 2 ÷ 33; `2 78` in 2 ÷ 39; `3 '129` in 2 ÷ 43; the "Find" rows of 2 ÷ 47 and 2 ÷ 49.

## Scribal errors the editions point out

- **2 ÷ 9** (Chace): "The fractional dot is omitted in the first term of the answer written at the top, although this
  term is '6 and not the whole number 6."
- **2 ÷ 11** (Chace, footnote): the dot is omitted with the last 6 on the plate, "but there may have been a dot on the
  papyrus, obscured now by a crack or fold".
- **2 ÷ 17** (Chace, footnote): the 6 of the fourth row "should be '6 and not 6".
- **2 ÷ 19** (Chace): the dot before the red '76 "seems to have been made in black and then corrected to red"; in the
  third column "the 76 is written without a dot although the first two partial products are dotted".
- **2 ÷ 21** (Chace): "The fractional dot is omitted in the second term of the answer" (42 for '42), as also with the sign
  for 40 in 2 ÷ 35 (twice), 43 (twice), 63, 67, 69, 71.
- **2 ÷ 29** (Chace): "The scribe has been careless here" — the "1" at the start of the first line. (Peet: "1 (sic)",
  "The multiplier 1 (a dot) with a tick is obviously an error".)
- **2 ÷ 31** (Chace): "Here again the scribe has by mistake put '1' at the top of the first column, opposite '20."
  (Peet: "1 (sic)", "The dot 1 in front of '20 is an error".)
- **2 ÷ 35** (Chace, footnote): the dot is omitted with the red 30 on the plate, "but it is possible that a dot on the
  papyrus is concealed under the patch"; the 42 lacks its dot (note to 2 ÷ 21).
- **2 ÷ 41** (Chace): "The scribe has carelessly added a fractional dot to the 27."
- **2 ÷ 43** (Chace): "The sign for 60 is used here twice in what was intended to be '86" (also in 2 ÷ 83 and twice in
  2 ÷ 93); the 42s lack their dots (note to 2 ÷ 21).
- Peet (p. 38), generally: "there are endless scribe's errors, the most usual being the omission of the tick ... and the
  incorrect omission or insertion of the fractional dot", the dots "corrected without further comment".
- Peet notes tacit assumptions (not errors): 2 ÷ 13, that 1 '2 '8 with '4 and '8 makes 2; 2 ÷ 17, that 1 '4 '6 is
  1 '3 '12; 2 ÷ 23, that 1 '2 '4 '6 is 1 2/3 '4.
