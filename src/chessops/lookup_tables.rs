use std::collections::HashMap;

use crate::chessops::{Bitboard, Color, File, Quadrant, Rank, Square};

#[derive(Debug, PartialEq)]
pub struct LookupTables {
    pub clear_file: Vec<Bitboard>,
    pub clear_rank: Vec<Bitboard>,
    pub mask_file: Vec<Bitboard>,
    pub mask_rank: Vec<Bitboard>,
    pub mask_quadrant: Vec<Bitboard>,
    pub clear_colored_squares: HashMap<Color, Bitboard>,
}

impl LookupTables {
    pub fn new() -> Self {
        let mut clear_file = Vec::new();
        for file in File::iter() {
            let bitboard = Bitboard::new_clear_file(file.clone());
            clear_file.push(bitboard);
        }

        let mut clear_rank = Vec::new();
        for rank in Rank::iter() {
            let bitboard = Bitboard::new_clear_rank(rank.clone());
            clear_rank.push(bitboard);
        }

        let mut mask_file = Vec::new();
        for file in File::iter() {
            let bitboard = Bitboard::new_mask_file(file.clone());
            mask_file.push(bitboard);
        }

        let mut mask_rank = Vec::new();
        for rank in Rank::iter() {
            let bitboard = Bitboard::new_mask_rank(rank.clone());
            mask_rank.push(bitboard);
        }

        let mut mask_quadrant = Vec::new();
        for quadrant in Quadrant::iter() {
            let bitboard = Bitboard::new_mask_quadrant(quadrant.clone());
            mask_quadrant.push(bitboard);
        }

        let clear_colored_squares = LookupTables::build_clear_colored_squares();

        Self {
            clear_file: clear_file,
            clear_rank: clear_rank,
            mask_file: mask_file,
            mask_rank: mask_rank,
            mask_quadrant: mask_quadrant,
            clear_colored_squares,
        }
    }

    fn build_clear_colored_squares() -> HashMap<Color, Bitboard> {
        let mut map = HashMap::new();

        for color in Color::all() {
            let mut bitboard = Bitboard::new_full();

            match color {
                Color::Navy => {
                    bitboard.set(Square::E5.to_index(), false);
                    bitboard.set(Square::L12.to_index(), false);
                }
                Color::Red => {
                    bitboard.set(Square::L5.to_index(), false);
                    bitboard.set(Square::E12.to_index(), false);
                }
                Color::Green => {
                    bitboard.set(Square::F6.to_index(), false);
                    bitboard.set(Square::K11.to_index(), false);
                }
                Color::Violet => {
                    bitboard.set(Square::H6.to_index(), false);
                    bitboard.set(Square::I11.to_index(), false);
                }
                Color::Pink => {
                    bitboard.set(Square::I6.to_index(), false);
                    bitboard.set(Square::H11.to_index(), false);
                }
                Color::Yellow => {
                    bitboard.set(Square::K6.to_index(), false);
                    bitboard.set(Square::F11.to_index(), false);
                }
                Color::Ash => {
                    bitboard.set(Square::G7.to_index(), false);
                    bitboard.set(Square::J10.to_index(), false);
                }
                Color::Slate => {
                    bitboard.set(Square::J7.to_index(), false);
                    bitboard.set(Square::G10.to_index(), false);
                }
                Color::Cyan => {
                    bitboard.set(Square::F8.to_index(), false);
                    bitboard.set(Square::K9.to_index(), false);
                }
                Color::Black => {
                    bitboard.set(Square::H8.to_index(), false);
                    bitboard.set(Square::I9.to_index(), false);
                }
                Color::White => {
                    bitboard.set(Square::I8.to_index(), false);
                    bitboard.set(Square::H9.to_index(), false);
                }
                Color::Orange => {
                    bitboard.set(Square::K8.to_index(), false);
                    bitboard.set(Square::F9.to_index(), false);
                }
            }
            map.insert(color, bitboard);
        }

        map
    }
}
