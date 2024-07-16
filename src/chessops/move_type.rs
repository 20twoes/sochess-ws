use crate::chessops::{Color, Role};

#[derive(Debug, PartialEq)]
pub struct Move {
    pub color: Color,
    pub role: Role,
    //from: String,
    //to: String,
}

impl Move {
    pub fn from_san(san: &str) -> Self {
        assert_eq!(san.len(), 6);
        let mut chars = san.chars();

        Self {
            color: Color::from_char(chars.nth(0).unwrap()).expect("Invalid move color"),
            role: Role::from_char(chars.nth(0).unwrap()).expect("Invalid move role"),
            //from: String::new(),
            //to: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_san_works() {
        assert_eq!(
            Move::from_san("WNb1c3"),
            Move {
                color: Color::White,
                role: Role::Knight,
            },
        );
    }
}
