use bit_vec::BitVec;

pub const BOARD_WIDTH: usize = 16;
const BOARD_SIZE: usize = BOARD_WIDTH * BOARD_WIDTH;

/**
 * Bitboard for 256 square Sovereign Chess board.
 * The zero index represents Square a1.
 * The 255th index represents Square p16.
 * Because of BitVec, if we convert to bytes, the zero index is the
 * most significant bit (on the left side).
 */
#[derive(Clone, Debug, PartialEq)]
pub struct Bitboard {
    bv: BitVec,
}

impl Bitboard {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            bv: BitVec::from_bytes(bytes),
        }
    }

    pub fn len(&self) -> usize {
        self.bv.len()
    }

    /// Bit shift towards least significant bit (lower index).
    /// Returns a new Bitboard object.
    pub fn shift_left(&self, by: usize) -> Self {
        let mut bv = BitVec::from_elem(BOARD_SIZE, false);
        let mut count = 0;

        for i in self.bv.iter().skip(by) {
            bv.set(count, i);
            count += 1;
        }

        Self {
            bv: bv,
        }
    }

    /// Bit shift towards most significant bit (higher index).
    /// Returns a new Bitboard object.
    pub fn shift_right(&self, by: usize) -> Self {
        let mut bv = BitVec::with_capacity(BOARD_SIZE);
        let mut iter = self.bv.iter();

        for i in 0..BOARD_SIZE {
            if i < by {
                bv.push(false);
                continue;
            }

            bv.push(iter.next().expect("Invalid value found during shift_right").clone());
        }

        Self {
            bv: bv,
        }
    }

    pub fn or(&mut self, other: &Self) -> bool {
        self.bv.or(&other.bv)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shift_right_works() {
        let mut king_loc = Bitboard::from_bytes(&[
            0b00000000, 0b00000000,
            0b01000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
        ]);
        let expected = Bitboard::from_bytes(&[
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00100000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
        ]);

        let result = king_loc.shift_right(BOARD_WIDTH + 1);

        assert_eq!(result, expected);
        assert_eq!(result.len(), BOARD_SIZE);
    }

    #[test]
    fn shift_left_works() {
        let mut king_loc = Bitboard::from_bytes(&[
            0b00000000, 0b00000000,
            0b01000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
        ]);
        let expected = Bitboard::from_bytes(&[
            0b10000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
            0b00000000, 0b00000000,
        ]);

        let result = king_loc.shift_left(BOARD_WIDTH + 1);

        assert_eq!(result, expected);
        assert_eq!(result.len(), BOARD_SIZE);
    }
}
