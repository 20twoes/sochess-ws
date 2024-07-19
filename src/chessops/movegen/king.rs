use crate::chessops::{Bitboard, File, LookupTables, BOARD_WIDTH};

/**
* Explanation: https://pages.cs.wisc.edu/~psilord/blog/data/chess-pages/nonsliding.html
* Legal spots the King can move to:
* 1 2 3
* 8 K 4
* 7 6 5
*/
pub fn compute_king_moves(
    king_location: &Bitboard,
    own_side: &Bitboard,
    lookup_tables: &LookupTables,
) -> Bitboard {
    let mut king_clip_file_a = king_location.clone();
    king_clip_file_a.and(&lookup_tables.clear_file[File::A as usize]);

    let mut king_clip_file_p = king_location.clone();
    king_clip_file_p.and(&lookup_tables.clear_file[File::P as usize]);

    let mut spot_1 = king_clip_file_a.clone();
    spot_1.shift_right(BOARD_WIDTH - 1);
    let mut spot_2 = king_location.clone();
    spot_2.shift_right(BOARD_WIDTH);
    let mut spot_3 = king_clip_file_p.clone();
    spot_3.shift_right(BOARD_WIDTH + 1);
    let mut spot_4 = king_clip_file_p.clone();
    spot_4.shift_right(1);
    let mut spot_5 = king_clip_file_p.clone();
    spot_5.shift_left(BOARD_WIDTH - 1);
    let mut spot_6 = king_location.clone();
    spot_6.shift_left(BOARD_WIDTH);
    let mut spot_7 = king_clip_file_a.clone();
    spot_7.shift_left(BOARD_WIDTH + 1);
    let mut spot_8 = king_clip_file_a.clone();
    spot_8.shift_left(1);

    let mut king_moves = spot_1.clone();
    king_moves.or(&spot_2);
    king_moves.or(&spot_3);
    king_moves.or(&spot_4);
    king_moves.or(&spot_5);
    king_moves.or(&spot_6);
    king_moves.or(&spot_7);
    king_moves.or(&spot_8);

    let mut not_own_side = own_side.clone();
    not_own_side.not();
    king_moves.and(&not_own_side);

    king_moves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_king_moves_works() {
        #[rustfmt::skip]
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

        #[rustfmt::skip]
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

        let result = compute_king_moves(&king_loc, &Bitboard::new(), &LookupTables::new());
        assert_eq!(result, expected);

        #[rustfmt::skip]
        let king_loc = Bitboard::from_bytes(&[
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b10000000, 0b00000000,
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

        #[rustfmt::skip]
        let own_side = Bitboard::from_bytes(&[
            0b00000000, 0b00000000,
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
        ]);

        #[rustfmt::skip]
        let expected = Bitboard::from_bytes(&[
            0b00000000, 0b00000000,
            0b11000000, 0b00000000,
            0b00000000, 0b00000000,
            0b11000000, 0b00000000,
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

        let result = compute_king_moves(&king_loc, &own_side, &LookupTables::new());
        assert_eq!(result, expected);
    }
}
