use crate::chessops::{Bitboard, File, LookupTables, Quadrant, Rank, BOARD_WIDTH};

pub fn compute_pawn_moves(
    start_location: &Bitboard,
    all_pieces: &Bitboard,
    enemy_pieces: &Bitboard,
    lookup_tables: &LookupTables,
) -> Bitboard {
    // Pawns in each quadrant move in different directions.
    // Treat them accordingly.
    let mut pawn_in_q1 = start_location.clone();
    pawn_in_q1.and(&lookup_tables.mask_quadrant[Quadrant::SW.to_index()]);

    let mut pawn_in_q2 = start_location.clone();
    pawn_in_q2.and(&lookup_tables.mask_quadrant[Quadrant::SE.to_index()]);

    let mut pawn_in_q3 = start_location.clone();
    pawn_in_q3.and(&lookup_tables.mask_quadrant[Quadrant::NW.to_index()]);

    let mut pawn_in_q4 = start_location.clone();
    pawn_in_q4.and(&lookup_tables.mask_quadrant[Quadrant::NE.to_index()]);

    if pawn_in_q1.any() {
        return compute_sw_pawn_moves(start_location, all_pieces, enemy_pieces, lookup_tables);
    } else if pawn_in_q2.any() {
        return compute_se_pawn_moves(start_location, all_pieces, enemy_pieces, lookup_tables);
    } else if pawn_in_q3.any() {
        return compute_nw_pawn_moves(start_location, all_pieces, enemy_pieces, lookup_tables);
    } else if pawn_in_q4.any() {
        return compute_ne_pawn_moves(start_location, all_pieces, enemy_pieces, lookup_tables);
    }

    unreachable!()
}

fn compute_pawn_moves_base(
    start_location: &Bitboard,
    all_pieces: &Bitboard,
    enemy_pieces: &Bitboard,
    lookup_tables: &LookupTables,
    end_file: File,
    end_rank: Rank,
    // The rank the pawn ends up on after one step if starting from the first ring
    first_ring_rank: Rank,
    second_ring_rank: Rank,
    first_ring_file: File,
    second_ring_file: File,
    left_attack_file_edge: File,
    left_attack_rank_edge: Rank,
    right_attack_file_edge: File,
    right_attack_rank_edge: Rank,
    move_vertically: fn(&mut Bitboard),
    move_horizontally: fn(&mut Bitboard),
    attack_left: fn(&mut Bitboard),
    attack_center: fn(&mut Bitboard),
    attack_right: fn(&mut Bitboard),
) -> Bitboard {
    let mut empty_squares = all_pieces.clone();
    empty_squares.not();

    // Check one space in front of pawn
    let mut one_step = start_location.clone();
    one_step.and(&lookup_tables.clear_rank[end_rank.to_index()]);
    move_vertically(&mut one_step);
    one_step.and(&empty_squares);

    // Check for two steps when pawn is on rank 1
    let mut two_steps = one_step.clone();
    two_steps.and(&lookup_tables.mask_rank[first_ring_rank.to_index()]);
    move_vertically(&mut two_steps);
    two_steps.and(&empty_squares);

    // Check for two steps when pawn is on rank 2
    let mut two_steps_2 = one_step.clone();
    two_steps_2.and(&lookup_tables.mask_rank[second_ring_rank.to_index()]);
    move_vertically(&mut two_steps_2);
    two_steps_2.and(&empty_squares);

    // Check for one side step
    let mut one_side_step = start_location.clone();
    one_side_step.and(&lookup_tables.clear_file[end_file.to_index()]);
    move_horizontally(&mut one_side_step);
    one_side_step.and(&empty_squares);

    // Check for two side steps when pawn is on File A
    let mut two_side_steps = one_side_step.clone();
    two_side_steps.and(&lookup_tables.mask_file[first_ring_file.to_index()]);
    move_horizontally(&mut two_side_steps);
    two_side_steps.and(&empty_squares);

    // Check for two side steps when pawn is on File B
    let mut two_side_steps_2 = one_side_step.clone();
    two_side_steps_2.and(&lookup_tables.mask_file[second_ring_file.to_index()]);
    move_horizontally(&mut two_side_steps_2);
    two_side_steps_2.and(&empty_squares);

    // Check for attacks
    // There are at most three attacks.  We reference them from the POV of the pawn headed to the
    // center of the board:
    // - Left
    //  - Pawn in Quadrant 1 (SW quadrant), attack left and up a square
    //  - Pawn in Quadrant 2 (SE quadrant), attack left and down a square
    //  - Pawn in Quadrant 3 (NW quadrant), attack right and up a square
    //  - Pawn in Quadrant 4 (NE quadrant), attack right and down a square
    // - Center
    //  - Pawn in Quadrant 1 (SW quadrant), attack right and up a square
    //  - Pawn in Quadrant 2 (SE quadrant), attack left and up a square
    //  - Pawn in Quadrant 3 (NW quadrant), attack right and down a square
    //  - Pawn in Quadrant 4 (NE quadrant), attack left and down a square
    // - Right
    //  - Pawn in Quadrant 1 (SW quadrant), attack right and down a square
    //  - Pawn in Quadrant 2 (SE quadrant), attack right and up a square
    //  - Pawn in Quadrant 3 (NW quadrant), attack left and down a square
    //  - Pawn in Quadrant 4 (NE quadrant), attack left and up a square
    let mut left_attack = start_location.clone();
    left_attack.and(&lookup_tables.clear_file[left_attack_file_edge.to_index()]);
    left_attack.and(&lookup_tables.clear_rank[left_attack_rank_edge.to_index()]);
    attack_left(&mut left_attack);

    let mut center_attack = start_location.clone();
    attack_center(&mut center_attack);

    let mut right_attack = start_location.clone();
    right_attack.and(&lookup_tables.clear_file[right_attack_file_edge.to_index()]);
    right_attack.and(&lookup_tables.clear_rank[right_attack_rank_edge.to_index()]);
    attack_right(&mut right_attack);

    let mut all_attacks = left_attack.clone();
    all_attacks.or(&center_attack);
    all_attacks.or(&right_attack);
    all_attacks.and(&enemy_pieces);

    let mut legal_moves = one_step.clone();
    legal_moves.or(&two_steps);
    legal_moves.or(&two_steps_2);
    legal_moves.or(&one_side_step);
    legal_moves.or(&two_side_steps);
    legal_moves.or(&two_side_steps_2);
    legal_moves.or(&all_attacks);

    legal_moves
}

