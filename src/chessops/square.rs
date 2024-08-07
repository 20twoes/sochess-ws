use crate::chessops::{Color, File, Rank, BOARD_SIZE, BOARD_WIDTH};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u16)]
pub enum Square {
    A1 = 0,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,
    H1,
    I1,
    J1,
    K1,
    L1,
    M1,
    N1,
    O1,
    P1,
    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,
    H2,
    I2,
    J2,
    K2,
    L2,
    M2,
    N2,
    O2,
    P2,
    A3,
    B3,
    C3,
    D3,
    E3,
    F3,
    G3,
    H3,
    I3,
    J3,
    K3,
    L3,
    M3,
    N3,
    O3,
    P3,
    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,
    H4,
    I4,
    J4,
    K4,
    L4,
    M4,
    N4,
    O4,
    P4,
    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,
    I5,
    J5,
    K5,
    L5,
    M5,
    N5,
    O5,
    P5,
    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,
    H6,
    I6,
    J6,
    K6,
    L6,
    M6,
    N6,
    O6,
    P6,
    A7,
    B7,
    C7,
    D7,
    E7,
    F7,
    G7,
    H7,
    I7,
    J7,
    K7,
    L7,
    M7,
    N7,
    O7,
    P7,
    A8,
    B8,
    C8,
    D8,
    E8,
    F8,
    G8,
    H8,
    I8,
    J8,
    K8,
    L8,
    M8,
    N8,
    O8,
    P8,
    A9,
    B9,
    C9,
    D9,
    E9,
    F9,
    G9,
    H9,
    I9,
    J9,
    K9,
    L9,
    M9,
    N9,
    O9,
    P9,
    A10,
    B10,
    C10,
    D10,
    E10,
    F10,
    G10,
    H10,
    I10,
    J10,
    K10,
    L10,
    M10,
    N10,
    O10,
    P10,
    A11,
    B11,
    C11,
    D11,
    E11,
    F11,
    G11,
    H11,
    I11,
    J11,
    K11,
    L11,
    M11,
    N11,
    O11,
    P11,
    A12,
    B12,
    C12,
    D12,
    E12,
    F12,
    G12,
    H12,
    I12,
    J12,
    K12,
    L12,
    M12,
    N12,
    O12,
    P12,
    A13,
    B13,
    C13,
    D13,
    E13,
    F13,
    G13,
    H13,
    I13,
    J13,
    K13,
    L13,
    M13,
    N13,
    O13,
    P13,
    A14,
    B14,
    C14,
    D14,
    E14,
    F14,
    G14,
    H14,
    I14,
    J14,
    K14,
    L14,
    M14,
    N14,
    O14,
    P14,
    A15,
    B15,
    C15,
    D15,
    E15,
    F15,
    G15,
    H15,
    I15,
    J15,
    K15,
    L15,
    M15,
    N15,
    O15,
    P15,
    A16,
    B16,
    C16,
    D16,
    E16,
    F16,
    G16,
    H16,
    I16,
    J16,
    K16,
    L16,
    M16,
    N16,
    O16,
    P16,
}

impl Square {
    fn calc_index(file_index: usize, rank_index: usize) -> usize {
        (rank_index * BOARD_WIDTH) + file_index
    }

    // e.g. get Square::A1 from "A01"
    pub fn from_str(s: &str) -> Self {
        assert_eq!(s.len(), 3);

        let file_id = &s[0..1];
        let rank_id = &s[1..3];
        let file_index = File::str_to_index(file_id);
        let rank_index = Rank::str_to_index(rank_id);
        let index = Square::calc_index(file_index, rank_index);

        ALL_SQUARES[index].clone()
    }

    pub fn from_file_and_rank_index(file: usize, rank: usize) -> Self {
        let index = Square::calc_index(file, rank);
        ALL_SQUARES[index].clone()
    }

    pub fn from_index(i: usize) -> Self {
        ALL_SQUARES[i].clone()
    }

    pub fn to_index(&self) -> usize {
        *self as usize
    }

    pub fn file(&self) -> File {
        let i = self.clone() as usize;
        File::from_index(i % BOARD_WIDTH)
    }

    //pub fn rank(&self) -> Rank {
    //    let i = self.clone() as usize;
    //    Rank::from_index(i / BOARD_WIDTH)
    //}

