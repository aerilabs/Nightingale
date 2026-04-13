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

## len() and is_empty()

```rust
pub fn len(&self) -> usize
pub fn is_empty(&self) -> bool
```

Returns the cached document byte length or boolean emptiness. These are O(1) operations because the length is cached in `self.len`, not recalculated from pieces.

---

## to_string() and Display trait

```rust
pub fn to_string(&self) -> String
fn fmt(&self, f: &mut Formatter) -> fmt::Result  // Display impl
```

Reconstructs the full text by walking all pieces in order and appending each slice to a result string. Uses safe slicing with `checked_add` and `str::get()` to prevent panics from overflow or invalid UTF-8 boundaries:

```rust
for piece in &self.pieces {
    let source = match piece.buffer {
        Original => &self.original,
        Add      => &self.add,
    };
    let end = piece.start.checked_add(piece.len).ok_or(fmt::Error)?;
    let segment = source.get(piece.start..end).ok_or(fmt::Error)?;
    write!(f, "{}", segment)?;
}
```

**Safety:** Uses `checked_add` to detect overflow and `str::get()` to safely handle out-of-bounds or multi-byte char boundaries.

---

## Insert

```rust
pub fn insert(&mut self, pos: usize, text: &str) -> Result<(), String>
```

Inserts `text` at byte position `pos` in the document. Returns an error if validation fails; the piece table is never left in an inconsistent state.

### Validation (before mutation)

1. **Empty check:** Return error if `text` is empty
2. **Bounds check:** Return error if `pos > self.len` (cached document length)
3. **UTF-8 boundary check:** Reconstruct the full document and verify `pos` is on a char boundary using `doc.is_char_boundary(pos)`

### Insertion (after validation passes)

1. Record `add_start = self.add.len()` before appending
2. Append `text` to the add buffer (mutation safe now)
3. Walk pieces accumulating `offset` to find which piece contains `pos`
4. Calculate `split = pos - offset` (the split point within that piece)
5. Replace the piece at index `i` with three new pieces
6. Increment `self.len` by `text.len()`

### Key design choice

**Validate before mutating:** All validation happens before `self.add.push_str()`. If any check fails, the piece table remains unchanged.

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
pub fn delete(&mut self, pos: usize, len: usize) -> Result<(), String>
```

Deletes `len` bytes starting at byte position `pos` in the document. Returns an error if the deletion range is invalid or crosses piece boundaries; the piece table remains unchanged.

### Validation (before mutation)

1. **Bounds check:** Return error if `pos > doc_len` (document length)
2. **Range check:** Return error if `len == 0` (no-op but allowed)
3. **Overflow check:** Return error if `pos + len > doc_len` (deletion extends past document end)
4. **Single-piece constraint:** Return error if `split + len > piece.len` (deletion would cross into adjacent piece)

### Deletion (after validation passes)

1. Walk pieces accumulating `offset` to find which piece contains `pos`
2. Calculate `split = pos - offset` (start position within the piece)
3. Replace the piece at index `i` with two new pieces: left and right halves
4. Update `self.len` by decrementing by `len` bytes

### Key design choice

**Validate before mutating and fail atomically:** All validation happens before any mutation. If any check fails, the piece table remains entirely unchanged. Single-piece deletion only — deletions spanning multiple pieces are not yet supported.

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

- **Empty pieces can accumulate** — the current `insert` implementation skips zero-length left/right pieces at piece boundaries, but other operations (for example, `delete` when it records both split halves) may still leave zero-length pieces behind. Harmless but inefficient over time. A cleanup pass can remove them.
- **Byte indices only** — `start` and `len` are byte offsets, not character counts. The API requires callers to supply byte offsets and UTF-8 char boundaries; character-level navigation is deferred to the `Cursor` wrapper.
- **Single-piece deletion only** — the current delete implementation only handles deletions that fall within a single piece. Deletions spanning multiple pieces are not yet supported.
