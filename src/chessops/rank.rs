#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Rank {
    R1 = 0,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    R16,
}

impl Rank {
    //fn from_str(s: &str) -> Self {
    //    let i = Rank::str_to_index(s);
    //    ALL_RANKS[i].clone()
    //}

    pub fn str_to_index(s: &str) -> usize {
        RANK_IDS.iter().position(|&x| x == s).expect("Invalid rank")
    }

    //pub fn from_index(i: usize) -> Self {
    //    ALL_RANKS[i].clone()
    //}

    pub fn iter() -> impl Iterator<Item = &'static Rank> {
        ALL_RANKS.iter()
    }

    pub fn to_index(&self) -> usize {
        *self as usize
    }
}

const ALL_RANKS: [Rank; 16] = [
    Rank::R1,
    Rank::R2,
    Rank::R3,
    Rank::R4,
    Rank::R5,
    Rank::R6,
    Rank::R7,
    Rank::R8,
    Rank::R9,
    Rank::R10,
    Rank::R11,
    Rank::R12,
    Rank::R13,
    Rank::R14,
    Rank::R15,
    Rank::R16,
];

const RANK_IDS: [&str; 16] = [
    "01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11", "12", "13", "14", "15", "16",
];
