use crate::game::Move;

pub fn is_white_move(new_move: Move) -> bool {
    new_move.san.chars().nth(0).unwrap() == 'W'
}

pub fn is_legal_move(new_move: Move, current_fen: String) -> bool {
    true
}
