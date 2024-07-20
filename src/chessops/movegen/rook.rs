use crate::chessops::{Bitboard, LookupTables, Square};

pub fn compute_rook_moves(
    start_location: &Bitboard,
    own_side: &Bitboard,
    lookup_tables: &LookupTables,
) -> Bitboard {
    // Get file and rank from start_location
    let square_index = start_location
        .least_significant_bit()
        .expect("Invalid start location");
    let square = Square::from_index(square_index);
    let file = square.file();
    let rank = square.rank();
    let mut legal_moves = lookup_tables.mask_file[file as usize].clone();
    legal_moves.or(&lookup_tables.mask_rank[rank as usize].clone());

    let mut not_own_side = own_side.clone();
    not_own_side.not();
    legal_moves.and(&not_own_side);

    legal_moves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_rook_moves_works() {
        #[rustfmt::skip]
        let start_loc = Bitboard::from_bytes(&[
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
            0b01000000, 0b00000000,
            0b11111111, 0b11111111,
            0b01000000, 0b00000000,
            0b01000000, 0b00000000,
            0b01000000, 0b00000000,
            0b01000000, 0b00000000,
            0b01000000, 0b00000000,
            0b01000000, 0b00000000,
            0b01000000, 0b00000000,
            0b01000000, 0b00000000,
            0b01000000, 0b00000000,
            0b01000000, 0b00000000,
            0b01000000, 0b00000000,
            0b01000000, 0b00000000,
            0b01000000, 0b00000000,
            0b01000000, 0b00000000,
        ]);

        let result = compute_rook_moves(&start_loc, &Bitboard::new(), &LookupTables::new());
        assert_eq!(result, expected);
    }
}
