use crate::chessops::{Color, Piece, Role, Square};

#[derive(Debug, PartialEq)]
pub struct Move {
    pub color: Color,
    pub role: Role,
    pub from: Square,
    pub to: Square,
}

impl Move {
    pub fn from_san(san: &str) -> Self {
        assert_eq!(san.len(), 8);
        let lcase_san = san.to_lowercase();

        Self {
            color: Color::from_char(lcase_san[0..1].chars().nth(0).unwrap())
                .expect("Invalid move color"),
            role: Role::from_char(lcase_san[1..2].chars().nth(0).unwrap())
                .expect("Invalid move role"),
            from: Square::from_str(&lcase_san[2..5]),
            to: Square::from_str(&lcase_san[5..8]),
        }
    }

    pub fn to_piece(&self) -> Piece {
        Piece {
            color: self.color.clone(),
            role: self.role.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_san_works() {
        assert_eq!(
            Move::from_san("WNb01c03"),
            Move {
                color: Color::White,
                role: Role::Knight,
                from: Square::B1,
                to: Square::C3,
            },
        );
    }
}
