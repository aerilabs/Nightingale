pub mod cursor;
pub mod piece_table;

// Re-export commonly used types at the crate root so doctests and external
// callers can import `nightingale::{PieceTable, Cursor}`.
pub use crate::cursor::Cursor;
pub use crate::piece_table::PieceTable;
