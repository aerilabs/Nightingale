use std::fmt;

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

impl fmt::Display for PieceTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for piece in &self.pieces {
            let source = match piece.buffer {
                BufferKind::Original => &self.original,
                BufferKind::Add => &self.add,
            };
            write!(f, "{}", &source[piece.start..(piece.start + piece.len)])?;
        }
        Ok(())
    }
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

    pub fn delete(&mut self, pos: usize, len: usize) {
        let mut offset = 0usize;
        for i in 0..self.pieces.len() {
            let piece = self.pieces[i];

            if pos >= offset && pos <= (offset + piece.len) {
                let split = pos - offset;

                // Guard to prevent overflow
                if split + len > piece.len {
                    break;
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
    }

    pub fn insert(&mut self, pos: usize, text: &str) {
        let add_start = self.add.len();
        self.add.push_str(text);

        let mut offset = 0usize;
        for i in 0..self.pieces.len() {
            let piece = self.pieces[i];
            if pos >= offset && pos <= (offset + piece.len) {
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
                self.pieces.insert(i, right_piece);
                self.pieces.insert(i, new_piece);
                self.pieces.insert(i, left_piece);
                break;
            }
            offset += piece.len;
        }
    }

    /// Walks through every piece, gets its length, and sums up all the piece lengths to get the total length of the text represented by the piece table
    /// An alternative approach would be to use
    /// ```rust
    /// table.to_string().len()
    /// ```
    /// but that would be inefficient as it would require reconstructing the entire string just to get its length
    pub fn len(&self) -> usize {
        if !self.pieces.is_empty() {
            self.pieces.iter().map(|p| p.len).sum()
        } else {
            0
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests;
