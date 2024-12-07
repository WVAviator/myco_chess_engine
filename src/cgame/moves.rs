use std::fmt;

use anyhow::anyhow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LongAlgebraicMove {
    orig_square: u64,
    dest_square: u64,
    promotion: Option<Promotion>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Promotion {
    Rook,
    Knight,
    Bishop,
    Queen,
}

impl LongAlgebraicMove {
    pub fn new(orig_square: u64, dest_square: u64) -> Self {
        LongAlgebraicMove {
            orig_square,
            dest_square,
            promotion: None,
        }
    }

    pub fn new_promotion(orig_square: u64, dest_square: u64) -> Vec<Self> {
        vec![
            LongAlgebraicMove {
                orig_square,
                dest_square,
                promotion: Some(Promotion::Rook),
            },
            LongAlgebraicMove {
                orig_square,
                dest_square,
                promotion: Some(Promotion::Bishop),
            },
            LongAlgebraicMove {
                orig_square,
                dest_square,
                promotion: Some(Promotion::Knight),
            },
            LongAlgebraicMove {
                orig_square,
                dest_square,
                promotion: Some(Promotion::Queen),
            },
        ]
    }

    pub fn to_algebraic(&self) -> Result<String, anyhow::Error> {
        Ok(format!(
            "{}{}{}",
            u64_to_algebraic(self.orig_square)?,
            u64_to_algebraic(self.dest_square)?,
            match self.promotion {
                Some(Promotion::Rook) => "r",
                Some(Promotion::Knight) => "n",
                Some(Promotion::Bishop) => "b",
                Some(Promotion::Queen) => "q",
                None => "",
            },
        ))
    }

    pub fn from_algebraic(algebraic: &str) -> Result<Self, anyhow::Error> {
        let len = algebraic.len();
        if len < 4 || len > 5 {
            return Err(anyhow!("Invalid algebraic move: {}", algebraic));
        }

        let orig_square = algebraic_to_u64(&algebraic[0..2])?;
        let dest_square = algebraic_to_u64(&algebraic[2..4])?;

        let promotion = if len == 5 {
            match &algebraic[4..5] {
                "r" => Some(Promotion::Rook),
                "n" => Some(Promotion::Knight),
                "b" => Some(Promotion::Bishop),
                "q" => Some(Promotion::Queen),
                _ => return Err(anyhow!("Invalid promotion piece: {}", &algebraic[4..5])),
            }
        } else {
            None
        };

        Ok(LongAlgebraicMove {
            orig_square,
            dest_square,
            promotion,
        })
    }
}

pub fn algebraic_to_u64(square: &str) -> Result<u64, anyhow::Error> {
    if square.len() != 2 {
        return Err(anyhow!("Invalid square format: {}", square));
    }

    let chars: Vec<char> = square.chars().collect();
    let file = chars[0];
    let rank = chars[1];

    if !('a'..='h').contains(&file) || !('1'..='8').contains(&rank) {
        return Err(anyhow!("Invalid square coordinates: {}", square));
    }

    let file_index = (file as u8 - b'a') as u64;
    let rank_index = (rank as u8 - b'1') as u64;

    let square_bit = 1u64 << (rank_index * 8 + file_index);

    Ok(square_bit)
}

pub fn u64_to_algebraic(square: u64) -> Result<String, anyhow::Error> {
    if square == 0 || square.count_ones() != 1 {
        return Err(anyhow::anyhow!(
            "Invalid square: {}. Must be a single bit set.",
            square
        ));
    }

    let position = square.trailing_zeros() as u64;
    let rank = position / 8;
    let file = position % 8;

    let file_char = (b'a' + file as u8) as char;
    let rank_char = (b'1' + rank as u8) as char;

    Ok(format!("{}{}", file_char, rank_char))
}

impl fmt::Display for LongAlgebraicMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self.to_algebraic().unwrap())
    }
}

impl LongAlgebraicMove {
    pub fn print_list(moves: &Vec<LongAlgebraicMove>) {
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
        let long_algebraic_move = LongAlgebraicMove {
            orig_square: 1 << 52,
            dest_square: 1 << 60,
            promotion: Some(Promotion::Queen),
        };
        let long_algebraic_str = long_algebraic_move.to_algebraic().unwrap();
        assert_eq!(long_algebraic_str, "e7e8q");
    }

    #[test]
    fn converts_from_algebraic() {
        let long_algebraic_move = LongAlgebraicMove {
            orig_square: 1 << 52,
            dest_square: 1 << 60,
            promotion: Some(Promotion::Queen),
        };
        assert_eq!(
            LongAlgebraicMove::from_algebraic("e7e8q").unwrap(),
            long_algebraic_move
        );
    }
}
