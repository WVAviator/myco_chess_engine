use std::fmt;

use anyhow::anyhow;

use super::{
    common::{algebraic_to_u64, u64_to_algebraic, PieceType},
    contextual_move::ContextualMove,
};

#[derive(Debug, Clone, Eq)]
pub struct SimpleMove {
    orig_square: u64,
    dest_square: u64,
    promotion: Option<PieceType>,
}

impl SimpleMove {
    pub fn new(orig_square: u64, dest_square: u64) -> Self {
        SimpleMove {
            orig_square,
            dest_square,
            promotion: None,
        }
    }

    pub fn get_bits(&self) -> u64 {
        self.orig_square | self.dest_square
    }

    pub fn get_orig(&self) -> u64 {
        self.orig_square
    }

    pub fn get_dest(&self) -> u64 {
        self.dest_square
    }

    pub fn get_promotion(&self) -> &Option<PieceType> {
        &self.promotion
    }

    pub fn new_promotion(orig_square: u64, dest_square: u64) -> Vec<Self> {
        vec![
            SimpleMove {
                orig_square,
                dest_square,
                promotion: Some(PieceType::Rook),
            },
            SimpleMove {
                orig_square,
                dest_square,
                promotion: Some(PieceType::Bishop),
            },
            SimpleMove {
                orig_square,
                dest_square,
                promotion: Some(PieceType::Knight),
            },
            SimpleMove {
                orig_square,
                dest_square,
                promotion: Some(PieceType::Queen),
            },
        ]
    }

    pub fn to_algebraic(&self) -> Result<String, anyhow::Error> {
        Ok(format!(
            "{}{}{}",
            u64_to_algebraic(self.orig_square),
            u64_to_algebraic(self.dest_square),
            match self.promotion {
                Some(PieceType::Rook) => "r",
                Some(PieceType::Knight) => "n",
                Some(PieceType::Bishop) => "b",
                Some(PieceType::Queen) => "q",
                _ => "",
            },
        ))
    }

    pub fn from_algebraic(algebraic: &str) -> Result<Self, anyhow::Error> {
        let len = algebraic.len();
        if len < 4 || len > 5 {
            return Err(anyhow!("Invalid algebraic move: {}", algebraic));
        }

        let orig_square = algebraic_to_u64(&algebraic[0..2]);
        let dest_square = algebraic_to_u64(&algebraic[2..4]);

        let promotion = if len == 5 {
            match &algebraic[4..5] {
                "r" => Some(PieceType::Rook),
                "n" => Some(PieceType::Knight),
                "b" => Some(PieceType::Bishop),
                "q" => Some(PieceType::Queen),
                _ => return Err(anyhow!("Invalid promotion piece: {}", &algebraic[4..5])),
            }
        } else {
            None
        };

        Ok(SimpleMove {
            orig_square,
            dest_square,
            promotion,
        })
    }
}

impl fmt::Display for SimpleMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self.to_algebraic().unwrap())
    }
}

impl SimpleMove {
    pub fn print_list(moves: &Vec<SimpleMove>) {
        println!(
            "Moves: {}",
            moves
                .iter()
                .map(|m| m.to_algebraic().unwrap())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
}

impl PartialEq for SimpleMove {
    fn eq(&self, other: &Self) -> bool {
        self.orig_square == other.orig_square
            && self.dest_square == other.dest_square
            && self.promotion == other.promotion
    }
}

impl From<&ContextualMove> for SimpleMove {
    fn from(value: &ContextualMove) -> Self {
        SimpleMove {
            orig_square: value.orig,
            dest_square: value.dest,
            promotion: value.promotion.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn algebraic_to_u64_correct() {
        let square_a1 = algebraic_to_u64("a1");
        assert_eq!(square_a1, 1);
        let square_h1 = algebraic_to_u64("h1");
        assert_eq!(square_h1, 1 << 7);
        let square_h8 = algebraic_to_u64("h8");
        assert_eq!(square_h8, 1 << 63);
    }

    #[test]
    fn u64_to_algebraic_correct() {
        let square_a1 = u64_to_algebraic(1);
        assert_eq!(square_a1, "a1");
        let square_h1 = u64_to_algebraic(1 << 7);
        assert_eq!(square_h1, "h1");
        let square_h8 = u64_to_algebraic(1 << 63);
        assert_eq!(square_h8, "h8");
    }

    #[test]
    fn converts_to_algebraic() {
        let long_algebraic_move = SimpleMove {
            orig_square: 1 << 52,
            dest_square: 1 << 60,
            promotion: Some(PieceType::Queen),
        };
        let long_algebraic_str = long_algebraic_move.to_algebraic().unwrap();
        assert_eq!(long_algebraic_str, "e7e8q");
    }

    #[test]
    fn converts_from_algebraic() {
        let long_algebraic_move = SimpleMove {
            orig_square: 1 << 52,
            dest_square: 1 << 60,
            promotion: Some(PieceType::Queen),
        };
        assert_eq!(
            SimpleMove::from_algebraic("e7e8q").unwrap(),
            long_algebraic_move
        );
    }
}
