# Piece Table

A **piece table** is a data structure used by text editors to manage and modify text efficiently without changing the original content. Instead of repeatedly editing a large string — which is slow and memory-intensive — it tracks changes by referencing pieces of text stored in separate buffers.

It has two main components:

- **Original buffer** – contains the file's initial text and is never modified.
- **Add buffer** – stores all newly inserted text, append-only. Text is never removed from it.

Rather than editing text in place, the piece table maintains a sequence of *pieces* (references) that point to segments in these buffers. When text is inserted or deleted, the editor simply updates these references instead of rewriting the entire document.

**The text still exists in the buffer. You just stop pointing to it.**

---

## Piece

A piece has three fields:

```
buffer  – which buffer it points to (Original or Add)
start   – the byte index in that buffer where this piece begins
len     – how many bytes this piece covers
```

To reconstruct the full document, you walk every piece in order, slice its buffer from `start` to `start + len`, and concatenate the results.

### Example

Original buffer: `"hello world"`
Add buffer: `""`
Pieces: `[ { Original, 0, 11 } ]`

Reconstructed text: `"hello world"` ✓

---

## Constructor

```rust
pub fn new(text: String) -> Self
```

Creates a piece table from an initial string. The entire text becomes one piece pointing to the original buffer.

```
original: "hello world"
add:      ""
pieces:   [ { Original, start: 0, len: 11 } ]
```

If the initial string is empty, the piece has `len: 0`. This is harmless — slicing `[0..0]` returns an empty string and `push_str("")` does nothing.

---

## to_string()

```rust
pub fn to_string(&self) -> String
```

Reconstructs the full text by walking all pieces in order and appending each slice to a result string.

```rust
for piece in &self.pieces {
    let source = match piece.buffer {
        Original => &self.original,
        Add      => &self.add,
    };
    result.push_str(&source[piece.start..piece.start + piece.len]);
}
```

**Note:** If `piece.start + piece.len` exceeds the buffer length, Rust will panic at the slice indexing line. There is currently no bounds guard — pieces must always be constructed correctly.

---

## Insert

```rust
pub fn insert(&mut self, pos: usize, text: &str)
```

Inserts `text` at byte position `pos` in the document.

### How it works

The add buffer is append-only, so the new text is appended to it first. Then the piece containing `pos` is split into three pieces: the left half, the new text, and the right half. The original buffer is never touched.

### Algorithm

1. Record `add_start = self.add.len()` before appending
2. Append `text` to the add buffer
3. Walk pieces accumulating `offset` to find which piece contains `pos`
4. Calculate `split = pos - offset` (the split point within that piece)
5. Replace the piece at index `i` with three new pieces

### Piece formulas

```
left  = { buffer: piece.buffer,  start: piece.start,               len: split            }
new   = { buffer: Add,           start: add_start,                 len: text.len()       }
right = { buffer: piece.buffer,  start: piece.start + split,       len: piece.len - split}
```

### Visual example

Document: `"hello world"` → one piece `{ Original, 0, 11 }`

Call `insert(5, ",")`:

```
Before:
[ { Original, 0, 11 } ]        →  "hello world"

After:
[ { Original, 0, 5  } ]        →  "hello"
[ { Add,      0, 1  } ]        →  ","
[ { Original, 5, 6  } ]        →  " world"
```

Reconstructed: `"hello, world"` ✓

The original buffer still says `"hello world"` — untouched.

---

## Delete

```rust
pub fn delete(&mut self, pos: usize, len: usize)
```

Deletes `len` bytes starting at byte position `pos` in the document.

### How it works

The piece containing `pos` is split into two pieces: the left half (before the deletion) and the right half (after the deletion). The middle region simply has no piece pointing to it anymore — it is not erased from the buffer, just no longer referenced.

### Algorithm

1. Walk pieces accumulating `offset` to find which piece contains `pos`
2. Calculate `split = pos - offset`
3. Guard against overflow: if `split + len > piece.len`, the deletion goes out of bounds — skip
4. Replace the piece at index `i` with two new pieces

### Piece formulas

```
left  = { buffer: piece.buffer,  start: piece.start,                    len: split                  }
right = { buffer: piece.buffer,  start: piece.start + split + len,      len: piece.len - split - len}
```

### Visual example

Document: `"hello world"` — indices:

```
h  e  l  l  o     w  o  r  l  d
0  1  2  3  4  5  6  7  8  9  10
```

Call `delete(3, 5)` — remove 5 characters starting at index 3 (`"lo wo"`):

```
Before:
[ { Original, 0, 11 } ]        →  "hello world"

After:
[ { Original, 0, 3 } ]         →  "hel"
[ { Original, 8, 3 } ]         →  "rld"
```

Reconstructed: `"helrld"` ✓

The deleted text `"lo wo"` still exists in the original buffer — there is just no piece pointing to it.

---

## Key invariants to never break

- The original buffer is **never modified** after construction
- The add buffer is **append-only** — text is never removed from it
- All piece `start` and `len` values must stay within bounds of their buffer
- Pieces are always stored in document order

---

## Known limitations (to address later)

- **Empty pieces accumulate** — inserting at a piece boundary creates a left piece with `len: 0`. Harmless but inefficient over time. A cleanup pass can remove zero-length pieces.
- **Byte indices only** — `start` and `len` are byte offsets, not character counts. Multi-byte Unicode characters (anything outside ASCII) will cause incorrect splits or panics if a split lands inside a multi-byte character. Unicode support requires tracking character boundaries separately.
- **Single-piece deletion only** — the current delete implementation only handles deletions that fall within a single piece. Deletions spanning multiple pieces are not yet supported.
