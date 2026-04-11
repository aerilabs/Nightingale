#[derive(Clone, Copy)]
// The 2 buffers: one for original text, the other for added text
enum BufferKind {
    Original,
    Add,
}

#[derive(Clone, Copy)]
struct Piece {
    buffer: BufferKind,
    start: usize,
    len: usize,
}

pub struct PieceTable {
    original: String,
    add: String,
    pieces: Vec<Piece>,
}

// Minimal Constructor for PieceTable
impl PieceTable {
    pub fn new(text: String) -> Self {
        let len = text.len();
        Self {
            original: text,
            add: String::new(),
            pieces: vec![Piece {
                buffer: BufferKind::Original,
                start: 0,
                len,
            }],
        }
    }

    // Reconstruct the full text from the piece table
    /// Reconstructs the current document text by concatenating the slices
    /// referenced by the piece table.
    ///
    /// The `start` and `len` values stored in each piece are byte-based
    /// indices into either the original or add buffer.
    pub fn to_string(&self) -> String {
        let mut result = String::new();

        for piece in &self.pieces {
            let source = match piece.buffer {
                BufferKind::Original => &self.original,
                BufferKind::Add => &self.add,
            };

            // Takes the range of the string in terms of the index, from the beginning to the end
            result.push_str(&source[piece.start..(piece.start + piece.len)]);
        }

        result
    }

    pub fn delete(&mut self, pos: usize, len: usize) -> Result<(), String> {
        let doc_len: usize = self.pieces.iter().map(|p| p.len).sum();
        if pos > doc_len {
            return Err(format!(
                "delete position {pos} is out of bounds (document length is {doc_len})"
            ));
        }

        if len == 0 {
            return Ok(());
        }

        if len > doc_len - pos {
            return Err(format!(
                "delete range [{pos}, {}) is out of bounds (document length is {doc_len})",
                pos + len
            ));
        }

        let mut offset = 0usize;
        for i in 0..self.pieces.len() {
            let piece = self.pieces[i];

            if pos >= offset && pos < (offset + piece.len) {
                let split = pos - offset;

                // Guard to prevent overflow
                if split + len > piece.len {
                    return Err(format!(
                        "invalid delete range: {pos} {len} exceeds bounds of current piece ({}, {offset})",
                        piece.len
                    ));
                }

                let left_piece = Piece {
                    buffer: piece.buffer,
                    start: piece.start,
                    len: split,
                };

                let right_piece = Piece {
                    buffer: piece.buffer,
                    start: piece.start + split + len,
                    len: piece.len - split - len,
                };

                self.pieces.remove(i);
                self.pieces.insert(i, right_piece);
                self.pieces.insert(i, left_piece);
                break;
            }
            offset += piece.len;
        }
        Ok(())
    }

