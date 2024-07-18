use std::collections::{HashMap, HashSet};

use crate::chessops::{
    Bitboard, Color, File, LookupTables, Move, Piece, Role, Square, BOARD_WIDTH,
};

#[derive(Debug, PartialEq)]
pub struct Board {
    pub by_square: HashMap<Square, Piece>,
    /// e.g. locations of all white pawns
    pub by_piece: HashMap<Piece, Bitboard>,
    pub by_color: HashMap<Color, Bitboard>,
    lookup_tables: LookupTables,
}

impl Board {
    pub fn new() -> Self {
        Self {
            by_square: HashMap::new(),
            by_piece: HashMap::new(),
            by_color: HashMap::new(),
            lookup_tables: LookupTables::new(),
        }
    }

    pub fn insert_piece(&mut self, square: Square, piece: Piece) {
        self.by_square.insert(square.clone(), piece.clone());

        match self.by_piece.contains_key(&piece) {
            true => {
                self.by_piece
                    .get_mut(&piece)
                    .unwrap()
                    .set(square.clone() as usize, true);
            }
            false => {
                let mut bitboard = Bitboard::new();
                bitboard.set(square.clone() as usize, true);
                self.by_piece.insert(piece.clone(), bitboard);
            }
        }

        match self.by_color.contains_key(&piece.color) {
            true => {
                self.by_color
                    .get_mut(&piece.color)
                    .unwrap()
                    .set(square as usize, true);
            }
            false => {
                let mut bitboard = Bitboard::new();
                bitboard.set(square as usize, true);
                self.by_color.insert(piece.color, bitboard);
            }
        }
    }

    pub fn is_legal_move(&self, move_: &Move, own_side: &HashSet<Color>) -> bool {
        // Construct bitboard for own side's pieces
        let mut own_side_bitboard = Bitboard::new();
        for color in own_side {
            if let Some(bitboard) = self.by_color.get(color) {
                own_side_bitboard.or(&bitboard);
            }
        }

        match move_.role {
            Role::King => self.is_legal_king_move(&move_, &own_side_bitboard),
            _ => true,
        }
    }

    fn is_legal_king_move(&self, move_: &Move, own_side: &Bitboard) -> bool {
        let piece = move_.to_piece();
        let king_loc = self.by_piece.get(&piece).expect("No king on the board");
        let valid_moves = compute_king_moves(&king_loc, &own_side, &self.lookup_tables);
        valid_moves.get(move_.to.clone() as usize).unwrap()
    }
}

/**
* Explanation: https://pages.cs.wisc.edu/~psilord/blog/data/chess-pages/nonsliding.html
* Legal spots the King can move to:
* 1 2 3
* 8 K 4
* 7 6 5
*/
fn compute_king_moves(
    king_location: &Bitboard,
    own_side: &Bitboard,
    lookup_tables: &LookupTables,
) -> Bitboard {
    let mut king_clip_file_a = king_location.clone();
    king_clip_file_a.and(&lookup_tables.clear_file[File::A as usize]);

    let mut king_clip_file_p = king_location.clone();
    king_clip_file_p.and(&lookup_tables.clear_file[File::P as usize]);

    let spot_1 = king_clip_file_a.shift_right(BOARD_WIDTH - 1);
    let spot_2 = king_location.shift_right(BOARD_WIDTH);
    let spot_3 = king_clip_file_p.shift_right(BOARD_WIDTH + 1);
    let spot_4 = king_clip_file_p.shift_right(1);
    let spot_5 = king_clip_file_p.shift_left(BOARD_WIDTH - 1);
    let spot_6 = king_location.shift_left(BOARD_WIDTH);
    let spot_7 = king_clip_file_a.shift_left(BOARD_WIDTH + 1);
    let spot_8 = king_clip_file_a.shift_left(1);

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
    fn king_moves_are_legal() {
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

    #[test]
    fn is_legal_move_works() {
        let mut king_loc = Bitboard::new();
        king_loc.set(Square::A1 as usize, true);

        let mut board = Board::new();
        board.by_piece = HashMap::from([(
            Piece {
                color: Color::White,
                role: Role::King,
            },
            king_loc,
        )]);

        let move_ = Move {
            color: Color::White,
            role: Role::King,
            from: Square::A1,
            to: Square::B1,
        };

        let own_side = HashSet::from([Color::White]);
        assert!(board.is_legal_move(&move_, &own_side));

        let move_ = Move {
            color: Color::White,
            role: Role::King,
            from: Square::A1,
            to: Square::C1,
        };

        assert!(!board.is_legal_move(&move_, &own_side));
    }
}
