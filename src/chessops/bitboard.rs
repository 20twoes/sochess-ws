use std::fmt;

use bit_vec::BitVec;

use crate::chessops::{File, Quadrant, Rank};

pub const BOARD_WIDTH: usize = 16;
pub const BOARD_SIZE: usize = BOARD_WIDTH * BOARD_WIDTH;

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

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();

        for (i, bit) in self.bv.iter().enumerate() {
            match bit {
                true => output.push('1'),
                false => output.push('0'),
            }
            output.push(' '); // Add some spacing
            if i != 0 && i % BOARD_WIDTH == (BOARD_WIDTH - 1) {
                output.push('\n');
            }
        }
        write!(f, "Bitboard:\n{}", output)
    }
}

impl Bitboard {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            bv: BitVec::from_bytes(bytes),
        }
    }

    pub fn new() -> Self {
        Self {
            bv: BitVec::from_elem(BOARD_SIZE, false),
        }
    }

    pub fn new_clear_file(file: File) -> Self {
        let mut bv = BitVec::from_elem(BOARD_SIZE, true);
        let mut index = file as usize;
        for _ in 0..BOARD_WIDTH {
            bv.set(index, false);
            index += BOARD_WIDTH;
        }

        Self { bv: bv }
    }

    pub fn new_clear_rank(rank: Rank) -> Self {
        let mut bv = BitVec::from_elem(BOARD_SIZE, true);
        let start_pos = BOARD_WIDTH * rank as usize;
        let end_pos = start_pos + BOARD_WIDTH;
        for i in start_pos..end_pos {
            bv.set(i, false);
        }

        Self { bv: bv }
    }

    pub fn new_mask_file(file: File) -> Self {
        let mut bv = BitVec::from_elem(BOARD_SIZE, false);
        let mut index = file as usize;
        for _ in 0..BOARD_WIDTH {
            bv.set(index, true);
            index += BOARD_WIDTH;
        }

        Self { bv: bv }
    }

    pub fn new_mask_rank(rank: Rank) -> Self {
        let mut bv = BitVec::from_elem(BOARD_SIZE, false);
        let start_pos = BOARD_WIDTH * rank as usize;
        let end_pos = start_pos + BOARD_WIDTH;
        for i in start_pos..end_pos {
            bv.set(i, true);
        }

        Self { bv: bv }
    }

    pub fn new_mask_quadrant(quadrant: Quadrant) -> Self {
        let bv = match quadrant {
            Quadrant::SW => BitVec::from_bytes(&[
                0b11111111, 0b00000000, 0b11111111, 0b00000000, 0b11111111, 0b00000000, 0b11111111,
                0b00000000, 0b11111111, 0b00000000, 0b11111111, 0b00000000, 0b11111111, 0b00000000,
                0b11111111, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
                0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
                0b00000000, 0b00000000, 0b00000000, 0b00000000,
            ]),
            Quadrant::SE => BitVec::from_bytes(&[
                0b00000000, 0b11111111, 0b00000000, 0b11111111, 0b00000000, 0b11111111, 0b00000000,
                0b11111111, 0b00000000, 0b11111111, 0b00000000, 0b11111111, 0b00000000, 0b11111111,
                0b00000000, 0b11111111, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
                0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
                0b00000000, 0b00000000, 0b00000000, 0b00000000,
            ]),
            Quadrant::NW => BitVec::from_bytes(&[
                0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
                0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
                0b00000000, 0b00000000, 0b11111111, 0b00000000, 0b11111111, 0b00000000, 0b11111111,
                0b00000000, 0b11111111, 0b00000000, 0b11111111, 0b00000000, 0b11111111, 0b00000000,
                0b11111111, 0b00000000, 0b11111111, 0b00000000,
            ]),
            Quadrant::NE => BitVec::from_bytes(&[
                0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
                0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
                0b00000000, 0b00000000, 0b00000000, 0b11111111, 0b00000000, 0b11111111, 0b00000000,
                0b11111111, 0b00000000, 0b11111111, 0b00000000, 0b11111111, 0b00000000, 0b11111111,
                0b00000000, 0b11111111, 0b00000000, 0b11111111,
            ]),
        };

        Self { bv: bv }
    }

    pub fn get(&self, i: usize) -> Option<bool> {
        self.bv.get(i)
    }

    pub fn len(&self) -> usize {
        self.bv.len()
    }

    pub fn set(&mut self, i: usize, val: bool) {
        self.bv.set(i, val);
    }

    /// Bit shift towards least significant bit (lower index).
    /// Returning a bool to keep signature the same as `and` `or` operations
    pub fn shift_left(&mut self, by: usize) -> bool {
        let mut bv = BitVec::from_elem(BOARD_SIZE, false);
        let mut count = 0;

        for i in self.bv.iter().skip(by) {
            bv.set(count, i);
            count += 1;
        }

        self.bv = bv;
        true
    }

    /// Bit shift towards most significant bit (higher index).
    /// Returning a bool to keep signature the same as `and` `or` operations
    pub fn shift_right(&mut self, by: usize) -> bool {
        let mut bv = BitVec::with_capacity(BOARD_SIZE);
        let mut iter = self.bv.iter();

        for i in 0..BOARD_SIZE {
            if i < by {
                bv.push(false);
                continue;
            }

            bv.push(
                iter.next()
                    .expect("Invalid value found during shift_right")
                    .clone(),
            );
        }

        self.bv = bv;
        true
    }

    pub fn or(&mut self, other: &Self) -> bool {
        self.bv.or(&other.bv)
    }

    pub fn and(&mut self, other: &Self) -> bool {
        self.bv.and(&other.bv)
    }

    pub fn not(&mut self) {
        self.bv.negate();
    }

    pub fn least_significant_bit(&self) -> Option<usize> {
        for (i, bit) in self.bv.iter().enumerate() {
            if bit {
                return Some(i);
            }
        }
        None
    }

    pub fn any(&self) -> bool {
        self.bv.any()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::chessops::Square;

    #[test]
    fn shift_right_works() {
        #[rustfmt::skip]
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
        #[rustfmt::skip]
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

        king_loc.shift_right(BOARD_WIDTH + 1);

        assert_eq!(king_loc, expected);
        assert_eq!(king_loc.len(), BOARD_SIZE);
    }

    #[test]
    fn shift_left_works() {
        #[rustfmt::skip]
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
        #[rustfmt::skip]
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

        king_loc.shift_left(BOARD_WIDTH + 1);

        assert_eq!(king_loc, expected);
        assert_eq!(king_loc.len(), BOARD_SIZE);
    }

    #[test]
    fn new_clear_file_works() {
        let bitboard = Bitboard::new_clear_file(File::A);
        assert!(!bitboard.get(Square::A1 as usize).unwrap());
        assert!(!bitboard.get(Square::A2 as usize).unwrap());
        assert!(!bitboard.get(Square::A16 as usize).unwrap());
        assert!(bitboard.get(Square::B1 as usize).unwrap());
        assert!(bitboard.get(Square::P16 as usize).unwrap());

        let bitboard = Bitboard::new_clear_file(File::I);
        assert!(bitboard.get(Square::A1 as usize).unwrap());
        assert!(bitboard.get(Square::P16 as usize).unwrap());
        assert!(!bitboard.get(Square::I1 as usize).unwrap());
        assert!(!bitboard.get(Square::I9 as usize).unwrap());
        assert!(!bitboard.get(Square::I16 as usize).unwrap());
    }

    #[test]
    fn new_mask_rank_works() {
        #[rustfmt::skip]
        let expected = Bitboard::from_bytes(&[
            0b11111111, 0b11111111,
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

        let bitboard = Bitboard::new_mask_rank(Rank::R1);
        assert_eq!(bitboard, expected);

        #[rustfmt::skip]
        let expected = Bitboard::from_bytes(&[
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
            0b11111111, 0b11111111,
        ]);
        let bitboard = Bitboard::new_mask_rank(Rank::R16);
        assert_eq!(bitboard, expected);
    }
}
