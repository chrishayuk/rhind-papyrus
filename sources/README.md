# Sources

The editions are not kept in the repository. They are public domain and on
the Internet Archive; fetch them into `pdf/` if you want to check a reading.
What is kept is what was read out of them: the transcriptions in
`../papyrus/`, the notes on each range in `notes/`, and the brief the
transcribers worked to, `TRANSCRIBING.md`.

| File in `pdf/` | Edition | Internet Archive item |
|---|---|---|
| `chace-1929-vol2.pdf` | A. B. Chace, L. Bull, H. P. Manning, *The Rhind Mathematical Papyrus*, vol. II: *Photographs, Transcription, Transliteration, Literal Translation*. Mathematical Association of America, Oberlin, 1929. **The primary witness.** | `arnoldbuffumchaceludlowbullhenryparkermanningtherhindmathematicalpapyrus.volumei` |
| `peet-1923.pdf` | T. E. Peet, *The Rhind Mathematical Papyrus, British Museum 10057 and 10058*. University Press of Liverpool / Hodder & Stoughton, 1923 (the scan is of a 1970 reprint; same pages). **The second witness.** | `Peet_1923` |
| `chace-1927-vol1.pdf` | Chace, vol. I: *Free Translation and Commentary*, 1927. Read for the commentary only. | `TheRhindPapyrusVolume1` |
| `eisenlohr-1877.pdf` | A. Eisenlohr, *Ein mathematisches Handbuch der alten Aegypter*, 1877. Not read. | `einmathematische00eise` |

Downloaded on 22 November 2026, for example:

```bash
curl -L -o pdf/peet-1923.pdf "https://archive.org/download/Peet_1923/Peet%2C%20T.%20Eric%20-%20The%20Rhind%20mathematical%20papyrus%20British%20Museum%2010057%20and%2010058%20%281970%29%20LR.pdf"
```

(`https://archive.org/metadata/<item>/files` lists each item's files.)

## Pages

Chace vol. II, PDF pages: 51-112 the table of 2 ÷ n (odd page: literal
translation; even page: plate), 113-114 2 ÷ 101 and the numbers 1-9 ÷ 10,
115-266 Problems 1-87, a literal translation page facing each plate. Peet:
PDF page = printed page + 4 throughout.

## How it was read

Every figure was read from page images (`pdftoppm -r 200` or more), never
from OCR: the OCR loses the bar that makes a number a fraction, and the bars
are the point. Each range was read by one sub-agent of a coding agent, from
Chace first and Peet second. Every disagreement between the two, and every
place either says the scribe erred, is a `%` note on its line and is listed
in `notes/`. No person has checked the reading.

Afterwards `rhind verify` listed every place an edition says the scribe
erred where `rhind` found the line right. Some of those were transcription
errors (a part sign the transcriber put back) and were corrected to what the
papyrus has.
