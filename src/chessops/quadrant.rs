#[derive(Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Quadrant {
    SW = 0, // Southwest
    SE,     // Southeast
    NW,     // Northwest
    NE,     // Northeast
}

impl Quadrant {
    pub fn to_index(&self) -> usize {
        self.clone() as usize
    }

    pub fn iter() -> impl Iterator<Item = &'static Quadrant> {
        ALL_QUADRANTS.iter()
    }
}

const ALL_QUADRANTS: [Quadrant; 4] = [Quadrant::SW, Quadrant::SE, Quadrant::NW, Quadrant::NE];
