#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum File {
    A = 0,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
}

impl File {
    //fn from_str(s: &str) -> Self {
    //    let i = File::str_to_index(s);
    //    ALL_FILES[i].clone()
    //}

    pub fn str_to_index(s: &str) -> usize {
        FILE_IDS.iter().position(|&x| x == s).expect("Invalid file")
    }

    pub fn from_index(i: usize) -> Self {
        ALL_FILES[i].clone()
    }

    pub fn to_index(&self) -> usize {
        *self as usize
    }

    pub fn iter() -> impl Iterator<Item = &'static File> {
        ALL_FILES.iter()
    }

    pub fn last() -> Self {
        File::P
    }
}

const ALL_FILES: [File; 16] = [
    File::A,
    File::B,
    File::C,
    File::D,
    File::E,
    File::F,
    File::G,
    File::H,
    File::I,
    File::J,
    File::K,
    File::L,
    File::M,
    File::N,
    File::O,
    File::P,
];

const FILE_IDS: [&str; 16] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p",
];
