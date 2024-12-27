use std::fmt;

use anyhow::anyhow;

use super::{
    common::{algebraic_to_u64, u64_to_algebraic, PieceType},
    contextual_move::ContextualMove,
};

#[derive(Debug, Clone, Eq)]
pub struct SimpleMove {
    pub orig: u64,
    pub dest: u64,
    pub promotion: usize,
}

impl SimpleMove {
    pub fn new(orig: u64, dest: u64) -> Self {
        SimpleMove {
            orig,
            dest,
            promotion: 0,
        }
    }

    pub fn new_promotion(orig: u64, dest: u64) -> Vec<Self> {
        vec![
            SimpleMove {
                orig,
                dest,
                promotion: 1,
            },
            SimpleMove {
                orig,
                dest,
                promotion: 3,
            },
            SimpleMove {
                orig,
                dest,
                promotion: 2,
            },
            SimpleMove {
                orig,
                dest,
                promotion: 4,
            },
        ]
    }

    pub fn to_algebraic(&self) -> String {
        format!(
            "{}{}{}",
            u64_to_algebraic(self.orig).unwrap(),
            u64_to_algebraic(self.dest).unwrap(),
            match self.promotion {
                1 => "r",
                2 => "n",
                3 => "b",
                4 => "q",
                _ => "",
            },
        )
    }

    pub fn from_algebraic(algebraic: &str) -> Result<Self, anyhow::Error> {
        let len = algebraic.len();
        if len < 4 || len > 5 {
            return Err(anyhow!("Invalid algebraic move: {}", algebraic));
        }

        let orig = algebraic_to_u64(&algebraic[0..2]).unwrap();
        let dest = algebraic_to_u64(&algebraic[2..4]).unwrap();

        let promotion = if len == 5 {
            match &algebraic[4..5] {
                "r" => 1,
                "n" => 2,
                "b" => 3,
                "q" => 4,
                _ => return Err(anyhow!("Invalid promotion piece: {}", &algebraic[4..5])),
            }
        } else {
            0
        };

        Ok(SimpleMove {
            orig,
            dest,
            promotion,
        })
    }

    pub fn en_passant_target(&self, pawns: u64, empty: u64) -> u64 {
        match (self.orig & pawns) | (self.dest & empty) {
            0x20100000000 => 0x200000000,
            0x40200000000 => 0x400000000,
            0x80400000000 => 0x800000000,
            0x100800000000 => 0x1000000000,
            0x201000000000 => 0x2000000000,
            0x402000000000 => 0x4000000000,
            0x804000000000 => 0x8000000000,

            0x10200000000 => 0x100000000,
            0x20400000000 => 0x200000000,
            0x40800000000 => 0x400000000,
            0x81000000000 => 0x800000000,
            0x102000000000 => 0x1000000000,
            0x204000000000 => 0x2000000000,
            0x408000000000 => 0x4000000000,

            0x10200 => 0x20000,
            0x20400 => 0x40000,
            0x40800 => 0x80000,
            0x81000 => 0x100000,
            0x102000 => 0x200000,
            0x204000 => 0x400000,
            0x408000 => 0x800000,

            0x20100 => 0x10000,
            0x40200 => 0x20000,
            0x80400 => 0x40000,
            0x100800 => 0x80000,
            0x201000 => 0x100000,
            0x402000 => 0x200000,
            0x804000 => 0x400000,

            _ => 0,
        }
    }
}

impl fmt::Display for SimpleMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self.to_algebraic())
    }
}

impl SimpleMove {
    pub fn print_list(moves: &Vec<SimpleMove>) {
        println!(
            "Moves: {}",
            moves
                .iter()
                .map(|m| m.to_algebraic())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
}

impl PartialEq for SimpleMove {
    fn eq(&self, other: &Self) -> bool {
        self.orig == other.orig && self.dest == other.dest && self.promotion == other.promotion
    }
}

impl From<&ContextualMove> for SimpleMove {
    fn from(value: &ContextualMove) -> Self {
        let promotion = match value.promotion {
            Some(PieceType::Rook) => 1,
            Some(PieceType::Knight) => 2,
            Some(PieceType::Bishop) => 3,
            Some(PieceType::Queen) => 4,
            _ => 0,
        };

        SimpleMove {
            orig: value.orig,
            dest: value.dest,
            promotion,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn algebraic_to_u64_correct() {
        let square_a1 = algebraic_to_u64("a1").unwrap();
        assert_eq!(square_a1, 1);
        let square_h1 = algebraic_to_u64("h1").unwrap();
        assert_eq!(square_h1, 1 << 7);
        let square_h8 = algebraic_to_u64("h8").unwrap();
        assert_eq!(square_h8, 1 << 63);
    }

    #[test]
    fn u64_to_algebraic_correct() {
        let square_a1 = u64_to_algebraic(1).unwrap();
        assert_eq!(square_a1, "a1");
        let square_h1 = u64_to_algebraic(1 << 7).unwrap();
        assert_eq!(square_h1, "h1");
        let square_h8 = u64_to_algebraic(1 << 63).unwrap();
        assert_eq!(square_h8, "h8");
    }

    #[test]
    fn converts_to_algebraic() {
        let long_algebraic_move = SimpleMove {
            orig: 1 << 52,
            dest: 1 << 60,
            promotion: 4,
        };
        let long_algebraic_str = long_algebraic_move.to_algebraic();
        assert_eq!(long_algebraic_str, "e7e8q");
    }

    #[test]
    fn converts_from_algebraic() {
        let long_algebraic_move = SimpleMove {
            orig: 1 << 52,
            dest: 1 << 60,
            promotion: 4,
        };
        assert_eq!(
            SimpleMove::from_algebraic("e7e8q").unwrap(),
            long_algebraic_move
        );
    }
}
