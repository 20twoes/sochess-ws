use crate::chessops::{movegen::MAX_RANGE, Bitboard, File, Square, BOARD_SIZE, BOARD_WIDTH};

/**
 * Use directions to specfiy the four different rays of a bishop:
 * NW  .  .  . NE
 *  . NW  . NE  .
 *  .  .  B  .  .
 *  . SW  . SE  .
 * SW  .  .  . SE
 */
pub fn compute_bishop_moves(
    start_location: &Bitboard,
    own_side: &Bitboard,
    enemy_pieces: &Bitboard,
) -> Bitboard {
    let start_index = start_location
        .least_significant_bit()
        .expect("Invalid start location");
    let square = Square::from_index(start_index);
    let file = square.file();
    let mut legal_moves = Bitboard::new();

    // We iterate max MAX_RANGE times, or until we hit the end of the board
    let num_files_to_edge = File::last().to_index() - file.to_index();
    let limit_iter_eastern = std::cmp::min(num_files_to_edge, MAX_RANGE);
    let limit_iter_western = std::cmp::min(file.to_index(), MAX_RANGE);

    let northeast_ray = compute_bishop_northern_ray(
        start_index.clone(),
        limit_iter_eastern,
        BOARD_WIDTH + 1,
        own_side,
        enemy_pieces,
    );
    let northwest_ray = compute_bishop_northern_ray(
        start_index.clone(),
        limit_iter_western,
        BOARD_WIDTH - 1,
        own_side,
        enemy_pieces,
    );
    let southwest_ray = compute_bishop_southern_ray(
        start_index.clone(),
        limit_iter_western,
        BOARD_WIDTH + 1,
        own_side,
        enemy_pieces,
    );
    let southeast_ray = compute_bishop_southern_ray(
        start_index.clone(),
        limit_iter_eastern,
        BOARD_WIDTH - 1,
        own_side,
        enemy_pieces,
    );

    legal_moves.or(&northeast_ray);
    legal_moves.or(&northwest_ray);
    legal_moves.or(&southwest_ray);
    legal_moves.or(&southeast_ray);

    legal_moves
}

fn compute_bishop_northern_ray(
    start_index: usize,
    iter_limit: usize,
    step: usize,
    own_side: &Bitboard,
    enemy_pieces: &Bitboard,
) -> Bitboard {
    let mut ray = Bitboard::new();
    let mut index = start_index.clone();

    index += step;

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

fn compute_bishop_southern_ray(
    start_index: usize,
    iter_limit: usize,
    step: usize,
    own_side: &Bitboard,
    enemy_pieces: &Bitboard,
) -> Bitboard {
    let mut ray = Bitboard::new();
    let mut index = start_index.clone();

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
    fn compute_bishop_moves_works() {
        // Base case
        #[rustfmt::skip]
        let start_loc = Bitboard::from_bytes(&[
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00001000, 0b00000000,
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
            0b10000000, 0b10000000,
            0b01000001, 0b00000000,
            0b00100010, 0b00000000,
            0b00010100, 0b00000000,
            0b00000000, 0b00000000,
            0b00010100, 0b00000000,
            0b00100010, 0b00000000,
            0b01000001, 0b00000000,
            0b10000000, 0b10000000,
            0b00000000, 0b01000000,
            0b00000000, 0b00100000,
            0b00000000, 0b00010000,
            0b00000000, 0b00001000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
        ]);

        let result = compute_bishop_moves(&start_loc, &Bitboard::new(), &Bitboard::new());

        assert_eq!(result, expected);

        // With blocking piece
        #[rustfmt::skip]
        let start_loc = Bitboard::from_bytes(&[
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00001000, 0b00000000,
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
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000010, 0b00000000,
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
            0b10000000, 0b10000000,
            0b01000001, 0b00000000,
            0b00100010, 0b00000000,
            0b00010100, 0b00000000,
            0b00000000, 0b00000000,
            0b00010100, 0b00000000,
            0b00100000, 0b00000000,
            0b01000000, 0b00000000,
            0b10000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
        ]);

        let result = compute_bishop_moves(&start_loc, &own_side, &Bitboard::new());

        assert_eq!(result, expected);

        // With enemy blocking piece
        #[rustfmt::skip]
        let start_loc = Bitboard::from_bytes(&[
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00001000, 0b00000000,
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
        let enemy_pieces = Bitboard::from_bytes(&[
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000001, 0b00000000,
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
            0b10000000, 0b10000000,
            0b01000001, 0b00000000,
            0b00100010, 0b00000000,
            0b00010100, 0b00000000,
            0b00000000, 0b00000000,
            0b00010100, 0b00000000,
            0b00100010, 0b00000000,
            0b01000001, 0b00000000,
            0b10000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
        ]);

        let result = compute_bishop_moves(&start_loc, &Bitboard::new(), &enemy_pieces);

        assert_eq!(result, expected);

        // In another quadrant
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
            0b00010000, 0b00000000,
            0b00001000, 0b00000000,
            0b00000100, 0b00000000,
            0b00000010, 0b00000000,
            0b00000001, 0b00000001,
            0b00000000, 0b10000010,
            0b00000000, 0b01000100,
            0b00000000, 0b00101000,
            0b00000000, 0b00000000,
            0b00000000, 0b00101000,
            0b00000000, 0b01000100,
            0b00000000, 0b10000010,
        ]);

        let result = compute_bishop_moves(&start_loc, &Bitboard::new(), &Bitboard::new());

        println!("result:\n{}", result);
        println!("expected:\n{}", expected);
        assert_eq!(result, expected);
    }
}
