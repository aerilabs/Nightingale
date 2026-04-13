use std::fmt;

/// Buffer identifier for piece table segments.
#[derive(Clone, Copy)]
enum BufferKind {
    Original,
    Add,
}

/// A contiguous segment of text within a buffer.
#[derive(Clone, Copy)]
struct Piece {
    buffer: BufferKind,
    start: usize,
    len: usize,
}

/// A piece table data structure for efficient text editing.
///
/// The piece table maintains two buffers: one for the original text and one for
/// added text. It tracks segments (pieces) from these buffers, enabling efficient
/// insertions and deletions without modifying the original content.
///
/// # Examples
///
/// ```
/// use nightingale::PieceTable;
///
/// let mut table = PieceTable::new("Hello, world!".to_string());
/// table.insert(7, "Rust ");
/// assert_eq!(table.to_string(), "Hello, Rust world!");
/// ```
pub struct PieceTable {
    original: String,
    add: String,
    pieces: Vec<Piece>,
    len: usize, // cached document byte length
}

impl fmt::Display for PieceTable {
    /// Reconstructs and formats the text represented by the piece table.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for piece in &self.pieces {
            let source = match piece.buffer {
                BufferKind::Original => &self.original,
                BufferKind::Add => &self.add,
            };

            // Rust strings are UTF-8, but slicing them with [start..end] operates on bytes. If a slice boundary lands in the middle of a multi-byte character, Rust panics immediately.

            // 1. checked_add prevents integer overflow on piece.start + piece.len.
            // If it overflows, returns fmt::Error instead of wrapping/panicking.
            let end = piece.start.checked_add(piece.len).ok_or(fmt::Error)?;

            // 2. str::get() is the non-panicking version of slicing.
            // Returns None if the range is out of bounds OR splits a char boundary, converting that into fmt::Error instead of a panic.
            let segment = source.get(piece.start..end).ok_or(fmt::Error)?;
            write!(f, "{}", segment)?;
        }
        Ok(())
    }
}

impl Default for PieceTable {
    /// Creates an empty piece table.
    ///
    /// # Examples
    ///
    /// ```
    /// use nightingale::PieceTable;
    ///
    /// let table = PieceTable::default();
    /// assert!(table.is_empty());
    /// ```
    fn default() -> Self {
        Self::new(String::new())
    }
}

impl PieceTable {
    /// Creates a new piece table with the given initial text.
    ///
    /// # Examples
    ///
    /// ```
    /// use nightingale::PieceTable;
    ///
    /// let table = PieceTable::new("Initial text".to_string());
    /// assert_eq!(table.to_string(), "Initial text");
    /// assert_eq!(table.len(), 12);
    /// ```
    pub fn new(text: String) -> Self {
        let len = text.len();
        Self {
            len,
            original: text,
            add: String::new(),
            pieces: vec![Piece {
                buffer: BufferKind::Original,
                start: 0,
                len,
            }],
        }
    }