fn compute_sw_pawn_moves(
    start_location: &Bitboard,
    all_pieces: &Bitboard,
    enemy_pieces: &Bitboard,
    lookup_tables: &LookupTables,
) -> Bitboard {
    let end_file: File = File::H;
    let end_rank: Rank = Rank::R8;

    let first_ring_rank: Rank = Rank::R2;
    let second_ring_rank: Rank = Rank::R3;
    let first_ring_file: File = File::B;
    let second_ring_file: File = File::C;

    let left_attack_file_edge: File = File::A;
    let left_attack_rank_edge: Rank = Rank::R8;
    let right_attack_file_edge: File = File::H;
    let right_attack_rank_edge: Rank = Rank::R1;

    fn move_vertically(b: &mut Bitboard) {
        b.shift_right(BOARD_WIDTH);
    }

    fn move_horizontally(b: &mut Bitboard) {
        b.shift_right(1);
    }

    fn attack_left(b: &mut Bitboard) {
        b.shift_right(BOARD_WIDTH - 1);
    }

    fn attack_center(b: &mut Bitboard) {
        b.shift_right(BOARD_WIDTH + 1);
    }

    fn attack_right(b: &mut Bitboard) {
        b.shift_left(BOARD_WIDTH - 1);
    }

    compute_pawn_moves_base(
        start_location,
        all_pieces,
        enemy_pieces,
        lookup_tables,
        end_file,
        end_rank,
        first_ring_rank,
        second_ring_rank,
        first_ring_file,
        second_ring_file,
        left_attack_file_edge,
        left_attack_rank_edge,
        right_attack_file_edge,
        right_attack_rank_edge,
        move_vertically,
        move_horizontally,
        attack_left,
        attack_center,
        attack_right,
    )
}

