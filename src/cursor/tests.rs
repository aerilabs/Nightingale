use super::*;

#[test]
fn test_cursor_movement() {
    let mut cursor = Cursor::new();
    let table = PieceTable::new("Hello".to_string());

    // cursor starts at 0
    assert_eq!(cursor.get_index(), 0);

    // move right within bounds
    cursor.move_right(&table);
    assert_eq!(cursor.get_index(), 1); // only moves one per call
    cursor.move_right(&table);
    assert_eq!(cursor.get_index(), 2);

    // move left
    cursor.move_left(&table);
    assert_eq!(cursor.get_index(), 1);

    // cannot move left past 0
    cursor.move_left(&table);
    cursor.move_left(&table);
    assert_eq!(cursor.get_index(), 0);

    // cannot move right past max
    cursor.move_right(&table);
    cursor.move_right(&table);
    cursor.move_right(&table);
    cursor.move_right(&table);
    cursor.move_right(&table);
    cursor.move_right(&table);
    assert_eq!(cursor.get_index(), 5);
}

#[test]
fn test_insert_char() {
    let mut cursor = Cursor::new();
    let mut table = PieceTable::new("".to_string());

    // Assert that the cursor starts at 0
    assert_eq!(cursor.get_index(), 0);

    // Insert 'H' at index 0
    cursor.insert_char(&mut table, 'H').unwrap();
    cursor.insert_char(&mut table, 'i').unwrap();

    assert_eq!(table.to_string(), "Hi");
    assert_eq!(cursor.get_index(), 2);
}

#[test]
fn test_delete_char() {
    let mut cursor = Cursor::new();
    let mut table = PieceTable::new("Hello".to_string());

    cursor.index = 5;

    // Assert that the cursor is at 5
    assert_eq!(cursor.get_index(), 5);

    // Delete two characters starting from the end of the text
    cursor.delete_char(&mut table).unwrap();
    cursor.delete_char(&mut table).unwrap();

    assert_eq!(table.to_string(), "Hel");
    assert_eq!(cursor.get_index(), 3);
}

#[test]
fn test_insert_then_delete() {
    let mut cursor = Cursor::new();
    let mut table = PieceTable::new("My name is Jose".to_string());

    // Assert that the cursor starts at 0
    assert_eq!(cursor.index, 0);

    cursor.index = 15;

    cursor.insert_char(&mut table, 'p').unwrap();
    cursor.insert_char(&mut table, 'h').unwrap();

    assert_eq!(table.to_string(), "My name is Joseph");
    assert_eq!(cursor.get_index(), 17);

    cursor.index = 10;

    cursor.delete_char(&mut table).unwrap();
    cursor.delete_char(&mut table).unwrap();

    assert_eq!(table.to_string(), "My name  Joseph");
    assert_eq!(cursor.get_index(), 8);
}

#[test]
fn test_cursor_movement_with_multibyte_chars() {
    let mut cursor = Cursor::new();
    // "Héllo" = H(1) + é(2) + l(1) + l(1) + o(1) = 6 bytes total
    let table = PieceTable::new("Héllo".to_string());

    // Start at position 0 (before 'H')
    assert_eq!(cursor.get_index(), 0);

    // Move right to position 1 (after 'H', before 'é')
    cursor.move_right(&table);
    assert_eq!(cursor.get_index(), 1);

    // Move right from position 1 should skip to position 3 (after 'é')
    // because 'é' is 2 bytes and we must land on a valid boundary
    cursor.move_right(&table);
    assert_eq!(cursor.get_index(), 3);

    // Move right to 4 (after 'l')
    cursor.move_right(&table);
    assert_eq!(cursor.get_index(), 4);

    // Move right to 5 (after 'l')
    cursor.move_right(&table);
    assert_eq!(cursor.get_index(), 5);

    // Move right to 6 (after 'o', at end)
    cursor.move_right(&table);
    assert_eq!(cursor.get_index(), 6);

    // Move right at end does nothing
    cursor.move_right(&table);
    assert_eq!(cursor.get_index(), 6);

    // Move left from 6 to 5
    cursor.move_left(&table);
    assert_eq!(cursor.get_index(), 5);

    // Move left from 5 to 4
    cursor.move_left(&table);
    assert_eq!(cursor.get_index(), 4);

    // Move left from 4 to 3 (after 'é')
    cursor.move_left(&table);
    assert_eq!(cursor.get_index(), 3);

    // Move left from 3 should skip back to 1 (after 'H')
    // because position 2 is in the middle of 'é'
    cursor.move_left(&table);
    assert_eq!(cursor.get_index(), 1);

    // Move left to 0
    cursor.move_left(&table);
    assert_eq!(cursor.get_index(), 0);

    // Move left at start does nothing
    cursor.move_left(&table);
    assert_eq!(cursor.get_index(), 0);
}

