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
/// use piece_table::PieceTable;
///
/// let mut table = PieceTable::new("Hello, world!".to_string());
/// table.insert(7, "Rust ");
/// assert_eq!(table.to_string(), "Hello, Rust world!");
/// ```
pub struct PieceTable {
    original: String,
    add: String,
    pieces: Vec<Piece>,
    len: usize, //cached document byte length
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
    /// use piece_table::PieceTable;
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
    /// use piece_table::PieceTable;
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
    /// Passing a `pos` or `len` that splits a multi-byte character will produce
    /// invalid UTF-8 and panic when the document is read back. Always ensure both
    /// `pos` and `pos + len` fall on character boundaries.
    ///
    /// To find a safe byte length from a character count, use:
    /// ```
    /// let len = text[byte_start..].char_indices().nth(char_count).map(|(i, _)| i).unwrap_or(text.len() - byte_start);
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use piece_table::PieceTable;
    ///
    /// let mut table = PieceTable::new("Hello, world!".to_string());
    /// table.delete(5, 7);  // Remove ", world" (7 bytes, all ASCII)
    /// assert_eq!(table.to_string(), "Hello!");
    /// ```
    pub fn delete(&mut self, pos: usize, len: usize) -> Result<(), String> {
        let doc_len = self.pieces.iter().map(|p| p.len).sum();

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
        self.len -= len; // decrement by bytes deleted
        Ok(())
    }

    /// Inserts text at the specified position.
    ///
    /// # Arguments
    ///
    /// * `pos` - The position where text should be inserted
    /// * `text` - The text to insert
    ///
    /// # Examples
    ///
    /// ```
    /// use piece_table::PieceTable;
    ///
    /// let mut table = PieceTable::new("Hello!".to_string());
    /// table.insert(5, ", world");
    /// assert_eq!(table.to_string(), "Hello, world!");
    /// ```
    ///
    /// Multiple insertions:
    ///
    /// ```
    /// use piece_table::PieceTable;
    ///
    /// let mut table = PieceTable::new("The end".to_string());
    /// table.insert(0, "Beginning. ");
    /// table.insert(11, "Middle. ");
    /// assert_eq!(table.to_string(), "Beginning. Middle. The end");
    /// ```
    pub fn insert(&mut self, pos: usize, text: &str) -> Result<(), String> {
        let add_start = self.add.len();

        if text.is_empty() {
            return Err("insert text must be non-empty".to_owned());
        }

        // Support for UTF-8 characters
        if !self.original.is_char_boundary(pos) {
            return Err(format!("pos {pos} is not on a UTF-8 character boundary"));
        }

        if pos > add_start {
            return Err(format!(
                "Value {pos} is greater than {add_start}, hence, out of bounds"
            ));
        } else {
            self.add.push_str(text);
        }

        let doc_len: usize = self.pieces.iter().map(|p| p.len).sum();
        if pos > doc_len {
            return Err(format!(
                "insert position {pos} is out of bounds (document length is {doc_len})"
            ));
        }

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
    /// This is an O(1) operation where 1 is the number of pieces.
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
    /// use piece_table::PieceTable;
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
