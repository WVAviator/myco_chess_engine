use core::fmt;
use std::ops::{BitAnd, BitOr, Not, Shl, Shr};

#[derive(Clone, PartialEq, Copy, Eq)]
pub struct Bitboard(u64);

impl Bitboard {
    pub const FIRST_RANK: Bitboard = Bitboard(0xff);
    pub const SECOND_RANK: Bitboard = Bitboard(0xff00);
    pub const SEVENTH_RANK: Bitboard = Bitboard(0xff000000000000);
    pub const EIGTH_RANK: Bitboard = Bitboard(0xff00000000000000);
    pub const A_FILE: Bitboard = Bitboard(0x101010101010101);
    pub const H_FILE: Bitboard = Bitboard(0x8080808080808080);

    pub fn from_fen(fen_board_str: &str, piece_char: char) -> Self {
        let mut bitboard = 0u64;

        let mut rank = 7;
        let mut file = 0;

        for c in fen_board_str.chars() {
            match c {
                ch if ch == piece_char => {
                    let square_index = rank * 8 + file;
                    bitboard |= 1 << square_index;
                    file += 1;
                }
                '1'..='8' => {
                    file += c.to_digit(10).unwrap() as usize;
                }
                '/' => {
                    rank -= 1;
                    file = 0;
                }
                _ => file += 1,
            }
        }

        Self(bitboard)
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl<'a, 'b> BitAnd<&'b Bitboard> for &'a Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: &Bitboard) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl<'a, 'b> BitOr<&'b Bitboard> for &'a Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: &Bitboard) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl<'a> Not for &'a Bitboard {
    type Output = Bitboard;

    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl Shl<u32> for Bitboard {
    type Output = Self;

    fn shl(self, rhs: u32) -> Self::Output {
        Bitboard(self.0 << rhs)
    }
}

impl<'a> Shl<u32> for &'a Bitboard {
    type Output = Bitboard;

    fn shl(self, rhs: u32) -> Self::Output {
        Bitboard(self.0 << rhs)
    }
}

impl Shr<u32> for Bitboard {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self::Output {
        Bitboard(self.0 >> rhs)
    }
}

impl<'a> Shr<u32> for &'a Bitboard {
    type Output = Bitboard;

    fn shr(self, rhs: u32) -> Self::Output {
        Bitboard(self.0 >> rhs)
    }
}

impl fmt::Debug for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:064b}", self.0)
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let index = rank * 8 + file;
                if (self.0 & (1 << index)) != 0 {
                    write!(f, "1 ")?;
                } else {
                    write!(f, ". ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl From<u64> for Bitboard {
    fn from(bits: u64) -> Self {
        Bitboard(bits)
    }
}

impl Into<u64> for Bitboard {
    fn into(self) -> u64 {
        self.0
    }
}

impl IntoIterator for Bitboard {
    type Item = u32;
    type IntoIter = BitboardIterator;

    fn into_iter(self) -> Self::IntoIter {
        BitboardIterator { bitboard: self.0 }
    }
}

pub struct BitboardIterator {
    bitboard: u64,
}

impl Iterator for BitboardIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.bitboard {
            0 => None,
            _ => {
                let lsb = self.bitboard & (!self.bitboard + 1);
                let index = lsb.trailing_zeros();
                self.bitboard &= self.bitboard - 1;
                Some(index)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iterates_through_each_bit() {
        let bb = Bitboard(42); // Bits 2, 4, and 6 set
        let mut bb_iter = bb.into_iter();

        assert_eq!(bb_iter.next(), Some(1));
        assert_eq!(bb_iter.next(), Some(3));
        assert_eq!(bb_iter.next(), Some(5));
        assert_eq!(bb_iter.next(), None);
    }

    #[test]
    fn implements_bit_and() {
        let bb1 = Bitboard(42);
        let bb2 = Bitboard(32);
        assert_eq!(bb1 & bb2, Bitboard(32));
    }

    #[test]
    fn implements_bit_or() {
        let bb1 = Bitboard(32);
        let bb2 = Bitboard(10);
        assert_eq!(bb1 | bb2, Bitboard(42));
    }

    #[test]
    fn implements_shl() {
        let bb1 = Bitboard(2);
        assert_eq!(bb1 << 1, Bitboard(4));
    }

    #[test]
    fn implements_shr() {
        let bb1 = Bitboard(12);
        assert_eq!(bb1 >> 1, Bitboard(6));
    }

    #[test]
    fn parses_fen() {
        let fen_board_str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let bb = Bitboard::from_fen(fen_board_str, 'P');
        assert_eq!(bb.0, 0xff00);
    }
}
