use crate::chessops::{Color, Role, Square};

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

        Self {
            color: Color::from_char(san[0..1].chars().nth(0).unwrap()).expect("Invalid move color"),
            role: Role::from_char(san[1..2].chars().nth(0).unwrap()).expect("Invalid move role"),
            from: Square::from_str(&san[2..5]),
            to: Square::from_str(&san[5..8]),
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
