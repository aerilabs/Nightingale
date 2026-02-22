## _Piece Table_
A **piece table** is a data structure used by text editors to manage and modify text efficiently without changing the original content. Instead of repeatedly editing a large string—which is slow and memory-intensive—it tracks changes by referencing pieces of text stored in separate buffers.

It has two main components:

* **Original buffer** – contains the file’s initial text and is never modified.
* **Add buffer** – stores all newly inserted text.

Rather than editing text in place, the piece table maintains a sequence of *pieces* (references) that point to segments in these buffers. When text is inserted or deleted, the editor simply updates these references instead of rewriting the entire document. This makes operations like insertion, deletion, and undo highly efficient, even for very large files.

### Insert function
- It works with the following formula as its backbone:
``` markdown
left  = Piece { start: piece.start, len: split_offset }
right = Piece { start: piece.start + split_offset, len: piece.len - split_offset }
```
