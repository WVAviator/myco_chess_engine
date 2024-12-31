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

    pub fn new_with_promotion(orig: u64, dest: u64, promotion: usize) -> Self {
        SimpleMove {
            orig,
            dest,
            promotion,
        }
    }

    pub fn new_promotion(orig: u64, dest: u64) -> [Self; 4] {
        [
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

    #[inline(always)]
    pub fn en_passant_target(&self, pawns: u64, empty: u64) -> u64 {
        let ep_orig = self.orig & pawns;
        let ep_orig_index = ep_orig.trailing_zeros() as usize;

        let ep_dest = self.dest & empty & PAWN_ATTACKS[ep_orig_index & !64];

        if ep_orig == 0 || ep_dest == 0 {
            return 0;
        }

        let ep_dest_index = ep_dest.trailing_zeros() as usize;

        let orig_file = ep_orig_index % 8;
        let dest_file = ep_dest_index % 8;

        // let direction = (orig_file < dest_file) as usize;
        // let side = (ep_orig_index & 8) << 1;
        // let index = (orig_file << 1) + direction + side;

        let index =
            ((ep_orig_index & 8) << 1) | (orig_file << 1) | ((orig_file < dest_file) as usize);

        ENPASSANT_CAPTURES[index]
    }
}

const ENPASSANT_CAPTURES: [u64; 32] = [
    0,
    0x200000000,
    0x100000000,
    0x400000000,
    0x200000000,
    0x800000000,
    0x400000000,
    0x1000000000,
    0x800000000,
    0x2000000000,
    0x1000000000,
    0x4000000000,
    0x2000000000,
    0x8000000000,
    0x4000000000,
    0,
    0,
    0x2000000,
    0x1000000,
    0x4000000,
    0x2000000,
    0x8000000,
    0x4000000,
    0x10000000,
    0x8000000,
    0x20000000,
    0x10000000,
    0x40000000,
    0x20000000,
    0x80000000,
    0x40000000,
    0,
];

const PAWN_ATTACKS: [u64; 64] = [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    163840,
    327681,
    655362,
    1310724,
    2621448,
    5242896,
    10485792,
    4194368,
    41943040,
    83886336,
    167772672,
    335545344,
    671090688,
    1342181376,
    2684362752,
    1073758208,
    10737418240,
    21474902016,
    42949804032,
    85899608064,
    171799216128,
    343598432256,
    687196864512,
    274882101248,
    2748779069440,
    5497574916096,
    10995149832192,
    21990299664384,
    43980599328768,
    87961198657536,
    175922397315072,
    70369817919488,
    703687441776640,
    1407379178520576,
    2814758357041152,
    5629516714082304,
    11259033428164608,
    22518066856329216,
    45036133712658432,
    18014673387388928,
    180143985094819840,
    360289069701267456,
    720578139402534912,
    1441156278805069824,
    2882312557610139648,
    5764625115220279296,
    11529250230440558592,
    4611756387171565568,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];

impl fmt::Display for SimpleMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self.to_algebraic())
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
    use crate::game::constants::{A_FILE, EIGHTH_RANK, FIRST_RANK, H_FILE};

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

    #[test]
    fn enpassant_target_correct() {
        let long_algebraic_move = SimpleMove {
            orig: algebraic_to_u64("d4").unwrap(),
            dest: algebraic_to_u64("e3").unwrap(),
            promotion: 0,
        };

        let pawns = algebraic_to_u64("d4").unwrap() | algebraic_to_u64("e4").unwrap();
        let empty = !pawns;

        assert_eq!(
            long_algebraic_move.en_passant_target(pawns, empty),
            algebraic_to_u64("e4").unwrap()
        );
    }

    #[test]
    fn enpassant_target_correct_black() {
        let long_algebraic_move = SimpleMove {
            orig: algebraic_to_u64("f5").unwrap(),
            dest: algebraic_to_u64("g6").unwrap(),
            promotion: 0,
        };

        let pawns = algebraic_to_u64("f5").unwrap() | algebraic_to_u64("g5").unwrap();
        let empty = !pawns;

        assert_eq!(
            long_algebraic_move.en_passant_target(pawns, empty),
            algebraic_to_u64("g5").unwrap()
        );
    }

    #[test]
    fn enpassant_target_correct_white() {
        let long_algebraic_move = SimpleMove {
            orig: algebraic_to_u64("h4").unwrap(),
            dest: algebraic_to_u64("g3").unwrap(),
            promotion: 0,
        };

        let pawns = algebraic_to_u64("h4").unwrap() | algebraic_to_u64("g4").unwrap();
        let empty = !pawns;

        assert_eq!(
            long_algebraic_move.en_passant_target(pawns, empty),
            algebraic_to_u64("g4").unwrap()
        );
    }

    #[ignore = "not a test"]
    #[test]
    fn generate_pawn_attacks() {
        let mut attacks = [0; 64];
        for i in 0..64 {
            let pawn = (1 << i) & !FIRST_RANK & !EIGHTH_RANK;

            attacks[i] |= (pawn & !A_FILE) << 7;
            attacks[i] |= (pawn & !A_FILE) >> 9;
            attacks[i] |= (pawn & !H_FILE) << 9;
            attacks[i] |= (pawn & !H_FILE) << 7;
        }

        println!("Pawn Attacks: {:?}", attacks);
    }
}