fn compute_se_pawn_moves(
    start_location: &Bitboard,
    all_pieces: &Bitboard,
    enemy_pieces: &Bitboard,
    lookup_tables: &LookupTables,
) -> Bitboard {
    let end_file: File = File::I;
    let end_rank: Rank = Rank::R8;

    let first_ring_rank: Rank = Rank::R2;
    let second_ring_rank: Rank = Rank::R3;
    let first_ring_file: File = File::O;
    let second_ring_file: File = File::N;

    let left_attack_file_edge: File = File::I;
    let left_attack_rank_edge: Rank = Rank::R1;
    let right_attack_file_edge: File = File::P;
    let right_attack_rank_edge: Rank = Rank::R8;

    fn move_vertically(b: &mut Bitboard) {
        b.shift_right(BOARD_WIDTH);
    }

    fn move_horizontally(b: &mut Bitboard) {
        b.shift_left(1);
    }

    fn attack_left(b: &mut Bitboard) {
        b.shift_left(BOARD_WIDTH + 1);
    }

    fn attack_center(b: &mut Bitboard) {
        b.shift_right(BOARD_WIDTH - 1);
    }

    fn attack_right(b: &mut Bitboard) {
        b.shift_right(BOARD_WIDTH + 1);
    }

    compute_pawn_moves_base(
        start_location,
        all_pieces,
        enemy_pieces,
        lookup_tables,
        end_file,
        end_rank,
        first_ring_rank,
        second_ring_rank,
        first_ring_file,
        second_ring_file,
        left_attack_file_edge,
        left_attack_rank_edge,
        right_attack_file_edge,
        right_attack_rank_edge,
        move_vertically,
        move_horizontally,
        attack_left,
        attack_center,
        attack_right,
    )
}

fn compute_nw_pawn_moves(
    start_location: &Bitboard,
    all_pieces: &Bitboard,
    enemy_pieces: &Bitboard,
    lookup_tables: &LookupTables,
) -> Bitboard {
    let end_file: File = File::H;
    let end_rank: Rank = Rank::R9;

    let first_ring_rank: Rank = Rank::R15;
    let second_ring_rank: Rank = Rank::R14;
    let first_ring_file: File = File::B;
    let second_ring_file: File = File::C;

    let left_attack_file_edge: File = File::H;
    let left_attack_rank_edge: Rank = Rank::R16;
    let right_attack_file_edge: File = File::A;
    let right_attack_rank_edge: Rank = Rank::R9;

    fn move_vertically(b: &mut Bitboard) {
        b.shift_left(BOARD_WIDTH);
    }

    fn move_horizontally(b: &mut Bitboard) {
        b.shift_right(1);
    }

    fn attack_left(b: &mut Bitboard) {
        b.shift_right(BOARD_WIDTH + 1);
    }

    fn attack_center(b: &mut Bitboard) {
        b.shift_left(BOARD_WIDTH - 1);
    }

    fn attack_right(b: &mut Bitboard) {
        b.shift_left(BOARD_WIDTH + 1);
    }

    compute_pawn_moves_base(
        start_location,
        all_pieces,
        enemy_pieces,
        lookup_tables,
        end_file,
        end_rank,
        first_ring_rank,
        second_ring_rank,
        first_ring_file,
        second_ring_file,
        left_attack_file_edge,
        left_attack_rank_edge,
        right_attack_file_edge,
        right_attack_rank_edge,
        move_vertically,
        move_horizontally,
        attack_left,
        attack_center,
        attack_right,
    )
}

