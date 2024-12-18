use std::sync::OnceLock;

use anyhow::bail;
use rand::random;

use super::game::Turn;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CastlingRights(u8);

impl CastlingRights {
    pub const WHITE_KINGSIDE: u8 = 1;
    pub const WHITE_QUEENSIDE: u8 = 2;
    pub const BLACK_KINGSIDE: u8 = 4;
    pub const BLACK_QUEENSIDE: u8 = 8;

    pub fn from_fen(fen_cr_str: &str) -> Result<Self, anyhow::Error> {
        let mut cr = 0;
        for c in fen_cr_str.chars() {
            match c {
                'K' => cr |= CastlingRights::WHITE_KINGSIDE,
                'Q' => cr |= CastlingRights::WHITE_QUEENSIDE,
                'k' => cr |= CastlingRights::BLACK_KINGSIDE,
                'q' => cr |= CastlingRights::BLACK_QUEENSIDE,
                '-' => {}
                _ => bail!("Invalid castling rights string {}", fen_cr_str),
            }
        }
        Ok(CastlingRights(cr))
    }

    pub fn to_fen(&self) -> String {
        let mut fen_str = String::new();
        if self.is_set(CastlingRights::WHITE_KINGSIDE) {
            fen_str.push_str("K");
        }
        if self.is_set(CastlingRights::WHITE_QUEENSIDE) {
            fen_str.push_str("Q");
        }
        if self.is_set(CastlingRights::BLACK_KINGSIDE) {
            fen_str.push_str("k");
        }
        if self.is_set(CastlingRights::BLACK_QUEENSIDE) {
            fen_str.push_str("q");
        }

        if fen_str.len() == 0 {
            fen_str.push_str("-");
        }

        fen_str
    }

    pub fn set(&mut self, value: u8) {
        self.0 |= value;
    }

    pub fn unset(&mut self, value: u8) {
        self.0 &= !value;
    }

    pub fn is_set(&self, value: u8) -> bool {
        self.0 & value > 0
    }

    pub fn castling_positions(&self, turn: &Turn, pieces: u64) -> u64 {
        match turn {
            Turn::White => {
                let mut positions = 0;
                if self.is_set(CastlingRights::WHITE_KINGSIDE) {
                    // f1 and g1 must be empty
                    if 0x60 & pieces == 0 {
                        positions |= 0x40
                    }
                };
                if self.is_set(CastlingRights::WHITE_QUEENSIDE) {
                    // b1, c1, and d1 must be empty
                    if 0xe & pieces == 0 {
                        positions |= 0x4
                    }
                };
                positions
            }
            Turn::Black => {
                let mut positions = 0;
                if self.is_set(CastlingRights::BLACK_KINGSIDE) {
                    // f8 and g8 must be empty
                    if 0x6000000000000000 & pieces == 0 {
                        positions |= 0x4000000000000000
                    }
                };
                if self.is_set(CastlingRights::BLACK_QUEENSIDE) {
                    // b8, c8, and d8 must be empty
                    if 0xe00000000000000 & pieces == 0 {
                        positions |= 0x400000000000000
                    }
                };
                positions
            }
        }
    }

    pub fn position_hash(&self) -> u64 {
        (self.0 as u64).wrapping_mul(*get_multiplicative_hash())
    }
}

static CASTLING_MULTIPLICTIVE_HASH: OnceLock<u64> = OnceLock::new();

fn get_multiplicative_hash() -> &'static u64 {
    CASTLING_MULTIPLICTIVE_HASH.get_or_init(|| random::<u64>())
}
