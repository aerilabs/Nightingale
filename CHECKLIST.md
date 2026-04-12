# Nightingale Rust Code Editor Checklist

## Phase 1: Text Engine (Piece Table)

- [x] Create a new Rust project
- [x] Create a `PieceTable` struct
- [x] Store original file text
- [x] Store add buffer (append-only)
- [x] Define `Piece` structure
- [ ] Load a file into the piece table
- [x] Implement `to_string()` (reconstruct full text)

## Phase 2: Cursor & Editing

- [x] Create `Cursor` struct
- [x] Track cursor position (logical index)
- [x] Move cursor left
- [x] Move cursor right
- [x] Insert character at cursor
- [x] Delete character before cursor
- [x] Prevent cursor from going out of bounds
- [x] Write tests for cursor movement
- [x] Write tests for cursor editing

## Rules (Do Not Break)

- [x] No UI code
- [x] No rendering
- [x] No async
- [x] No crates beyond std


Phase 3 Checklist (Insert & Delete)

## Core Piece Table Enhancements

- [x] Implement `PieceTable::insert(pos: usize, text: &str)` that:
    - [x] Splits the piece where insertion happens
    - [x] Adds new text to the add buffer
    - [x] Updates pieces vector to include the new piece
- [x] Implement `PieceTable::delete(start: usize, len: usize)` that:
    - [x] Splits pieces at deletion range
    - [x] Removes pieces or trims them
    - [x] Keeps original and add buffers immutable
- [x] Write unit tests for `insert`:
    - [x] Insert at start
    - [x] Insert in middle
    - [x] Insert at end
- [ ] Write unit tests for `delete`:
    - [x] Delete single character
    - [ ] Delete range across pieces
    - [ ] Delete at start/end

## Cursor Integration

- [ ] Update `Cursor::insert_char(&mut self, table: &mut PieceTable, c: char)` to use new `insert`
- [ ] Update `Cursor::delete_char(&mut self, table: &mut PieceTable)` to use new `delete`
- [ ] Ensure cursor index updates correctly after:
    - [ ] Insertion
    - [ ] Deletion
    - [ ] Edge cases (start/end of document)
- [ ] Write cursor + piece table integration tests:
    - [ ] Insert characters at different positions
    - [ ] Delete characters at different positions
    - [ ] Verify `to_string()` matches expected result
    - [ ] Verify cursor index is correct

## Optional / Bonus (for stability)

- [ ] Implement `Cursor::move_to(pos: usize)` to jump anywhere
- [ ] Add basic undo stack (optional for Phase 3)
- [ ] Edge case tests:
    - [ ] Insert/delete on empty document
    - [ ] Insert/delete with only add buffer
    - [ ] Cursor at document boundaries

