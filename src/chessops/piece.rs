use crate::chessops::{Color, Role};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Piece {
    pub color: Color,
    pub role: Role,
}

impl Piece {
    pub fn new(color: Color, role: Role) -> Self {
        Self {
            color: color,
            role: role,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}{}", self.color.to_char(), self.role.to_char())
    }
}
