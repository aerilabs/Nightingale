# Nightingale Rust Code Editor Checklist

## Phase 1: Text Engine (Piece Table)

- [ ] Create a new Rust project
- [ ] Create a `PieceTable` struct
- [ ] Store original file text
- [ ] Store add buffer (append-only)
- [ ] Define `Piece` structure
- [ ] Load a file into the piece table
- [ ] Implement `to_string()` (reconstruct full text)

## Phase 2: Cursor & Editing

- [ ] Create `Cursor` struct
- [ ] Track cursor position (logical index)
- [ ] Move cursor left
- [ ] Move cursor right
- [ ] Insert character at cursor
- [ ] Delete character before cursor
- [ ] Prevent cursor from going out of bounds
- [ ] Write tests for cursor movement
- [ ] Write tests for cursor editing

## Rules (Do Not Break)

- [ ] No UI code
- [ ] No rendering
- [ ] No async
- [ ] No crates beyond std


Phase 3 Checklist (Insert & Delete)

## Core Piece Table Enhancements

- [ ] Implement `PieceTable::insert(pos: usize, text: &str)` that:
    - [ ] Splits the piece where insertion happens
    - [ ] Adds new text to the add buffer
    - [ ] Updates pieces vector to include the new piece
- [ ] Implement `PieceTable::delete(start: usize, len: usize)` that:
    - [ ] Splits pieces at deletion range
    - [ ] Removes pieces or trims them
    - [ ] Keeps original and add buffers immutable
- [ ] Write unit tests for `insert`:
    - [ ] Insert at start
    - [ ] Insert in middle
    - [ ] Insert at end
- [ ] Write unit tests for `delete`:
    - [ ] Delete single character
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

