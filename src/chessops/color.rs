#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Color {
    Ash,
    Black,
    Cyan,
    Green,
    Navy,
    Orange,
    Pink,
    Red,
    Slate,
    Violet,
    White,
    Yellow,
}

impl Color {
    pub fn from_char(ch: char) -> Option<Self> {
        match ch {
            'A' => Some(Self::Ash),
            'B' => Some(Self::Black),
            'C' => Some(Self::Cyan),
            'G' => Some(Self::Green),
            'N' => Some(Self::Navy),
            'O' => Some(Self::Orange),
            'P' => Some(Self::Pink),
            'R' => Some(Self::Red),
            'S' => Some(Self::Slate),
            'V' => Some(Self::Violet),
            'W' => Some(Self::White),
            'Y' => Some(Self::Yellow),
            _ => None,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::Ash => 'A',
            Self::Black => 'B',
            Self::Cyan => 'C',
            Self::Green => 'G',
            Self::Navy => 'N',
            Self::Orange => 'O',
            Self::Pink => 'P',
            Self::Red => 'R',
            Self::Slate => 'S',
            Self::Violet => 'V',
            Self::White => 'W',
            Self::Yellow => 'Y',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_char_works() {
        let color = Color::Ash;
        assert_eq!(color.to_char(), 'A');

        let color = Color::White;
        assert_eq!(color.to_char(), 'W');
    }
}
