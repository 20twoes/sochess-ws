use crate::chessops::{movegen::MAX_RANGE, Bitboard, File, Square, BOARD_SIZE, BOARD_WIDTH};

/**
 * Ray directions for Rook
 * . N .
 * W R E
 * . S .
 */
pub fn compute_rook_moves(
    start_location: &Bitboard,
    own_side: &Bitboard,
    enemy_side: &Bitboard,
) -> Bitboard {
    // Get file from start_location
    let start_index = start_location
        .least_significant_bit()
        .expect("Invalid start location");
    let square = Square::from_index(start_index);
    let file = square.file();

    // We iterate max MAX_RANGE times, or until we hit the end of the board
    let num_files_to_edge = File::last().to_index() - file.to_index();
    let iter_limit_eastern = std::cmp::min(num_files_to_edge, MAX_RANGE);
    let iter_limit_western = std::cmp::min(file.to_index(), MAX_RANGE);

    let northern_ray = compute_northern_ray(
        start_index.clone(),
        MAX_RANGE,
        BOARD_WIDTH,
        own_side,
        enemy_side,
    );
    let eastern_ray = compute_northern_ray(
        start_index.clone(),
        iter_limit_eastern,
        1,
        own_side,
        enemy_side,
    );
    let southern_ray = compute_southern_ray(
        start_index.clone(),
        MAX_RANGE,
        BOARD_WIDTH,
        own_side,
        enemy_side,
    );
    let western_ray = compute_southern_ray(
        start_index.clone(),
        iter_limit_western,
        1,
        own_side,
        enemy_side,
    );

    let mut legal_moves = northern_ray.clone();
    legal_moves.or(&eastern_ray);
    legal_moves.or(&southern_ray);
    legal_moves.or(&western_ray);

    //println!("{}", legal_moves);
    legal_moves
}

fn compute_northern_ray(
    start_index: usize,
    iter_limit: usize,
    step: usize,
    own_side: &Bitboard,
    enemy_pieces: &Bitboard,
) -> Bitboard {
    let mut ray = Bitboard::new();
    let mut index = start_index + step;

    for _ in 0..iter_limit {
        if index >= BOARD_SIZE {
            break;
        }
        let Some(blocker) = own_side.get(index) else {
            unreachable!()
        };
        if blocker {
            break;
        }
        let Some(enemy_piece) = enemy_pieces.get(index) else {
            unreachable!()
        };
        if enemy_piece {
            ray.set(index, true);
            break;
        }
        ray.set(index, true);
        index += step;
    }

    ray
}

fn compute_southern_ray(
    start_index: usize,
    iter_limit: usize,
    step: usize,
    own_side: &Bitboard,
    enemy_pieces: &Bitboard,
) -> Bitboard {
    let mut ray = Bitboard::new();
    let mut index = start_index;

    for _ in 0..iter_limit {
        if index < step {
            break;
        }

        index -= step;

        let Some(blocker) = own_side.get(index) else {
            unreachable!()
        };
        if blocker {
            break;
        }
        let Some(enemy_piece) = enemy_pieces.get(index) else {
            unreachable!()
        };
        if enemy_piece {
            ray.set(index, true);
            break;
        }
        ray.set(index, true);
    }

    ray
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_rook_moves_works() {
        // Base case
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
            0b10111111, 0b10000000,
            0b01000000, 0b00000000,
            0b01000000, 0b00000000,
            0b01000000, 0b00000000,
            0b01000000, 0b00000000,
            0b01000000, 0b00000000,
            0b01000000, 0b00000000,
            0b01000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
        ]);

        let result = compute_rook_moves(&start_loc, &Bitboard::new(), &Bitboard::new());
        assert_eq!(result, expected);

        // With a blocker piece
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
        let own_side = Bitboard::from_bytes(&[
            0b00000000, 0b00000000,
            0b00000010, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
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
        ]);

        #[rustfmt::skip]
        let expected = Bitboard::from_bytes(&[
            0b01000000, 0b00000000,
            0b10111100, 0b00000000,
            0b01000000, 0b00000000,
            0b01000000, 0b00000000,
            0b01000000, 0b00000000,
            0b01000000, 0b00000000,
            0b01000000, 0b00000000,
            0b01000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
        ]);

        let result = compute_rook_moves(&start_loc, &own_side, &Bitboard::new());
        assert_eq!(result, expected);

        // With an enemy piece
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
        let enemy_side = Bitboard::from_bytes(&[
            0b00000000, 0b00000000,
            0b00001000, 0b00000000,
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
        ]);

        #[rustfmt::skip]
        let expected = Bitboard::from_bytes(&[
            0b01000000, 0b00000000,
            0b10111000, 0b00000000,
            0b01000000, 0b00000000,
            0b01000000, 0b00000000,
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

        let result = compute_rook_moves(&start_loc, &Bitboard::new(), &enemy_side);
        assert_eq!(result, expected);

        // Test when Rook is near the top edge of the board
        #[rustfmt::skip]
        let start_loc = Bitboard::from_bytes(&[
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
            0b00000000, 0b00000100,
        ]);

        #[rustfmt::skip]
        let expected = Bitboard::from_bytes(&[
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000100,
            0b00000000, 0b00000100,
            0b00000000, 0b00000100,
            0b00000000, 0b00000100,
            0b00000000, 0b00000100,
            0b00000000, 0b00000100,
            0b00000000, 0b00000100,
            0b00000011, 0b11111011,
        ]);

        let result = compute_rook_moves(&start_loc, &Bitboard::new(), &Bitboard::new());
        assert_eq!(result, expected);

        #[rustfmt::skip]
        let mut start_loc = Bitboard::new();
        start_loc.set(Square::C13 as usize, true);

        #[rustfmt::skip]
        let expected = Bitboard::from_bytes(&[
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00100000, 0b00000000,
            0b00100000, 0b00000000,
            0b00100000, 0b00000000,
            0b00100000, 0b00000000,
            0b00100000, 0b00000000,
            0b00100000, 0b00000000,
            0b00100000, 0b00000000,
            0b11011111, 0b11000000,
            0b00100000, 0b00000000,
            0b00100000, 0b00000000,
            0b00100000, 0b00000000,
        ]);

        let result = compute_rook_moves(&start_loc, &Bitboard::new(), &Bitboard::new());
        assert_eq!(result, expected);

        // In NE quadrant
        #[rustfmt::skip]
        let start_loc = Bitboard::from_bytes(&[
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
            0b00000000, 0b00010000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
        ]);

        #[rustfmt::skip]
        let expected = Bitboard::from_bytes(&[
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00010000,
            0b00000000, 0b00010000,
            0b00000000, 0b00010000,
            0b00000000, 0b00010000,
            0b00000000, 0b00010000,
            0b00000000, 0b00010000,
            0b00000000, 0b00010000,
            0b00001111, 0b11101111,
            0b00000000, 0b00010000,
            0b00000000, 0b00010000,
            0b00000000, 0b00010000,
        ]);

        let result = compute_rook_moves(&start_loc, &Bitboard::new(), &Bitboard::new());
        println!("result:\n{}", result);
        println!("expected:\n{}", expected);
        assert_eq!(result, expected);
    }
}