fn compute_ne_pawn_moves(
    start_location: &Bitboard,
    all_pieces: &Bitboard,
    enemy_pieces: &Bitboard,
    lookup_tables: &LookupTables,
) -> Bitboard {
    let end_file: File = File::I;
    let end_rank: Rank = Rank::R9;

    let first_ring_rank: Rank = Rank::R15;
    let second_ring_rank: Rank = Rank::R14;
    let first_ring_file: File = File::O;
    let second_ring_file: File = File::N;

    let left_attack_file_edge: File = File::P;
    let left_attack_rank_edge: Rank = Rank::R9;
    let right_attack_file_edge: File = File::I;
    let right_attack_rank_edge: Rank = Rank::R16;

    fn move_vertically(b: &mut Bitboard) {
        b.shift_left(BOARD_WIDTH);
    }

    fn move_horizontally(b: &mut Bitboard) {
        b.shift_left(1);
    }

    fn attack_left(b: &mut Bitboard) {
        b.shift_left(BOARD_WIDTH - 1);
    }

    fn attack_center(b: &mut Bitboard) {
        b.shift_left(BOARD_WIDTH + 1);
    }

    fn attack_right(b: &mut Bitboard) {
        b.shift_right(BOARD_WIDTH - 1);
    }

    compute_pawn_moves_base(
        start_location,
        all_pieces,
        enemy_pieces,
        lookup_tables,
        end_file,
        end_rank,
        first_ring_rank,
        second_ring_rank,
        first_ring_file,
        second_ring_file,
        left_attack_file_edge,
        left_attack_rank_edge,
        right_attack_file_edge,
        right_attack_rank_edge,
        move_vertically,
        move_horizontally,
        attack_left,
        attack_center,
        attack_right,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_pawn_moves_for_pawn_in_sw_quadrant_works() {
        // Pawn on first rank
        #[rustfmt::skip]
        let start_loc = Bitboard::from_bytes(&[
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
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
        ]);

        #[rustfmt::skip]
        let expected = Bitboard::from_bytes(&[
            0b00010000, 0b00000000,
            0b00100000, 0b00000000,
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

        let result = compute_pawn_moves(
            &start_loc,
            &Bitboard::new(),
            &Bitboard::new(),
            &LookupTables::new(),
        );
        assert_eq!(result, expected);

        // Pawn with blocker in front
        #[rustfmt::skip]
        let start_loc = Bitboard::from_bytes(&[
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
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
        ]);

        #[rustfmt::skip]
        let all_pieces = Bitboard::from_bytes(&[
            0b00010000, 0b00000000,
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
            0b00000000, 0b00000000,
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
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
        ]);

        let result = compute_pawn_moves(
            &start_loc,
            &all_pieces,
            &Bitboard::new(),
            &LookupTables::new(),
        );
        assert_eq!(result, expected);

        // Pawn with blocker two squares away
        #[rustfmt::skip]
        let start_loc = Bitboard::from_bytes(&[
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
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
        ]);

        #[rustfmt::skip]
        let all_pieces = Bitboard::from_bytes(&[
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
            0b00010000, 0b00000000,
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
            0b00000000, 0b00000000,
        ]);

        let result = compute_pawn_moves(
            &start_loc,
            &all_pieces,
            &Bitboard::new(),
            &LookupTables::new(),
        );
        assert_eq!(result, expected);

        // Pawn with valid capture moves
        #[rustfmt::skip]
        let start_loc = Bitboard::from_bytes(&[
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
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
        ]);

        #[rustfmt::skip]
        let all_pieces = Bitboard::from_bytes(&[
            0b00000000, 0b00000000,
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
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
        ]);

        #[rustfmt::skip]
        let expected = Bitboard::from_bytes(&[
            0b00010000, 0b00000000,
            0b01110000, 0b00000000,
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

        let result = compute_pawn_moves(&start_loc, &all_pieces, &all_pieces, &LookupTables::new());
        assert_eq!(result, expected);

        // Pawn on second rank
        #[rustfmt::skip]
        let start_loc = Bitboard::from_bytes(&[
            0b00000000, 0b00000000,
            0b00000100, 0b00000000,
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
            0b00000010, 0b00000000,
            0b00000100, 0b00000000,
            0b00000100, 0b00000000,
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

        let result = compute_pawn_moves(
            &start_loc,
            &Bitboard::new(),
            &Bitboard::new(),
            &LookupTables::new(),
        );
        assert_eq!(result, expected);

        // Pawn in middle of quadrant
        #[rustfmt::skip]
        let start_loc = Bitboard::from_bytes(&[
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
            0b00000000, 0b00000000,
        ]);

        #[rustfmt::skip]
        let enemy_pieces = Bitboard::from_bytes(&[
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000100, 0b00000000,
            0b00000000, 0b00000000,
            0b00010100, 0b00000000,
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
            0b00000000, 0b00000000,
            0b00000100, 0b00000000,
            0b00000100, 0b00000000,
            0b00011100, 0b00000000,
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

        let result = compute_pawn_moves(
            &start_loc,
            &enemy_pieces,
            &enemy_pieces,
            &LookupTables::new(),
        );
        assert_eq!(result, expected);

        // Pawn on edge of quadrant (File H)
        #[rustfmt::skip]
        let start_loc = Bitboard::from_bytes(&[
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
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
        ]);

        #[rustfmt::skip]
        let enemy_pieces = Bitboard::from_bytes(&[
            0b00000000, 0b10000000,
            0b00000000, 0b00000000,
            0b00000010, 0b10000000,
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
            0b00000000, 0b00000000,
            0b00000011, 0b10000000,
            0b00000001, 0b00000000,
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

        let result = compute_pawn_moves(
            &start_loc,
            &enemy_pieces,
            &enemy_pieces,
            &LookupTables::new(),
        );
        assert_eq!(result, expected);

        // Pawn at end of quadrant (Rank 8)
        #[rustfmt::skip]
        let start_loc = Bitboard::from_bytes(&[
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
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
        ]);

        #[rustfmt::skip]
        let enemy_pieces = Bitboard::from_bytes(&[
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b10000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00010000, 0b00000000,
            0b00000000, 0b00000000,
            0b01010000, 0b00000000,
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
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00010000, 0b00000000,
            0b00010000, 0b00000000,
            0b00010000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
        ]);

        let result = compute_pawn_moves(
            &start_loc,
            &enemy_pieces,
            &enemy_pieces,
            &LookupTables::new(),
        );
        assert_eq!(result, expected);

        // Pawn on first ring, but on the side
        #[rustfmt::skip]
        let start_loc = Bitboard::from_bytes(&[
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
        let enemy_pieces = Bitboard::from_bytes(&[
            0b00000000, 0b00000000,
            0b01000000, 0b00000000,
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
        ]);

        #[rustfmt::skip]
        let expected = Bitboard::from_bytes(&[
            0b00000000, 0b00000000,
            0b01000000, 0b00000000,
            0b01100000, 0b00000000,
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

        let result = compute_pawn_moves(
            &start_loc,
            &enemy_pieces,
            &enemy_pieces,
            &LookupTables::new(),
        );
        assert_eq!(result, expected);

        // Pawn on second ring, but on the side
        #[rustfmt::skip]
        let start_loc = Bitboard::from_bytes(&[
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
        let enemy_pieces = Bitboard::from_bytes(&[
            0b00000000, 0b00000000,
            0b10100000, 0b00000000,
            0b00000000, 0b00000000,
            0b10100000, 0b00000000,
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
            0b00110000, 0b00000000,
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
        ]);

        let result = compute_pawn_moves(
            &start_loc,
            &enemy_pieces,
            &enemy_pieces,
            &LookupTables::new(),
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn compute_pawn_moves_for_pawn_in_se_quadrant_works() {
        #[rustfmt::skip]
        let start_loc = Bitboard::from_bytes(&[
            0b00000000, 0b00000100,
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
        let enemy_pieces = Bitboard::from_bytes(&[
            0b00000000, 0b00000000,
            0b00000000, 0b00001010,
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
            0b00000000, 0b00001000,
            0b00000000, 0b00001110,
            0b00000000, 0b00000100,
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

        let result = compute_pawn_moves(
            &start_loc,
            &enemy_pieces,
            &enemy_pieces,
            &LookupTables::new(),
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn compute_pawn_moves_for_pawn_in_nw_quadrant_works() {
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
            0b00010000, 0b00000000,
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
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00101000, 0b00000000,
            0b00000000, 0b00000000,
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
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00010000, 0b00000000,
            0b00111000, 0b00000000,
            0b00001000, 0b00000000,
        ]);

        let result = compute_pawn_moves(
            &start_loc,
            &enemy_pieces,
            &enemy_pieces,
            &LookupTables::new(),
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn compute_pawn_moves_for_pawn_in_ne_quadrant_works() {
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
            0b00000000, 0b10000000,
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
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000001, 0b01000000,
            0b00000000, 0b00000000,
            0b00000001, 0b00000000,
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
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000001, 0b11000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
        ]);

        let result = compute_pawn_moves(
            &start_loc,
            &enemy_pieces,
            &enemy_pieces,
            &LookupTables::new(),
        );
        assert_eq!(result, expected);
    }
}
