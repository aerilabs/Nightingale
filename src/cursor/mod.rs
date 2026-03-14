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

    pub fn insert_char(&mut self, table: &mut PieceTable, c: char) {
        table.insert(self.index, &c.to_string());
        self.index += 1;
    }

    pub fn delete_char(&mut self, table: &mut PieceTable) {
        if self.index > 0 {
            // Always delete one character
            table.delete(self.index - 1, 1);
            self.move_left();
        }
    }
}

#[cfg(test)]
mod tests;
