use std::fmt;

use anyhow::anyhow;

#[derive(Debug, Clone, Eq)]
pub struct SimpleMove {
    orig_square: u64,
    dest_square: u64,
    promotion: Option<Promotion>,
    pub evaluation: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Promotion {
    Rook,
    Knight,
    Bishop,
    Queen,
}

impl SimpleMove {
    pub fn new(orig_square: u64, dest_square: u64) -> Self {
        SimpleMove {
            orig_square,
            dest_square,
            promotion: None,
            evaluation: 0,
        }
    }

    pub fn empty_evaluation(evaluation: i32) -> Self {
        Self {
            orig_square: 0,
            dest_square: 0,
            promotion: None,
            evaluation,
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

    pub fn get_promotion(&self) -> &Option<Promotion> {
        &self.promotion
    }

    pub fn new_promotion(orig_square: u64, dest_square: u64) -> Vec<Self> {
        vec![
            SimpleMove {
                orig_square,
                dest_square,
                promotion: Some(Promotion::Rook),
                evaluation: 0,
            },
            SimpleMove {
                orig_square,
                dest_square,
                promotion: Some(Promotion::Bishop),
                evaluation: 0,
            },
            SimpleMove {
                orig_square,
                dest_square,
                promotion: Some(Promotion::Knight),
                evaluation: 0,
            },
            SimpleMove {
                orig_square,
                dest_square,
                promotion: Some(Promotion::Queen),
                evaluation: 0,
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

        let orig_square = algebraic_to_u64(&algebraic[0..2]);
        let dest_square = algebraic_to_u64(&algebraic[2..4]);

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

        Ok(SimpleMove {
            orig_square,
            dest_square,
            promotion,
            evaluation: 0,
        })
    }
}

pub fn algebraic_to_u64(square: &str) -> u64 {
    if square.len() != 2 {
        panic!("Invalid square format: {}", square);
    }

    let chars: Vec<char> = square.chars().collect();
    let file = chars[0];
    let rank = chars[1];

    if !('a'..='h').contains(&file) || !('1'..='8').contains(&rank) {
        panic!("Invalid square coordinates: {}", square);
    }

    let file_index = (file as u8 - b'a') as u64;
    let rank_index = (rank as u8 - b'1') as u64;

    let square_bit = 1u64 << (rank_index * 8 + file_index);

    square_bit
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

impl PartialOrd for SimpleMove {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.evaluation.partial_cmp(&other.evaluation)
    }
}

impl Ord for SimpleMove {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.evaluation.cmp(&other.evaluation)
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
            orig_square: 1 << 52,
            dest_square: 1 << 60,
            promotion: Some(Promotion::Queen),
            evaluation: 0,
        };
        let long_algebraic_str = long_algebraic_move.to_algebraic().unwrap();
        assert_eq!(long_algebraic_str, "e7e8q");
    }

    #[test]
    fn converts_from_algebraic() {
        let long_algebraic_move = SimpleMove {
            orig_square: 1 << 52,
            dest_square: 1 << 60,
            promotion: Some(Promotion::Queen),
            evaluation: 0,
        };
        assert_eq!(
            SimpleMove::from_algebraic("e7e8q").unwrap(),
            long_algebraic_move
        );
    }
}
