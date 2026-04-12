use crate::piece_table::PieceTable;

/// A cursor for navigating and editing text in a piece table.
///
/// The cursor maintains a position index and provides methods for
/// movement and character-level editing operations.
///
/// # Examples
///
/// ```
/// use nightingale::{PieceTable, Cursor};
///
/// let mut table = PieceTable::new("Hello".to_string());
/// let mut cursor = Cursor::new();
///
/// // Move to end and insert text
/// cursor.move_right(&table);
/// cursor.move_right(&table);
/// cursor.insert_char(&mut table, '!');
///
/// assert_eq!(table.to_string(), "He!llo");
/// ```
pub struct Cursor {
    index: usize,
}

impl Default for Cursor {
    /// Creates a cursor at position 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use nightingale::cursor::Cursor;
    ///
    /// let cursor = Cursor::default();
    /// assert_eq!(cursor.get_index(), 0);
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

impl Cursor {
    /// Creates a new cursor at position 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use nightingale::cursor::Cursor;
    ///
    /// let cursor = Cursor::new();
    /// assert_eq!(cursor.get_index(), 0);
    /// ```
    pub fn new() -> Self {
        Self { index: 0 }
    }

    /// Returns the current cursor position.
    ///
    /// # Examples
    ///
    /// ```
    /// use nightingale::{PieceTable, Cursor};
    ///
    /// let mut cursor = Cursor::new();
    /// let table = PieceTable::new("Hello".to_string());
    ///
    /// assert_eq!(cursor.get_index(), 0);
    /// cursor.move_right(&table);
    /// assert_eq!(cursor.get_index(), 1);
    /// ```
    pub fn get_index(&self) -> usize {
        self.index
    }

    /// Moves the cursor one position to the left.
    ///
    /// Does nothing if the cursor is already at position 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use nightingale::{PieceTable, Cursor};
    ///
    /// let mut cursor = Cursor::new();
    /// let table = PieceTable::new("Hello".to_string());
    ///
    /// cursor.move_right(&table);
    /// cursor.move_right(&table);
    /// assert_eq!(cursor.get_index(), 2);
    ///
    /// cursor.move_left();
    /// assert_eq!(cursor.get_index(), 1);
    ///
    /// // Moving left at position 0 does nothing
    /// let mut cursor_at_start = Cursor::new();
    /// cursor_at_start.move_left();
    /// assert_eq!(cursor_at_start.get_index(), 0);
    /// ```
    pub fn move_left(&mut self) {
        if self.get_index() > 0 {
            self.index -= 1;
        }
    }

    /// Moves the cursor one position to the right.
    ///
    /// Does nothing if the cursor is already at the end of the text.
    ///
    /// # Examples
    ///
    /// ```
    /// use nightingale::{PieceTable, Cursor};
    ///
    /// let mut cursor = Cursor::new();
    /// let table = PieceTable::new("Hi".to_string());
    ///
    /// cursor.move_right(&table);
    /// assert_eq!(cursor.get_index(), 1);
    ///
    /// cursor.move_right(&table);
    /// assert_eq!(cursor.get_index(), 2);
    ///
    /// // Moving right at the end does nothing
    /// cursor.move_right(&table);
    /// assert_eq!(cursor.get_index(), 2);
    /// ```
    pub fn move_right(&mut self, table: &PieceTable) {
        if self.get_index() < table.len() {
            self.index += 1;
        }
    }

    /// Inserts a character at the cursor position and advances the cursor.
    ///
    /// After insertion, the cursor is positioned after the newly inserted character.
    ///
    /// # Examples
    ///
    /// ```
    /// use nightingale::{PieceTable, Cursor};
    ///
    /// let mut table = PieceTable::new("Hllo".to_string());
    /// let mut cursor = Cursor::new();
    ///
    /// cursor.move_right(&table);
    /// cursor.insert_char(&mut table, 'e');
    ///
    /// assert_eq!(table.to_string(), "Hello");
    /// assert_eq!(cursor.get_index(), 2);
    /// ```
    ///
    /// Multiple insertions:
    ///
    /// ```
    /// use nightingale::{PieceTable, Cursor};
    ///
    /// let mut table = PieceTable::new(String::new());
    /// let mut cursor = Cursor::new();
    ///
    /// cursor.insert_char(&mut table, 'H');
    /// cursor.insert_char(&mut table, 'i');
    /// cursor.insert_char(&mut table, '!');
    ///
    /// assert_eq!(table.to_string(), "Hi!");
    /// assert_eq!(cursor.get_index(), 3);
    /// ```
    pub fn insert_char(&mut self, table: &mut PieceTable, c: char) -> Result<(), String> {
        table.insert(self.get_index(), &c.to_string())?;
        // Ensure byte length is synchronized, regarled of format: ASCII or UTF-8, reference: https://github.com/aerilabs/Nightingale/pull/7#discussion_r3068757907
        self.index += c.len_utf8();
        Ok(())
    }

    /// Deletes the character before the cursor position (backspace behavior).
    ///
    /// After deletion, the cursor moves one position to the left.
    /// Does nothing if the cursor is at position 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use nightingale::{PieceTable, Cursor};
    ///
    /// let mut table = PieceTable::new("Hello".to_string());
    /// let mut cursor = Cursor::new();
    ///
    /// // Move to position 5 (after 'Hello')
    /// for _ in 0..5 {
    ///     cursor.move_right(&table);
    /// }
    ///
    /// cursor.delete_char(&mut table);
    /// assert_eq!(table.to_string(), "Hell");
    /// assert_eq!(cursor.get_index(), 4);
    /// ```
    ///
    /// Deleting at position 0:
    ///
    /// ```
    /// use nightingale::{PieceTable, Cursor};
    ///
    /// let mut table = PieceTable::new("Hi".to_string());
    /// let mut cursor = Cursor::new();
    ///
    /// // Cursor is at position 0, delete does nothing
    /// cursor.delete_char(&mut table);
    /// assert_eq!(table.to_string(), "Hi");
    /// assert_eq!(cursor.get_index(), 0);
    /// ```
    pub fn delete_char(&mut self, table: &mut PieceTable) -> Result<bool, String> {
        if self.get_index() > 0 {
            table.delete(self.get_index() - 1, 1)?;
            self.move_left();
            return Ok(true);
        }
        Ok(false)
    }
}

#[cfg(test)]
mod tests;
