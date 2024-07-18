use crate::chessops::{Bitboard, File};

#[derive(Debug, PartialEq)]
pub struct LookupTables {
    pub clear_file: Vec<Bitboard>,
}

impl LookupTables {
    pub fn new() -> Self {
        let mut clear_file = Vec::new();
        for file in File::iter() {
            let bitboard = Bitboard::new_clear_file(file.clone());
            clear_file.push(bitboard);
        }

        Self {
            clear_file: clear_file,
        }
    }
}
