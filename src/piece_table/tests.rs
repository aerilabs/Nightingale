use super::*;
#[test]
fn reconstructs_original_text() {
    let pt = PieceTable::new("hello".to_string());
    assert_eq!(pt.to_string(), "hello");
}

#[test]
fn insert_at_start() {
    let mut pt = PieceTable::new("ust".to_string());
    pt.insert(0, "R").unwrap();
    assert_eq!(pt.to_string(), "Rust");
}

#[test]
fn insert_at_middle() {
    let mut pt = PieceTable::new("Hi".to_string());
    pt.insert(1, "o").unwrap();
    assert_eq!(pt.to_string(), "Hoi");
}

#[test]
fn insert_at_end() {
    let mut pt = PieceTable::new("Rust".to_string());
    pt.insert(4, "acean").unwrap();
    assert_eq!(pt.to_string(), "Rustacean");
}

#[test]
fn delete_from_start() {
    let mut pt = PieceTable::new("Rust".to_string());
    pt.delete(0, 4).unwrap();
    assert_eq!(pt.to_string(), "");
}

#[test]
fn delete_from_middle() {
    let mut pt = PieceTable::new("Rust".to_string());
    pt.delete(1, 1).unwrap();
    assert_eq!(pt.to_string(), "Rst");
}

#[test]
fn delete_from_end() {
    let mut pt = PieceTable::new("Rust".to_string());
    pt.delete(2, 1).unwrap();
    assert_eq!(pt.to_string(), "Rut");
}
