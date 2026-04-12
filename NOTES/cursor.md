# Cursor

A **cursor** is a wrapper around a byte position in a `PieceTable` that provides character-level navigation and editing operations.

The `PieceTable` works with byte offsets and requires callers to respect UTF-8 character boundaries. The `Cursor` abstracts this away by automatically detecting and respecting character boundaries, making it safe to insert and delete characters without manually calculating UTF-8 byte offsets.

## Overview

```rust
pub struct Cursor {
    index: usize,  // cached byte position in the document
}
```

The cursor maintains a single field: the byte position in the document. It delegates all text operations to its associated `PieceTable`.

---

## new() and default()

```rust
pub fn new() -> Self
pub fn default() -> Self  // same as new()
```

Creates a cursor at position 0 (start of the document).

---

## get_index()

```rust
pub fn get_index(&self) -> usize
```

Returns the current byte position in the document.

---

## move_left()

```rust
pub fn move_left(&mut self, table: &PieceTable)
```

Moves the cursor one character to the left (towards position 0).

### How it works

1. If already at position 0, return immediately (no-op)
2. Reconstruct the full document from the piece table
3. Decrement `index` by one byte and walk backwards until landing on a valid UTF-8 character boundary
4. Update cursor position

### UTF-8 safety

In a UTF-8 string, only certain byte positions are valid character boundaries (marked by `is_char_boundary()`). A multi-byte character like `é` occupies 2 bytes. If the cursor is positioned after the second byte, stepping back one byte lands in the middle of the character, which is invalid. So `move_left()` keeps stepping back until it hits a valid boundary.

### Example

```
Text: "café" (4 bytes: c a f é[0xC3 0xA9])
Positions: 0 1 2 3 4 5

Cursor at 5 (end) → move_left() → lands at 4
Cursor at 4 → move_left() → steps back to 3, 2 (both invalid), landing at 2 (char 'f')
```

---

## move_right()

```rust
pub fn move_right(&mut self, table: &PieceTable)
```

Moves the cursor one character to the right (towards the document end).

### How it works

1. Reconstruct the full document
2. If cursor is already at or past the end (index >= doc.len()), return immediately
3. Increment `index` by one byte and walk forward until landing on a valid UTF-8 character boundary (or document end)
4. Cap position at document length and update cursor

### Example

```
Text: "café" (5 bytes: c a f é[0xC3 0xA9])
Positions: 0 1 2 3 4 5
Valid UTF-8 boundaries: 0 1 2 3 5

Cursor at 0 (before 'c') → move_right() → steps to 1, lands at 1 (before 'a')
Cursor at 3 (before 'é') → move_right() → steps to 4 (invalid), then 5, lands at 5 (end)
```

---

## insert_char()

```rust
pub fn insert_char(&mut self, table: &mut PieceTable, c: char) -> Result<(), String>
```

Inserts a single character at the cursor position and advances the cursor past the inserted character.

### How it works

1. Encode the character into UTF-8 bytes using a fixed stack buffer (`[0; 4]`)
2. Call `table.insert(self.index, encoded_str)` to insert the encoded character
3. Advance cursor by the character's UTF-8 byte length using `c.len_utf8()`

### Why stack buffer?

Characters in Rust are always valid Unicode and encode to at most 4 bytes. Using a fixed 4-byte buffer on the stack avoids heap allocation:

```rust
let mut buf = [0; 4];
let encoded = c.encode_utf8(&mut buf);  // &str pointing to buf
table.insert(cursor_pos, encoded)?;     // efficient, no allocation
```

### Example

```
Text: "Hllo" (cursor at 1)
insert_char('e'):
  → encode to "e"
  → insert at pos 1
  → advance cursor by 1
  → result: "Hello", cursor at 2
```

---

## delete_char()

```rust
pub fn delete_char(&mut self, table: &mut PieceTable) -> Result<bool, String>
```

Deletes the character immediately before the cursor (backspace behavior). Returns `Ok(true)` if a character was deleted, or `Ok(false)` if cursor was at position 0.

### How it works

1. Return `Ok(false)` if cursor is at position 0 (nothing to delete)
2. Reconstruct the document and validate:
   - Cursor position is within document bounds
   - Cursor is on a valid UTF-8 character boundary
3. Find the start of the previous character using `char_indices()`:
   - Slice the document up to cursor position
   - Get `char_indices()` to find all (byte_offset, char) pairs
   - Extract the byte offset of the last character before cursor
4. Call `table.delete(previous_index, cursor_index - previous_index)` to remove the character
5. Move cursor backwards to `previous_index`
6. Return `Ok(true)`

### Example

```
Text: "Café" (cursor at 5, after é)
- char_indices() on "Caf" → [(0, 'C'), (1, 'a'), (2, 'f')]
- last one is at offset 2
- delete(2, 5 - 2 = 3) removes the é
- result: "Caf", cursor at 2
```

---

## Design patterns

### Document reconstruction cost

Both `move_left()` and `move_right()` reconstruct the entire document with `table.to_string()` to check UTF-8 boundaries. This is **O(n)** in document size. Frequent cursor movements in large documents will be slow.

**Future optimization:** Cache or parse the piece table structure directly to find UTF-8 boundaries without full reconstruction.

### Stack allocation for encoding

`insert_char()` uses a fixed 4-byte stack buffer to encode the character, avoiding heap allocation for each insertion. This is efficient and safe because all Rust `char` values are valid Unicode and fit in 4 bytes.

### Error handling

- `insert_char()` and `delete_char()` return `Result` to propagate errors from the piece table
- Both return `Err` if the piece table operation fails or internal state becomes inconsistent
- `move_left()` and `move_right()` are infallible (they never error; they no-op at boundaries)

---

## Invariants

- Cursor `index` must always be a valid UTF-8 character boundary in the document (enforced by move operations)
- Cursor `index` must always be `<= document length` (enforced by bounds checks and capping)
- After any mutation (insert/delete), cursor is positioned correctly relative to the changed text
