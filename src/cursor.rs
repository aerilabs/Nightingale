use crate::piece_table::PieceTable;

pub struct Cursor {
    pub index: usize,
}

impl Cursor {
    pub fn new() -> Self {
        Self { index: 0 }
    }

    pub fn move_left(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
    }

    pub fn move_right(&mut self, max: usize) {
        if self.index < max {
            self.index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_movement() {
        let mut cursor = Cursor::new();

        // cursor starts at 0
        assert_eq!(cursor.index, 0);

        // move right within bounds
        cursor.move_right(5);
        assert_eq!(cursor.index, 1); // only moves one per call
        cursor.move_right(5);
        assert_eq!(cursor.index, 2);

        // move left
        cursor.move_left();
        assert_eq!(cursor.index, 1);

        // cannot move left past 0
        cursor.move_left();
        cursor.move_left();
        assert_eq!(cursor.index, 0);

        // cannot move right past max
        cursor.move_right(2);
        cursor.move_right(2);
        cursor.move_right(2);
        assert_eq!(cursor.index, 2);
    }
}
