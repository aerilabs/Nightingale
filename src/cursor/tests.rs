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
    cursor.move_left();
    assert_eq!(cursor.get_index(), 1);

    // cannot move left past 0
    cursor.move_left();
    cursor.move_left();
    assert_eq!(cursor.get_index(), 0);

    // cannot move right past max
    cursor.move_right(&table);
    cursor.move_right(&table);
    cursor.move_right(&table);
    assert_eq!(cursor.get_index(), 3);
}

#[test]
fn test_insert_char() {
    let mut cursor = Cursor::new();
    let mut table = PieceTable::new("".to_string());

    // Assert that the cursor starts at 0
    assert_eq!(cursor.get_index(), 0);

    // Insert 'H' at index 0
    cursor.insert_char(&mut table, 'H');
    cursor.insert_char(&mut table, 'i');

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

    // Insert 'H' at index 0
    cursor.delete_char(&mut table);
    cursor.delete_char(&mut table);

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

    cursor.insert_char(&mut table, 'p');
    cursor.insert_char(&mut table, 'h');

    assert_eq!(table.to_string(), "My name is Joseph");
    assert_eq!(cursor.get_index(), 17);

    cursor.index = 10;

    cursor.delete_char(&mut table);
    cursor.delete_char(&mut table);

    assert_eq!(table.to_string(), "My name  Joseph");
    assert_eq!(cursor.get_index(), 8);
}
