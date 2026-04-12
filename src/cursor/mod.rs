use crate::piece_table::PieceTable;

/// A cursor for navigating and editing text in a piece table.
///
/// The cursor maintains a position index and provides methods for
/// movement and character-level editing operations.
///
/// # Examples
///
/// ```
/// use nightingale::cursor::Cursor;
/// use nightingale::piece_table::PieceTable;
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
    /// use nightingale::cursor::Cursor;
    /// use nightingale::piece_table::PieceTable;
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
    /// use nightingale::cursor::Cursor;
    /// use nightingale::piece_table::PieceTable;
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
    /// cursor_at_start.move_left(&table);
    /// assert_eq!(cursor_at_start.get_index(), 0);
    /// ```
    pub fn move_left(&mut self, table: &PieceTable) {
        if self.get_index() == 0 {
            return;
        }

        // Reconstruct the full document string to ensure we can check character boundaries correctly
        let doc = table.to_string();

        // Walk backwards one byte at a time from current position until we land on a valid character boundary
        let mut new_index = self.index - 1;
        while !doc.is_char_boundary(new_index) {
            new_index -= 1;
        }

        self.index = new_index;
    }

    /// Moves the cursor one position to the right.
    ///
    /// Does nothing if the cursor is already at the end of the text.
    ///
    /// # Examples
    ///
    /// ```
    /// use nightingale::cursor::Cursor;
    /// use nightingale::piece_table::PieceTable;
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
        // Reconstruct the full document string to ensure we can check character boundaries correctly
        let doc = table.to_string();

        // If already at or past the end, do nothing.
        if self.index >= doc.len() {
            return;
        }

        // Move forward one byte and advance until we hit a valid character boundary.
        let mut new_index = self.index + 1;
        while new_index <= doc.len() && !doc.is_char_boundary(new_index) {
            new_index += 1;
        }

        if new_index > doc.len() {
            new_index = doc.len();
        }

        self.index = new_index;
    }

    /// Inserts a character at the cursor position and advances the cursor.
    ///
    /// After insertion, the cursor is positioned after the newly inserted character.
    ///
    /// # Examples
    ///
    /// ```
    /// use nightingale::cursor::Cursor;
    /// use nightingale::piece_table::PieceTable;
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
    /// use nightingale::cursor::Cursor;
    /// use nightingale::piece_table::PieceTable;
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
        let mut buf = [0; 4]; // a tiny fixed array on the STACK, not heap, to hold the UTF-8 bytes of the character

        let encoded = c.encode_utf8(&mut buf); // write the char's bytes into it

        table.insert(self.get_index(), encoded)?; // encoded is just a &str pointing to buf, so this is efficient and doesn't require heap allocation for the character

        // Ensure byte length is synchronized, regardless of format: ASCII or UTF-8, reference: https://github.com/aerilabs/Nightingale/pull/7#discussion_r3068757907
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
    /// use nightingale::cursor::Cursor;
    /// use nightingale::piece_table::PieceTable;
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
    /// use nightingale::cursor::Cursor;
    /// use nightingale::piece_table::PieceTable;
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
        if self.get_index() == 0 {
            return Ok(false);
        }

        let text = table.to_string();
        let cursor_index = self.get_index();

        if cursor_index > text.len() {
            return Err(format!(
                "Cursor index {cursor_index} is out of bounds for text length {}",
                text.len()
            ));
        }

        if !text.is_char_boundary(cursor_index) {
            return Err(format!(
                "Cursor index {cursor_index} is not on a valid UTF-8 character boundary in the text"
            ));
        }

        let previous_index = text[..cursor_index] // slice the document UP TO the cursor
            .char_indices() // iterate over (byte_index, char) pairs
            .last() // get the last character before cursor
            .map(|(index, _)| index) // extract just the byte index
            .ok_or_else(|| "No previous character found".to_string())?;

        table.delete(previous_index, cursor_index - previous_index)?;
        self.index = previous_index;
        Ok(true)
    }
}

#[cfg(test)]
mod tests;