    /// Deletes text at the specified byte offset. Supports deletion within only ONE piece.
    ///
    /// # Arguments
    ///
    /// * `pos` - The starting **byte offset** of the deletion (not a character index).
    /// * `len` - The number of **bytes** to delete (not a character count).
    ///
    /// For ASCII text, byte offsets and character indices are the same. For UTF-8
    /// text they differ — for example `é` is 2 bytes, so deleting it requires `len = 2`.
    ///
    /// `delete` validates that both `pos` and `pos + len` fall on UTF-8 character
    /// boundaries. If either boundary would split a multi-byte character, the method
    /// returns an error and leaves the piece table unchanged.
    ///
    /// To find a safe byte length from a character count, use:
    /// ```text
    /// let len = text[byte_start..].char_indices().nth(char_count).map(|(i, _)| i).unwrap_or(text.len() - byte_start);
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use nightingale::PieceTable;
    ///
    /// let mut table = PieceTable::new("Hello, world!".to_string());
    /// table.delete(5, 7);  // Remove ", world" (7 bytes, all ASCII)
    /// assert_eq!(table.to_string(), "Hello!");
    /// ```
    pub fn delete(&mut self, pos: usize, len: usize) -> Result<(), String> {
        let doc_len = self.len;

        // Ensure the starting position is within the document
        if pos > doc_len {
            return Err(format!(
                "delete position {pos} is out of bounds (document length is {doc_len})"
            ));
        }

        // Deleting 0 bytes is a no-op, nothing to do
        if len == 0 {
            return Ok(());
        }

        // Ensure the deletion range doesn't extend beyond the document
        if len > doc_len - pos {
            return Err(format!(
                "delete range [{pos}, {}) is out of bounds (document length is {doc_len})",
                pos + len
            ));
        }

        // `offset` tracks the cumulative document position at the start of each piece as we walk through the list, same as in insert
        let mut offset = 0usize;
        for i in 0..self.pieces.len() {
            let piece = self.pieces[i];

            // Check if `pos` falls within this piece's document range
            if pos >= offset && pos < (offset + piece.len) {
                // `split` is how far into this piece the deletion starts, converting from a global document position to a piece-local one
                let split = pos - offset;

                // Ensure the entire deletion fits within this single piece.
                // We only support deletion within one piece — if the range spans multiple pieces this is an error
                if split + len > piece.len {
                    return Err(format!(
                        "invalid delete range: {pos} {len} exceeds bounds of current piece ({}, {offset})",
                        piece.len
                    ));
                }

                // Get the actual buffer this piece points to so we can validate UTF-8 character boundaries against real byte positions in that buffer
                let source = match piece.buffer {
                    BufferKind::Original => &self.original,
                    BufferKind::Add => &self.add,
                };

                // Convert document-level split into a buffer-level byte index
                // `buf_start` converts the piece-local `split` into an actual byte index in the buffer. `piece.start` is where this piece begins in the buffer, so adding `split` gives the real position.
                // Example: piece.start = 0, split = 1 on "Héllo"
                // buf_start = 0 + 1 = 1  (start of 'é' in the buffer)
                let buf_start = piece.start + split;

                // `buf_end` is where the deletion ends in the buffer
                //
                // Example: buf_start = 1, len = 2
                // buf_end = 1 + 2 = 3  (byte after 'é')
                let buf_end = buf_start + len;

                // Validate that buf_start lands on a character boundary.
                // is_char_boundary returns false if the byte is in the middle of a multi-byte character. For example on "Héllo", byte 2 is the second byte of 'é' — not a valid boundary.
                if !source.is_char_boundary(buf_start) {
                    return Err(format!(
                        "delete position {pos} splits a multi-byte UTF-8 character"
                    ));
                }

                // Same check for the end of the deletion range
                if !source.is_char_boundary(buf_end) {
                    return Err(format!(
                        "delete end position {} splits a multi-byte UTF-8 character",
                        pos + len
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

                // Replace the original piece with left and right.
                // Insert in reverse order at index i so the final sequence is [left_piece, right_piece]
                self.pieces.remove(i);

                // Only insert right if it has content, split + len == piece.len means deletion reaches the end of the piece so right would be empty
                if split + len < piece.len {
                    self.pieces.insert(i, right_piece);
                }

                // Only insert left if it has content, split == 0 means deletion starts at the beginning of the piece so left would be empty
                if split > 0 {
                    self.pieces.insert(i, left_piece);
                }

                break;
            }
            // `pos` was not in this piece, advance offset to the next piece
            offset += piece.len;
        }

        // Update the cached document length
        self.len -= len;
        Ok(())
    }

    /// Inserts text at the specified position.
    ///
    /// # Arguments
    ///
    /// * `pos` - The byte index where text should be inserted. This must be a valid UTF-8 character boundary within the current document.
    /// * `text` - The text to insert
    ///
    /// # Examples
    ///
    /// ```
    /// use nightingale::PieceTable;
    ///
    /// let mut table = PieceTable::new("Hello!".to_string());
    /// table.insert(5, ", world");
    /// assert_eq!(table.to_string(), "Hello, world!");
    /// ```
    ///
    /// Multiple insertions:
    ///
    /// ```
    /// use nightingale::PieceTable;
    ///
    /// let mut table = PieceTable::new("The end".to_string());
    /// table.insert(0, "Beginning. ");
    /// table.insert(11, "Middle. ");
    /// assert_eq!(table.to_string(), "Beginning. Middle. The end");
    /// ```
    pub fn insert(&mut self, pos: usize, text: &str) -> Result<(), String> {
        if text.is_empty() {
            return Err("insert text must be non-empty".to_owned());
        }

        let doc_len: usize = self.len;
        if pos > doc_len {
            return Err(format!(
                "insert position {pos} is out of bounds (document length is {doc_len})"
            ));
        }

        // Support for UTF-8 characters: reconstruct the document and check
        // the boundary only after validating `pos` is within bounds.
        let doc = self.to_string();
        if !doc.is_char_boundary(pos) {
            return Err(format!("pos {pos} is not on a UTF-8 character boundary"));
        }

        // Mutate after checks to ensure piece table remains consistent even if insert fails due to invalid input.
        let add_start = self.add.len();
        self.add.push_str(text);

        let mut offset = 0usize;

        let mut inserted = false;

        for i in 0..self.pieces.len() {
            let piece = self.pieces[i];

            if pos >= offset && pos < (offset + piece.len) {
                let split = pos - offset;

                let left_piece = Piece {
                    buffer: piece.buffer,
                    start: piece.start,
                    len: split,
                };
                let new_piece = Piece {
                    buffer: BufferKind::Add,
                    start: add_start,
                    len: text.len(),
                };
                let right_piece = Piece {
                    buffer: piece.buffer,
                    start: piece.start + split,
                    len: piece.len - split,
                };

                self.pieces.remove(i);

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
            offset += piece.len;
        }
        if !inserted {
            self.pieces.push(Piece {
                buffer: BufferKind::Add,
                start: add_start,
                len: text.len(),
            })
        }
        self.len += text.len(); // increment by bytes inserted
        Ok(())
    }

    /// Returns the total length of the text in bytes.
    ///
    /// This is a true O(1) operation because the length is cached and does
    /// not depend on the number of pieces.
    ///
    /// # Examples
    ///
    /// ```
    /// use nightingale::piece_table::PieceTable;
    ///
    /// let mut table = PieceTable::new("Hello".to_string());
    /// assert_eq!(table.len(), 5);
    ///
    /// table.insert(5, " world");
    /// assert_eq!(table.len(), 11);
    /// ```
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the piece table contains no text.
    ///
    /// # Examples
    ///
    /// ```
    /// use nightingale::PieceTable;
    ///
    /// let empty = PieceTable::new(String::new());
    /// assert!(empty.is_empty());
    ///
    /// let non_empty = PieceTable::new("text".to_string());
    /// assert!(!non_empty.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests;