#[test]
fn test_delete_char_with_multibyte_char() {
    let mut cursor = Cursor::new();
    // "café" = c(1) + a(1) + f(1) + é(2) = 5 bytes total
    let mut table = PieceTable::new("café".to_string());

    // Position cursor at the end (after 'é', at byte position 5)
    cursor.index = 5;

    assert_eq!(cursor.get_index(), 5);
    assert_eq!(table.to_string(), "café");

    // Delete 'é' (should delete both bytes, positions 3-4)
    cursor.delete_char(&mut table).unwrap();

    assert_eq!(table.to_string(), "caf");
    // Cursor should move to position 3 (after 'f')
    assert_eq!(cursor.get_index(), 3);

    // Delete 'f'
    cursor.delete_char(&mut table).unwrap();
    assert_eq!(table.to_string(), "ca");
    assert_eq!(cursor.get_index(), 2);

    // Delete 'a'
    cursor.delete_char(&mut table).unwrap();
    assert_eq!(table.to_string(), "c");
    assert_eq!(cursor.get_index(), 1);

    // Delete 'c'
    cursor.delete_char(&mut table).unwrap();
    assert_eq!(table.to_string(), "");
    assert_eq!(cursor.get_index(), 0);
}

#[test]
fn test_insert_multibyte_char_then_delete() {
    let mut cursor = Cursor::new();
    let mut table = PieceTable::new("Hello".to_string());

    // Position at index 5 (after 'Hello')
    cursor.index = 5;

    // Insert multi-byte character 'é'
    cursor.insert_char(&mut table, 'é').unwrap();

    assert_eq!(table.to_string(), "Helloé");
    // Cursor should advance by 2 (byte length of 'é')
    assert_eq!(cursor.get_index(), 7);

    // Delete the 'é'
    cursor.delete_char(&mut table).unwrap();

    assert_eq!(table.to_string(), "Hello");
    assert_eq!(cursor.get_index(), 5);
}

#[test]
fn test_mixed_multibyte_chars() {
    let mut cursor = Cursor::new();
    // "H🦀llo" = H(1) + 🦀(4) + l(1) + l(1) + o(1) = 8 bytes
    let mut table = PieceTable::new("H🦀llo".to_string());

    // Start at 0
    assert_eq!(cursor.get_index(), 0);

    // Move right to 1 (after 'H')
    cursor.move_right(&table);
    assert_eq!(cursor.get_index(), 1);

    // Move right from 1 should skip to 5 (after 🦀)
    // because 🦀 is 4 bytes (positions 1-4)
    cursor.move_right(&table);
    assert_eq!(cursor.get_index(), 5);

    // Position at 5 (first 'l') and delete the previous character (the emoji)
    cursor.index = 5;
    cursor.delete_char(&mut table).unwrap();

    // Emoji is deleted, leaving "Hllo" (4 bytes: H=1, l=1, l=1, o=1)
    assert_eq!(table.to_string(), "Hllo");
    // Cursor moves to where the emoji started (position 1)
    assert_eq!(cursor.get_index(), 1);

    // Now position at the end of "Hllo" (4 bytes) and delete 'o'
    cursor.index = 4;
    cursor.delete_char(&mut table).unwrap();
    assert_eq!(table.to_string(), "Hll");
    assert_eq!(cursor.get_index(), 3);

    // Delete second 'l'
    cursor.delete_char(&mut table).unwrap();
    assert_eq!(table.to_string(), "Hl");
    assert_eq!(cursor.get_index(), 2);

    // Delete first 'l'
    cursor.delete_char(&mut table).unwrap();
    assert_eq!(table.to_string(), "H");
    assert_eq!(cursor.get_index(), 1);

    // Delete 'H'
    cursor.delete_char(&mut table).unwrap();
    assert_eq!(table.to_string(), "");
    assert_eq!(cursor.get_index(), 0);
}
