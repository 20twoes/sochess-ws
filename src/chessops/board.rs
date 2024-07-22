use std::collections::{HashMap, HashSet};

use crate::chessops::{movegen, Bitboard, Color, LookupTables, Move, Piece, Role, Square};

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

    pub fn is_legal_move(
        &self,
        move_: &Move,
        own_side: &HashSet<Color>,
        enemy_side: &HashSet<Color>,
    ) -> bool {
        let piece = move_.to_piece();

        // Check that starting position is consistent with our current position
        let _ = self
            .by_piece
            .get(&piece)
            .expect("Corresponing piece is not on the board");
        let mut start_loc = Bitboard::new();
        start_loc.set(move_.from.clone() as usize, true);

        // Construct bitboard for own side's pieces
        let mut own_side_bitboard = Bitboard::new();
        for color in own_side {
            if let Some(bitboard) = self.by_color.get(color) {
                own_side_bitboard.or(&bitboard);
            }
        }

        // Construct bitboard for enemy side's pieces
        let mut enemy_side_bitboard = Bitboard::new();
        for color in enemy_side {
            if let Some(bitboard) = self.by_color.get(color) {
                enemy_side_bitboard.or(&bitboard);
            }
        }

        let legal_moves = match move_.role {
            Role::Bishop => movegen::compute_bishop_moves(
                &start_loc,
                &own_side_bitboard,
                &enemy_side_bitboard,
                &self.lookup_tables,
            ),
            Role::King => {
                movegen::compute_king_moves(&start_loc, &own_side_bitboard, &self.lookup_tables)
            }
            Role::Knight => {
                movegen::compute_knight_moves(&start_loc, &own_side_bitboard, &self.lookup_tables)
            }
            Role::Rook => {
                movegen::compute_rook_moves(&start_loc, &own_side_bitboard, &self.lookup_tables)
            }
            _ => {
                return true;
            }
        };

        legal_moves.get(move_.to.clone() as usize).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let enemy_side = HashSet::new();
        assert!(board.is_legal_move(&move_, &own_side, &enemy_side));

        let move_ = Move {
            color: Color::White,
            role: Role::King,
            from: Square::A1,
            to: Square::C1,
        };

        assert!(!board.is_legal_move(&move_, &own_side, &enemy_side));
    }
}