    pub fn color(&self) -> Option<Color> {
        COLORED_SQUARES[self.to_index()]
    }
}

const fn init_colored_squares() -> [Option<Color>; BOARD_SIZE] {
    let mut arr = [None; BOARD_SIZE];

    arr[Square::E5 as usize] = Some(Color::Navy);
    arr[Square::L12 as usize] = Some(Color::Navy);
    arr[Square::L5 as usize] = Some(Color::Red);
    arr[Square::E12 as usize] = Some(Color::Red);
    arr[Square::F6 as usize] = Some(Color::Green);
    arr[Square::K11 as usize] = Some(Color::Green);
    arr[Square::H6 as usize] = Some(Color::Violet);
    arr[Square::I11 as usize] = Some(Color::Violet);
    arr[Square::I6 as usize] = Some(Color::Pink);
    arr[Square::H11 as usize] = Some(Color::Pink);
    arr[Square::K6 as usize] = Some(Color::Yellow);
    arr[Square::F11 as usize] = Some(Color::Yellow);
    arr[Square::G7 as usize] = Some(Color::Ash);
    arr[Square::J10 as usize] = Some(Color::Ash);
    arr[Square::J7 as usize] = Some(Color::Slate);
    arr[Square::G10 as usize] = Some(Color::Slate);
    arr[Square::F8 as usize] = Some(Color::Cyan);
    arr[Square::K9 as usize] = Some(Color::Cyan);
    arr[Square::H8 as usize] = Some(Color::Black);
    arr[Square::I9 as usize] = Some(Color::Black);
    arr[Square::I8 as usize] = Some(Color::White);
    arr[Square::H9 as usize] = Some(Color::White);
    arr[Square::K8 as usize] = Some(Color::Orange);
    arr[Square::F9 as usize] = Some(Color::Orange);

    arr
}

const COLORED_SQUARES: [Option<Color>; BOARD_SIZE] = init_colored_squares();

