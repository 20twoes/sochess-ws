use std::collections::{HashMap, HashSet};

use crate::chessops::{movegen, Bitboard, Color, LookupTables, Move, Piece, Role, Square};

#[derive(Debug, PartialEq)]
pub struct Board {
    pub by_square: HashMap<Square, Piece>,
    /// e.g. locations of all white pawns
    pub by_piece: HashMap<Piece, Bitboard>,
    pub by_color: HashMap<Color, Bitboard>,
    all_pieces: Bitboard,
    lookup_tables: LookupTables,
    occupied_colored_squares: HashSet<Color>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            by_square: HashMap::new(),
            by_piece: HashMap::new(),
            by_color: HashMap::new(),
            all_pieces: Bitboard::new(),
            lookup_tables: LookupTables::new(),
            occupied_colored_squares: HashSet::new(),
        }
    }

    pub fn insert_piece(&mut self, square: Square, piece: Piece) {
        self.by_square.insert(square.clone(), piece.clone());
        self.all_pieces.set(square.to_index(), true);

        match self.by_piece.contains_key(&piece) {
            true => {
                self.by_piece
                    .get_mut(&piece)
                    .unwrap()
                    .set(square.to_index(), true);
            }
            false => {
                let mut bitboard = Bitboard::new();
                bitboard.set(square.to_index(), true);
                self.by_piece.insert(piece.clone(), bitboard);
            }
        }

        match self.by_color.contains_key(&piece.color) {
            true => {
                self.by_color
                    .get_mut(&piece.color)
                    .unwrap()
                    .set(square.to_index(), true);
            }
            false => {
                let mut bitboard = Bitboard::new();
                bitboard.set(square.to_index(), true);
                self.by_color.insert(piece.color, bitboard);
            }
        }

        if let Some(color) = square.clone().color() {
            self.occupied_colored_squares.insert(color);
        }
    }

    pub fn remove_piece(&mut self, piece: Piece) {
        // Look up square that piece is located on
        let loc = self.by_piece.get_mut(&piece).unwrap();
        let index = loc
            .least_significant_bit()
            .expect("Invalid bitboard location");
        let square = Square::from_index(index);

        // Create a mask to clear the piece's location
        let mut mask = loc.clone();
        mask.not();

        self.by_square.remove(&square);
        self.by_piece.remove(&piece);

        let color_loc = self.by_color.get_mut(&piece.color).unwrap();
        color_loc.and(&mask);

        self.all_pieces.and(&mask);

        if let Some(color) = square.color() {
            self.occupied_colored_squares.remove(&color);
        }
    }

    #[cfg(test)]
    pub fn get(&self, square: &Square) -> Option<&Piece> {
        self.by_square.get(square)
    }

    pub fn find(&self, piece: &Piece) -> Option<Square> {
        // TODO: Should catch these errors and return None
        let bitboard = self
            .by_piece
            .get(piece)
            .expect(&format!("Piece not found: {:?}", piece));
        let index = bitboard
            .least_significant_bit()
            .expect("Bitboard was empty");
        Some(Square::from_index(index))
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

        let mut legal_moves = match move_.role {
            Role::Bishop => {
                movegen::compute_bishop_moves(&start_loc, &own_side_bitboard, &enemy_side_bitboard)
            }
            Role::King => {
                movegen::compute_king_moves(&start_loc, &own_side_bitboard, &self.lookup_tables)
            }
            Role::Knight => {
                movegen::compute_knight_moves(&start_loc, &own_side_bitboard, &self.lookup_tables)
            }
            Role::Pawn => movegen::compute_pawn_moves(
                &start_loc,
                &self.all_pieces,
                &enemy_side_bitboard,
                &self.lookup_tables,
            ),
            Role::Queen => {
                let mut queen_moves = movegen::compute_rook_moves(
                    &start_loc,
                    &own_side_bitboard,
                    &enemy_side_bitboard,
                );
                queen_moves.or(&movegen::compute_bishop_moves(
                    &start_loc,
                    &own_side_bitboard,
                    &enemy_side_bitboard,
                ));
                queen_moves
            }
            Role::Rook => {
                movegen::compute_rook_moves(&start_loc, &own_side_bitboard, &enemy_side_bitboard)
            }
        };

        // Only one square of each color may be occupied at a time
        let mut legal_colored_squares_mask = self.build_colored_squares_mask();
        // Allow capturing a piece on a colored square
        legal_colored_squares_mask.or(&enemy_side_bitboard);
        legal_moves.and(&legal_colored_squares_mask);

        // No piece may move onto a square of its own color
        legal_moves.and(
            &self
                .lookup_tables
                .clear_colored_squares
                .get(&move_.color)
                .unwrap(),
        );

        legal_moves.get(move_.to.to_index()).unwrap()
    }

    /// Return a bitboard with valid moves to legal colored squares
    pub fn build_colored_squares_mask(&self) -> Bitboard {
        let mut valid_squares = Bitboard::new_full();
        for color in &self.occupied_colored_squares {
            valid_squares.and(&self.lookup_tables.clear_colored_squares.get(color).unwrap());
        }
        valid_squares
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

        let move_ = Move::new(Color::White, Role::King, Square::A1, Square::B1);

        let own_side = HashSet::from([Color::White]);
        let enemy_side = HashSet::new();
        assert!(board.is_legal_move(&move_, &own_side, &enemy_side));

        let move_ = Move::new(Color::White, Role::King, Square::A1, Square::C1);

        assert!(!board.is_legal_move(&move_, &own_side, &enemy_side));
    }

    #[test]
    fn is_legal_move_should_disallow_move_onto_occupied_colored_square() {
        let mut board = Board::new();
        // On Navy colored square
        board.insert_piece(
            Square::E5,
            Piece {
                color: Color::White,
                role: Role::Queen,
            },
        );
        // About to try and go into the other Navy square
        board.insert_piece(
            Square::L13,
            Piece {
                color: Color::Black,
                role: Role::Queen,
            },
        );
        // Will try to capture Queen on Navy square
        board.insert_piece(
            Square::E6,
            Piece {
                color: Color::Black,
                role: Role::Rook,
            },
        );

        let move_ = Move::new(Color::Black, Role::Queen, Square::L13, Square::L12);

        let own_side = HashSet::from([Color::Black]);
        let enemy_side = HashSet::from([Color::White]);
        assert!(!board.is_legal_move(&move_, &own_side, &enemy_side));

        // But should be able to capture a piece that is on a colored square
        let move_ = Move::new(Color::Black, Role::Rook, Square::E6, Square::E5);

        assert!(board.is_legal_move(&move_, &own_side, &enemy_side));
    }

    #[test]
    fn is_legal_move_should_disallow_moving_to_a_square_of_own_color() {
        let mut board = Board::new();
        // On Navy colored square
        board.insert_piece(
            Square::I3,
            Piece {
                color: Color::White,
                role: Role::Queen,
            },
        );

        let move_ = Move::new(Color::White, Role::Queen, Square::I3, Square::I8);

        let own_side = HashSet::from([Color::White]);
        let enemy_side = HashSet::from([Color::Black]);
        assert!(!board.is_legal_move(&move_, &own_side, &enemy_side));
    }

    #[test]
    fn remove_piece_works() {
        let mut board = Board::new();
        let color = Color::Pink;
        let piece = Piece::new(color.clone(), Role::King);
        let square = Square::E5;

        board.insert_piece(square.clone(), piece.clone());
        board.remove_piece(piece.clone());

        assert!(board.get(&square).is_none());
        assert!(board.by_square.get(&square).is_none());
        assert!(board.by_piece.get(&piece).is_none());
        assert!(!board
            .by_color
            .get(&color)
            .unwrap()
            .get(square.to_index())
            .unwrap());
        assert!(!board.all_pieces.get(square.to_index()).unwrap());
        assert!(!board
            .occupied_colored_squares
            .contains(&square.color().unwrap()));
    }
}
