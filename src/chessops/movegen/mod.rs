mod bishop;
mod king;
mod knight;
mod rook;

pub use bishop::compute_bishop_moves;
pub use king::compute_king_moves;
pub use knight::compute_knight_moves;
pub use rook::compute_rook_moves;

pub const MAX_RANGE: usize = 7;
