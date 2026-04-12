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
