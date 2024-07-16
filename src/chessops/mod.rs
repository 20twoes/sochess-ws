mod bitboard;

use bitboard::{BOARD_WIDTH, Bitboard};

/**
 * Explanation: https://pages.cs.wisc.edu/~psilord/blog/data/chess-pages/nonsliding.html
 * Legal spots the King can move to:
 * 1 2 3
 * 8 K 4
 * 7 6 5
 */
pub fn compute_king_moves(king_location: Bitboard) -> Bitboard {
    // TODO: Handle cases when the king is on the edge of the board
    let spot_1 = king_location.shift_right(BOARD_WIDTH - 1);
    let spot_2 = king_location.shift_right(BOARD_WIDTH);
    let spot_3 = king_location.shift_right(BOARD_WIDTH + 1);
    let spot_4 = king_location.shift_right(1);
    let spot_5 = king_location.shift_left(BOARD_WIDTH - 1);
    let spot_6 = king_location.shift_left(BOARD_WIDTH);
    let spot_7 = king_location.shift_left(BOARD_WIDTH + 1);
    let spot_8 = king_location.shift_left(1);

    let mut king_moves = spot_1.clone();
    king_moves.or(&spot_2);
    king_moves.or(&spot_3);
    king_moves.or(&spot_4);
    king_moves.or(&spot_5);
    king_moves.or(&spot_6);
    king_moves.or(&spot_7);
    king_moves.or(&spot_8);

    king_moves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn king_moves_are_legal() {
        let king_loc = Bitboard::from_bytes(&[
            0b00000000, 0b00000000,
            0b01000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
        ]);

        let expected = Bitboard::from_bytes(&[
            0b11100000, 0b00000000,
            0b10100000, 0b00000000,
            0b11100000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
        ]);

        let result = compute_king_moves(king_loc);
        assert_eq!(result, expected);
    }
}