    /// Inserts `text` at the given byte offset `pos`.
    ///
    /// # Byte Offsets
    /// `pos` is a **byte offset**, not a character index. For ASCII text these are the same, but for UTF-8 text they differ. For example, the character
    /// `é` is 2 bytes, so inserting after it requires `pos = 2`, not `pos = 1`.
    ///
    /// Passing a `pos` that splits a multi-byte UTF-8 character will cause a panic when the document is read back, as it produces invalid UTF-8 slices.
    /// Always ensure `pos` falls on a character boundary.
    ///
    /// To find a safe byte offset from a character index, use:
    /// ```
    /// let pos = text.char_indices().nth(char_index).map(|(i, _)| i).unwrap_or(text.len());
    /// ```
    ///
    /// # Document Structure
    /// The document is represented as a list of pieces. Each piece points to a slice of either the original buffer or the add buffer. To insert, we find which piece contains `pos`, split it in two, and insert a new piece in between pointing to the newly added text.
    pub fn insert(&mut self, pos: usize, text: &str) -> Result<(), String> {
        // Record where in the add buffer this new text will start, then append it.
        // We do this first so the add buffer is ready before we touch the pieces.
        let add_start = self.add.len();

        if text.is_empty() {
            return Ok(());
        } else {
            self.add.push_str(text);
        }

        let doc_len: usize = self.pieces.iter().map(|p| p.len).sum();
        if pos > doc_len {
            return Err(format!(
                "insert position {pos} is out of bounds (document length is {doc_len})"
            ));
        }

        // `offset` tracks the cumulative document position at the start of each piece as we walk through the list. It starts at 0 (the beginning of the document) and advances by each piece's length each iteration.
        let mut offset = 0usize;

        let mut inserted = false;

        for i in 0..self.pieces.len() {
            let piece = self.pieces[i];

            // Check if `pos` falls within this piece's document range.
            // `offset` is where this piece starts, `offset + piece.len` is where it ends. If `pos` is outside this range, skip to the next piece.
            // Add boundary check cases
            if pos >= offset && pos < (offset + piece.len) {
                // `split` is the local offset — how far into this specific piece the insertion point falls. Subtracting `offset` (the piece's document start position) from `pos` converts from a global document position to a position relative to this piece.
                // Example: if this piece starts at document position 5 and we want to insert at document position 7, split = 7 - 5 = 2, meaning we cut 2 characters into this piece.
                let split = pos - offset;

                // Everything in the current piece before the insertion point.
                // Starts at the same place as the original piece, but is shortened to `split` characters.
                let left_piece = Piece {
                    buffer: piece.buffer,
                    start: piece.start,
                    len: split,
                };

                // The newly inserted text, pointing to what we just appended to the add buffer.
                let new_piece = Piece {
                    buffer: BufferKind::Add,
                    start: add_start,
                    len: text.len(),
                };

                // Everything in the current piece after the insertion point.
                // Its start is shifted forward by `split` to skip past the left portion, and its length is reduced accordingly.
                let right_piece = Piece {
                    buffer: piece.buffer,
                    start: piece.start + split,
                    len: piece.len - split,
                };

                // Replace the original piece with the three new pieces in order: [left_piece] [new_piece] [right_piece]
                // We remove the original first, then insert in reverse order at the same index so they end up in the correct sequence.

                self.pieces.remove(i);

                // Prevent addition of empty string pieces. If split == 0, len is 0, skip
                // Insert in reverse order at index i so the final sequence is [left_piece?, new_piece, right_piece?]
                if split < piece.len {
                    self.pieces.insert(i, right_piece);
                }
                self.pieces.insert(i, new_piece);
                if split > 0 {
                    self.pieces.insert(i, left_piece);
                }
                inserted = true;
                break;
            }

            // `pos` was not in this piece, so advance `offset` by this piece's length to move the window forward to the next piece.
            offset += piece.len;
        }

        // If no piece matched, `pos` is at or beyond the end of the document.
        // Simply push a new piece pointing to the added text — this handles the append case.
        if !inserted {
            self.pieces.push(Piece {
                buffer: BufferKind::Add,
                start: add_start,
                len: text.len(),
            })
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reconstructs_original_text() {
        let pt = PieceTable::new("hello".to_string());
        assert_eq!(pt.to_string(), "hello");
    }

    #[test]
    fn insert_at_start() {
        let mut pt = PieceTable::new("ust".to_string());
        pt.insert(0, "R").unwrap();
        assert_eq!(pt.to_string(), "Rust");
    }
    #[test]
    fn insert_at_middle() {
        let mut pt = PieceTable::new("Hi".to_string());
        pt.insert(1, "o").unwrap();
        assert_eq!(pt.to_string(), "Hoi");
    }
    #[test]
    fn insert_at_end() {
        let mut pt = PieceTable::new("Rust".to_string());
        pt.insert(4, "acean").unwrap();
        assert_eq!(pt.to_string(), "Rustacean");
    }

    #[test]
    fn delete_from_start() {
        let mut pt = PieceTable::new("Rust".to_string());
        pt.delete(0, 4).unwrap();
        assert_eq!(pt.to_string(), "");
    }

    #[test]
    fn delete_from_middle() {
        let mut pt = PieceTable::new("Rust".to_string());
        pt.delete(1, 1).unwrap();
        assert_eq!(pt.to_string(), "Rst");
    }

    #[test]
    fn delete_from_end() {
        let mut pt = PieceTable::new("Rust".to_string());
        pt.delete(2, 1).unwrap();
        assert_eq!(pt.to_string(), "Rut");
    }
}
