# 8. Reading the papyrus

A transcription says only what the papyrus says. It never corrects the
scribe: if the papyrus has 15 where 14 is right, the file has 15. The marks
it uses for the state of the papyrus:

| written | means |
|---|---|
| `{5}` | in red ink |
| `{Total} 21.` | a word of a statement in red |
| `[5]` | restored by an edition: the papyrus is damaged here |
| `[...]` | lost |
| `5?` | read, but uncertain |
| `% …` | a note, not in the papyrus |
| `"…"` | the scribe's words, in translation |

A restored figure is used for what follows, but never counted right or wrong:
it tests the edition, not the scribe.

```rmp
  1    9
  2    [18]
\ 4    36
```

```
  1    9                                ✓
  2    [18]                             ?
\ 4    36                               ✓

2 figures checked, 2 right, 0 wrong.
```

The whole papyrus is in `papyrus/`, one file a problem, read from the page
images of Chace's and Peet's editions by a coding agent and its sub-agents.
Every disagreement between the two editions is in a note on its line.
`rhind verify` runs it all.
