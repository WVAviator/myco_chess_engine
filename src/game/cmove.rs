use super::{board::Square, piece::Piece};

#[derive(Debug, Clone, PartialEq)]
pub struct CMove {
    pub starting_square: Square,
    pub destination_square: Square,
    pub promotion: Option<Piece>,
}

impl CMove {
    pub fn to_long_algebraic(self) -> String {
        format!(
            "{}{}{}",
            self.starting_square.to_algebraic(),
            self.destination_square.to_algebraic(),
            match self.promotion {
                Some(piece) => piece.to_fen().to_ascii_lowercase(),
                None => String::from(""),
            }
        )
    }

    pub fn new(from: Square, to: Square, promotion: Option<Piece>) -> Self {
        CMove {
            starting_square: from,
            destination_square: to,
            promotion,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn formats_properly_long_algebraic() {
        let cmove = CMove {
            starting_square: Square::from_algebraic("a2").unwrap(),
            destination_square: Square::from_algebraic("c3").unwrap(),
            promotion: None,
        };
        assert_eq!(cmove.to_long_algebraic(), String::from("a2c3"));
    }

    #[test]
    fn formats_properly_long_algebraic_promotion() {
        let cmove = CMove {
            starting_square: Square::from_algebraic("g7").unwrap(),
            destination_square: Square::from_algebraic("g8").unwrap(),
            promotion: Some(Piece::WhiteQueen),
        };
        assert_eq!(cmove.to_long_algebraic(), String::from("g7g8q"));
    }
}
