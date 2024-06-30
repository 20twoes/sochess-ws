use crate::game::{Game, Move};
use crate::user::User;

pub fn is_white_move(new_move: Move) -> bool {
    new_move.san.chars().nth(0).unwrap() == 'W'
}

pub fn is_legal_move(new_move: Move, current_fen: String) -> bool {
    true
}

pub fn is_own_piece(user: &User, game: &mut Game, new_move: Move) -> bool {
    let piece_color = new_move.san.chars().nth(0).unwrap();
    let last_move = game.last_move();
    if user.name == game.player1.clone().unwrap() {
        return last_move.p1_owned.contains(piece_color)
            || last_move.p1_controlled.contains(piece_color);
    } else {
        return last_move.p2_owned.contains(piece_color)
            || last_move.p2_controlled.contains(piece_color);
    }
}
