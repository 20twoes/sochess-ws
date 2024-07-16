#[derive(Debug, PartialEq)]
pub enum Player {
    P1,
    P2,
}

impl Player {
    pub fn from_char(ch: char) -> Option<Self> {
        match ch {
            '1' => Some(Self::P1),
            '2' => Some(Self::P2),
            _ => None,
        }
    }

    pub fn to_int(&self) -> u8 {
        match self {
            Player::P1 => 1,
            Player::P2 => 2,
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Player::P1 => Player::P2,
            Player::P2 => Player::P1,
        }
    }
}
