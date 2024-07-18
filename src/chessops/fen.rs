use std::collections::{HashMap, HashSet};

use crate::chessops::{Board, Color, Piece, Player, Role, Square, BOARD_SIZE, BOARD_WIDTH};

/**
 * Forsyth-Edwards Notation - notates the resulting board position
 * Fields:
 * - Piece placement.  Each piece is represented by two characters.  Color and Role
 * - Active player: 1 or 0
 * - Player 1's owned army.  e.g. `W` for White army
 * - Player 1's controlled armies.  e.g. `GY` for Green and Yellow armies
 * - Player 2's owned army
 * - Player 2's controlled armies
 * - Ply or halfmove number.  Starts at 1 after first move.
 */
pub struct Fen {}

impl Fen {
    pub fn parse(
        fen: &str,
    ) -> (
        Board,
        Player,
        Option<Color>,
        HashSet<Color>,
        Option<Color>,
        HashSet<Color>,
        u32,
    ) {
        let parts: Vec<&str> = fen.split(' ').collect();
        assert_eq!(parts.len(), 7);
        assert_eq!(parts[1].len(), 1);
        assert_eq!(parts[2].len(), 1);
        assert_eq!(parts[4].len(), 1);

        let active_player = Player::from_char(parts[1].chars().nth(0).unwrap());
        assert!(active_player.is_some());

        let mut p1_controlled = HashSet::new();
        for ch in parts[3].chars() {
            if let Some(c) = Color::from_char(ch) {
                p1_controlled.insert(c);
            }
        }

        let mut p2_controlled = HashSet::new();
        for ch in parts[5].chars() {
            if let Some(c) = Color::from_char(ch) {
                p2_controlled.insert(c);
            }
        }

        (
            // Board
            Fen::to_board(parts[0]),
            // Active player
            active_player.unwrap(),
            // P1 owned color
            Color::from_char(parts[2].chars().nth(0).unwrap()),
            // P1 controlled colors
            p1_controlled,
            // P2 owned color
            Color::from_char(parts[4].chars().nth(0).unwrap()),
            // P2 controlled colors
            p2_controlled,
            // Ply
            parts[6].parse::<u32>().expect("`ply` is invalid"),
        )
    }

    /// FEN order goes from top rank to bottom rank, while square order goes from
    /// bottom rank to top.  But in between ranks, we iterate through files the same order.
    fn fen_index_to_square(fen_index: usize) -> Square {
        let rank = if fen_index == 0 {
            BOARD_WIDTH - 1
        } else {
            (BOARD_WIDTH - (fen_index / BOARD_WIDTH)) - 1
        };
        let file = fen_index % BOARD_WIDTH;
        Square::from_file_and_rank_index(file, rank)
    }

    pub fn to_board(board_fen: &str) -> Board {
        let mut board = Board::new();

        // Split fen into ranks
        let ranks: Vec<&str> = board_fen.split("/").collect();
        assert_eq!(ranks.len(), BOARD_WIDTH);

        let mut index = 0;

        for rank in ranks {
            let mut iter = rank.chars();

            // Read two chars each loop iteration
            while let Some(color) = iter.nth(0) {
                let role = iter.nth(0).unwrap();

                if color.is_digit(10) { // base 10
                    // Skip this many spaces
                    let num_skipped: usize =
                        format!("{}{}", color, role).parse().expect("Invalid int");
                    index += num_skipped;
                } else {
                    let piece = Piece {
                        color: Color::from_char(color).expect("Invalid color"),
                        role: Role::from_char(role).expect("Invalid role"),
                    };
                    let square = Fen::fen_index_to_square(index);
                    board.insert_piece(square, piece);
                    index += 1;
                }
            }
        }

        board
    }

    pub fn from_board(board: &Board) -> String {
        let mut tokens = Vec::with_capacity(BOARD_SIZE);
        let mut empty_squares = 0;

        for i in 0..BOARD_SIZE {
            // If new rank, add a separator
            if i != 0 && i % BOARD_WIDTH == 0 {
                if empty_squares > 0 {
                    tokens.push(format!("{:02}", empty_squares));
                    empty_squares = 0;
                }
                tokens.push("/".to_string());
            }

            let square = Fen::fen_index_to_square(i);
            match board.by_square.get(&square) {
                Some(piece) => {
                    // Add skipped squares count
                    if empty_squares > 0 {
                        tokens.push(format!("{:02}", empty_squares));
                        empty_squares = 0;
                    }
                    tokens.push(piece.to_string());
                }
                None => {
                    empty_squares += 1;
                }
            }
        }

        if empty_squares > 0 {
            tokens.push(format!("{:02}", empty_squares));
        }

        tokens.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_board() {
        let board_fen = "aqabvrvnbrbnbbbqbkbbbnbrynyrsbsq/aranvpvpbpbpbpbpbpbpbpbpypypsnsr/nbnp12opob/nqnp12opoq/crcp12rprr/cncp12rprn/gbgp12pppb/gqgp12pppq/yqyp12vpvq/ybyp12vpvb/onop12npnn/orop12npnr/rqrp12cpcq/rbrp12cpcb/srsnppppwpwpwpwpwpwpwpwpgpgpanar/sqsbprpnwrwnwbwqwkwbwnwrgngrabaq";
        let board = Fen::to_board(board_fen);
        assert_eq!(
            *board.by_square.get(&Square::A16).unwrap(),
            Piece {
                color: Color::Ash,
                role: Role::Queen,
            },
        );
        assert_eq!(
            *board.by_square.get(&Square::C2).unwrap(),
            Piece {
                color: Color::Pink,
                role: Role::Pawn,
            },
        );
        assert_eq!(
            *board.by_square.get(&Square::P16).unwrap(),
            Piece {
                color: Color::Slate,
                role: Role::Queen,
            },
        );
        assert!(board.by_square.get(&Square::I8).is_none());
    }

    #[test]
    fn fen_index_to_square_works() {
        assert_eq!(Fen::fen_index_to_square(0), Square::A16);
        assert_eq!(Fen::fen_index_to_square(5), Square::F16);
        assert_eq!(Fen::fen_index_to_square(17), Square::B15);
        assert_eq!(Fen::fen_index_to_square(250), Square::K1);
        assert_eq!(Fen::fen_index_to_square(255), Square::P1);
    }

    #[test]
    fn from_board_works() {
        let pieces = HashMap::from([
            (Square::A1, Piece { color: Color::White, role: Role::King }),
        ]);
        let mut board = Board::new();
        board.by_square = pieces;
        assert_eq!(
            Fen::from_board(&board),
            "16/16/16/16/16/16/16/16/16/16/16/16/16/16/16/wk15".to_string(),
        );

        board.by_square = HashMap::from([
            (Square::A16, Piece { color: Color::White, role: Role::King }),
        ]);
        assert_eq!(
            Fen::from_board(&board),
            "wk15/16/16/16/16/16/16/16/16/16/16/16/16/16/16/16".to_string(),
        );

        board.by_square = HashMap::from([
            (Square::A16, Piece { color: Color::White, role: Role::King }),
            (Square::G13, Piece { color: Color::Black, role: Role::Pawn }),
        ]);
        assert_eq!(
            Fen::from_board(&board),
            "wk15/16/16/06bp09/16/16/16/16/16/16/16/16/16/16/16/16".to_string(),
        );
    }
}
