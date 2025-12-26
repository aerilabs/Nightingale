#[derive(Clone, Copy)]
enum BufferKind {
    Original,
    Add,
}

#[derive(Clone)]
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
    pub fn to_string(&self) -> String {
        let mut result = String::new();

        for piece in &self.pieces {
            let source = match piece.buffer {
                BufferKind::Original => &self.original,
                BufferKind::Add => &self.add,
            };
            result.push_str(&source[piece.start..piece.start + piece.len]);
        }

        result
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
}
