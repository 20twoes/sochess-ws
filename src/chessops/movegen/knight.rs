use crate::chessops::{Bitboard, File, LookupTables, BOARD_WIDTH};

/**
* Explanation: https://pages.cs.wisc.edu/~psilord/blog/data/chess-pages/nonsliding.html
* Legal spots the Knight can move to:
* . 2 . 3 .
* 1 . . . 4
* . . N . .
* 8 . . . 5
* . 7 . 6 .
*/
pub fn compute_knight_moves(
    knight_location: &Bitboard,
    own_side: &Bitboard,
    lookup_tables: &LookupTables,
) -> Bitboard {
    let mut spot_1_clip = lookup_tables.clear_file[File::A as usize].clone();
    spot_1_clip.and(&lookup_tables.clear_file[File::B as usize]);

    let spot_2_clip = &lookup_tables.clear_file[File::A as usize];

    let spot_3_clip = &lookup_tables.clear_file[File::P as usize];

    let mut spot_4_clip = lookup_tables.clear_file[File::P as usize].clone();
    spot_4_clip.and(&lookup_tables.clear_file[File::O as usize]);

    let spot_5_clip = spot_4_clip.clone();

    let spot_6_clip = &lookup_tables.clear_file[File::P as usize];

    let spot_7_clip = &lookup_tables.clear_file[File::A as usize];

    let spot_8_clip = spot_1_clip.clone();

    let mut spot_1 = knight_location.clone();
    spot_1.and(&spot_1_clip);
    spot_1.shift_right(BOARD_WIDTH - 2);

    let mut spot_2 = knight_location.clone();
    spot_2.and(&spot_2_clip);
    spot_2.shift_right((BOARD_WIDTH * 2) - 1);

    let mut spot_3 = knight_location.clone();
    spot_3.and(&spot_3_clip);
    spot_3.shift_right((BOARD_WIDTH * 2) + 1);

    let mut spot_4 = knight_location.clone();
    spot_4.and(&spot_4_clip);
    spot_4.shift_right(BOARD_WIDTH + 2);

    let mut spot_5 = knight_location.clone();
    spot_5.and(&spot_5_clip);
    spot_5.shift_left(BOARD_WIDTH - 2);

    let mut spot_6 = knight_location.clone();
    spot_6.and(&spot_6_clip);
    spot_6.shift_left((BOARD_WIDTH * 2) - 1);

    let mut spot_7 = knight_location.clone();
    spot_7.and(&spot_7_clip);
    spot_7.shift_left((BOARD_WIDTH * 2) + 1);

    let mut spot_8 = knight_location.clone();
    spot_8.and(&spot_8_clip);
    spot_8.shift_left(BOARD_WIDTH + 2);

    let mut legal_moves = spot_1.clone();
    legal_moves.or(&spot_2);
    legal_moves.or(&spot_3);
    legal_moves.or(&spot_4);
    legal_moves.or(&spot_5);
    legal_moves.or(&spot_6);
    legal_moves.or(&spot_7);
    legal_moves.or(&spot_8);

    let mut not_own_side = own_side.clone();
    not_own_side.not();
    legal_moves.and(&not_own_side);

    legal_moves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_knight_moves_works() {
        #[rustfmt::skip]
        let knight_loc = Bitboard::from_bytes(&[
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00100000, 0b00000000,
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
            0b01010000, 0b00000000,
            0b10001000, 0b00000000,
            0b00000000, 0b00000000,
            0b10001000, 0b00000000,
            0b01010000, 0b00000000,
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

        let result = compute_knight_moves(&knight_loc, &Bitboard::new(), &LookupTables::new());
        println!("Result:\n{}", result);
        println!("Expected:\n{}", expected);
        assert_eq!(result, expected);

        #[rustfmt::skip]
        let knight_loc = Bitboard::from_bytes(&[
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
            0b00000000, 0b00000000,
        ]);

        #[rustfmt::skip]
        let expected = Bitboard::from_bytes(&[
            0b00000000, 0b00000000,
            0b00100000, 0b00000000,
            0b00000000, 0b00000000,
            0b00100000, 0b00000000,
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
        ]);

        let result = compute_knight_moves(&knight_loc, &own_side, &LookupTables::new());

        assert_eq!(result, expected);
    }
}
