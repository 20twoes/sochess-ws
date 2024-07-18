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
            'a' => Some(Self::Ash),
            'b' => Some(Self::Black),
            'c' => Some(Self::Cyan),
            'g' => Some(Self::Green),
            'n' => Some(Self::Navy),
            'o' => Some(Self::Orange),
            'p' => Some(Self::Pink),
            'r' => Some(Self::Red),
            's' => Some(Self::Slate),
            'v' => Some(Self::Violet),
            'w' => Some(Self::White),
            'y' => Some(Self::Yellow),
            _ => None,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::Ash => 'a',
            Self::Black => 'b',
            Self::Cyan => 'c',
            Self::Green => 'g',
            Self::Navy => 'n',
            Self::Orange => 'o',
            Self::Pink => 'p',
            Self::Red => 'r',
            Self::Slate => 's',
            Self::Violet => 'v',
            Self::White => 'w',
            Self::Yellow => 'y',
        }
    }

    pub fn all() -> [Color; 12] {
        [
            Color::Ash,
            Color::Black,
            Color::Cyan,
            Color::Green,
            Color::Navy,
            Color::Orange,
            Color::Pink,
            Color::Red,
            Color::Slate,
            Color::Violet,
            Color::White,
            Color::Yellow,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_char_works() {
        let color = Color::Ash;
        assert_eq!(color.to_char(), 'a');

        let color = Color::White;
        assert_eq!(color.to_char(), 'w');
    }
}
