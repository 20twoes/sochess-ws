use crate::chessops::{Color, Piece, Role, Square};

#[derive(Debug, PartialEq)]
pub struct Move {
    pub color: Color,
    pub role: Role,
    pub from: Square,
    pub to: Square,
    pub promotion: Option<Role>,
}

impl Move {
    #[cfg(test)]
    pub fn new(color: Color, role: Role, from: Square, to: Square) -> Self {
        Move {
            color: color,
            role: role,
            from: from,
            to: to,
            promotion: None,
        }
    }

    pub fn from_san(san: &str) -> Self {
        assert!(san.len() >= 8);
        assert!(san.len() <= 10);
        let lcase_san = san.to_lowercase();

        let mut move_ = Self {
            color: Color::from_char(lcase_san[0..1].chars().nth(0).unwrap())
                .expect("Invalid move color"),
            role: Role::from_char(lcase_san[1..2].chars().nth(0).unwrap())
                .expect("Invalid move role"),
            from: Square::from_str(&lcase_san[2..5]),
            to: Square::from_str(&lcase_san[5..8]),
            promotion: None,
        };

        if san.len() > 8 {
            let promotion = if lcase_san[8..9].chars().nth(0).unwrap() == '=' {
                Some(
                    Role::from_char(lcase_san[9..10].chars().nth(0).unwrap())
                        .expect("Invalid role"),
                )
            } else {
                None
            };
            move_.promotion = promotion;
        }

        move_
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
            Move::new(Color::White, Role::Knight, Square::B1, Square::C3),
        );
    }

    #[test]
    fn from_san_with_promotion_works() {
        assert_eq!(
            Move::from_san("WPi06i07=Q"),
            Move {
                color: Color::White,
                role: Role::Pawn,
                from: Square::I6,
                to: Square::I7,
                promotion: Some(Role::Queen),
            },
        );
    }
}
