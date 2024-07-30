use crate::chessops::{Bitboard, File, Quadrant, Rank};

#[derive(Debug, PartialEq)]
pub struct LookupTables {
    pub clear_file: Vec<Bitboard>,
    pub clear_rank: Vec<Bitboard>,
    pub mask_file: Vec<Bitboard>,
    pub mask_rank: Vec<Bitboard>,
    pub mask_quadrant: Vec<Bitboard>,
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

        Self {
            clear_file: clear_file,
            clear_rank: clear_rank,
            mask_file: mask_file,
            mask_rank: mask_rank,
            mask_quadrant: mask_quadrant,
        }
    }
}
