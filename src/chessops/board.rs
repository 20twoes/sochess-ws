use std::collections::HashMap;

use crate::chessops::{Piece, Square};

#[derive(Debug, PartialEq)]
pub struct Board {
    pub pieces: HashMap<Square, Piece>,
}

impl Board {
    //pub fn from_str(fen: &str) -> Self {
    //    Board {
    //        pieces: HashMap::new(),
    //    }
    //}
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn from_str_works() {
//        let fen = "aqabvrvnbrbnbbbqbkbbbnbrynyrsbsq/aranvpvpbpbpbpbpbpbpbpbpypypsnsr/nbnp12opob/nqnp12opoq/crcp12rprr/cncp12rprn/gbgp12pppb/gqgp12pppq/yqyp12vpvq/ybyp12vpvb/onop12npnn/orop12npnr/rqrp12cpcq/rbrp12cpcb/srsnppppwpwpwpwpwpwpwpwpgpgpanar/sqsbprpnwrwnwbwqwkwbwnwrgngrabaq";
//        let board = Board::from_str(fen);
//    }
//}