const ALL_SQUARES: [Square; BOARD_SIZE] = [
    Square::A1,
    Square::B1,
    Square::C1,
    Square::D1,
    Square::E1,
    Square::F1,
    Square::G1,
    Square::H1,
    Square::I1,
    Square::J1,
    Square::K1,
    Square::L1,
    Square::M1,
    Square::N1,
    Square::O1,
    Square::P1,
    Square::A2,
    Square::B2,
    Square::C2,
    Square::D2,
    Square::E2,
    Square::F2,
    Square::G2,
    Square::H2,
    Square::I2,
    Square::J2,
    Square::K2,
    Square::L2,
    Square::M2,
    Square::N2,
    Square::O2,
    Square::P2,
    Square::A3,
    Square::B3,
    Square::C3,
    Square::D3,
    Square::E3,
    Square::F3,
    Square::G3,
    Square::H3,
    Square::I3,
    Square::J3,
    Square::K3,
    Square::L3,
    Square::M3,
    Square::N3,
    Square::O3,
    Square::P3,
    Square::A4,
    Square::B4,
    Square::C4,
    Square::D4,
    Square::E4,
    Square::F4,
    Square::G4,
    Square::H4,
    Square::I4,
    Square::J4,
    Square::K4,
    Square::L4,
    Square::M4,
    Square::N4,
    Square::O4,
    Square::P4,
    Square::A5,
    Square::B5,
    Square::C5,
    Square::D5,
    Square::E5,
    Square::F5,
    Square::G5,
    Square::H5,
    Square::I5,
    Square::J5,
    Square::K5,
    Square::L5,
    Square::M5,
    Square::N5,
    Square::O5,
    Square::P5,
    Square::A6,
    Square::B6,
    Square::C6,
    Square::D6,
    Square::E6,
    Square::F6,
    Square::G6,
    Square::H6,
    Square::I6,
    Square::J6,
    Square::K6,
    Square::L6,
    Square::M6,
    Square::N6,
    Square::O6,
    Square::P6,
    Square::A7,
    Square::B7,
    Square::C7,
    Square::D7,
    Square::E7,
    Square::F7,
    Square::G7,
    Square::H7,
    Square::I7,
    Square::J7,
    Square::K7,
    Square::L7,
    Square::M7,
    Square::N7,
    Square::O7,
    Square::P7,
    Square::A8,
    Square::B8,
    Square::C8,
    Square::D8,
    Square::E8,
    Square::F8,
    Square::G8,
    Square::H8,
    Square::I8,
    Square::J8,
    Square::K8,
    Square::L8,
    Square::M8,
    Square::N8,
    Square::O8,
    Square::P8,
    Square::A9,
    Square::B9,
    Square::C9,
    Square::D9,
    Square::E9,
    Square::F9,
    Square::G9,
    Square::H9,
    Square::I9,
    Square::J9,
    Square::K9,
    Square::L9,
    Square::M9,
    Square::N9,
    Square::O9,
    Square::P9,
    Square::A10,
    Square::B10,
    Square::C10,
    Square::D10,
    Square::E10,
    Square::F10,
    Square::G10,
    Square::H10,
    Square::I10,
    Square::J10,
    Square::K10,
    Square::L10,
    Square::M10,
    Square::N10,
    Square::O10,
    Square::P10,
    Square::A11,
    Square::B11,
    Square::C11,
    Square::D11,
    Square::E11,
    Square::F11,
    Square::G11,
    Square::H11,
    Square::I11,
    Square::J11,
    Square::K11,
    Square::L11,
    Square::M11,
    Square::N11,
    Square::O11,
    Square::P11,
    Square::A12,
    Square::B12,
    Square::C12,
    Square::D12,
    Square::E12,
    Square::F12,
    Square::G12,
    Square::H12,
    Square::I12,
    Square::J12,
    Square::K12,
    Square::L12,
    Square::M12,
    Square::N12,
    Square::O12,
    Square::P12,
    Square::A13,
    Square::B13,
    Square::C13,
    Square::D13,
    Square::E13,
    Square::F13,
    Square::G13,
    Square::H13,
    Square::I13,
    Square::J13,
    Square::K13,
    Square::L13,
    Square::M13,
    Square::N13,
    Square::O13,
    Square::P13,
    Square::A14,
    Square::B14,
    Square::C14,
    Square::D14,
    Square::E14,
    Square::F14,
    Square::G14,
    Square::H14,
    Square::I14,
    Square::J14,
    Square::K14,
    Square::L14,
    Square::M14,
    Square::N14,
    Square::O14,
    Square::P14,
    Square::A15,
    Square::B15,
    Square::C15,
    Square::D15,
    Square::E15,
    Square::F15,
    Square::G15,
    Square::H15,
    Square::I15,
    Square::J15,
    Square::K15,
    Square::L15,
    Square::M15,
    Square::N15,
    Square::O15,
    Square::P15,
    Square::A16,
    Square::B16,
    Square::C16,
    Square::D16,
    Square::E16,
    Square::F16,
    Square::G16,
    Square::H16,
    Square::I16,
    Square::J16,
    Square::K16,
    Square::L16,
    Square::M16,
    Square::N16,
    Square::O16,
    Square::P16,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_works() {
        assert_eq!(Square::from_str("a01"), Square::A1);
        assert_eq!(Square::from_str("b02"), Square::B2);
        assert_eq!(Square::from_str("a04"), Square::A4);
        assert_eq!(Square::from_str("j10"), Square::J10);
        assert_eq!(Square::from_str("n16"), Square::N16);
        assert_eq!(Square::from_str("p16"), Square::P16);
    }

    #[test]
    fn square_file_works() {
        assert_eq!(Square::A1.file(), File::A);
        assert_eq!(Square::A2.file(), File::A);
        assert_eq!(Square::B2.file(), File::B);
        assert_eq!(Square::C3.file(), File::C);
        assert_eq!(Square::D4.file(), File::D);
        assert_eq!(Square::E5.file(), File::E);
        assert_eq!(Square::F6.file(), File::F);
        assert_eq!(Square::G7.file(), File::G);
        assert_eq!(Square::H8.file(), File::H);
        assert_eq!(Square::I9.file(), File::I);
        assert_eq!(Square::J10.file(), File::J);
        assert_eq!(Square::K11.file(), File::K);
        assert_eq!(Square::L12.file(), File::L);
        assert_eq!(Square::M13.file(), File::M);
        assert_eq!(Square::N14.file(), File::N);
        assert_eq!(Square::O15.file(), File::O);
        assert_eq!(Square::P16.file(), File::P);
    }

    #[test]
    fn square_color_works() {
        assert_eq!(Square::E5.color(), Some(Color::Navy));
    }
}
