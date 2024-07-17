use crate::chessops::{Color, Role};

#[derive(Debug, PartialEq)]
pub struct Piece {
    pub color: Color,
    pub role: Role,
}

impl Piece {
    pub fn to_string(&self) -> String {
        format!("{}{}", self.color.to_char(), self.role.to_char())
    }
}
