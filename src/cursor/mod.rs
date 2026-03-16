use crate::piece_table::PieceTable;

pub struct Cursor {
    /// Set to private to ensure that nothing outside the module can directly read or write the field
    index: usize,
}

impl Default for Cursor {
    fn default() -> Self {
        Self::new()
    }
}

impl Cursor {
    /// Setter method that sets the index to 0 when the Cursor struct is instantiated
    pub fn new() -> Self {
        Self { index: 0 }
    }

    /// Getter method that returns the current index
    pub fn get_index(&self) -> usize {
        self.index
    }

    /// Moves the cursor to the left by one character
    /// Inbuilt guard is implemented to prevent the index from going out of bounds (to -1, which would be before the first character)
    pub fn move_left(&mut self) {
        if self.get_index() > 0 {
            self.index -= 1;
        }
    }

    /// Moves the cursor to the right by one character, so long as the index is less than the sum of the indexes of all the pieces in the table
    pub fn move_right(&mut self, table: &PieceTable) {
        if self.get_index() < table.len() {
            self.index += 1;
        }
    }

    pub fn insert_char(&mut self, table: &mut PieceTable, c: char) {
        table.insert(self.get_index(), &c.to_string());
        self.index += 1;
    }

    pub fn delete_char(&mut self, table: &mut PieceTable) {
        if self.get_index() > 0 {
            // Always delete one character
            table.delete(self.get_index() - 1, 1);
            self.move_left();
        }
    }
}

#[cfg(test)]
mod tests;
