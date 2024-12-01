use anyhow::bail;

use super::{piece::Piece, square::Square};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CMove {
    pub start: Square,
    pub dest: Square,
    pub promotion: Option<Piece>,
}

impl CMove {
    pub fn to_long_algebraic(self) -> String {
        format!(
            "{}{}{}",
            self.start.to_algebraic(),
            self.dest.to_algebraic(),
            match self.promotion {
                Some(piece) => piece.to_fen().to_ascii_lowercase(),
                None => String::from(""),
            }
        )
    }

    pub fn from_long_algebraic(algebraic: &str) -> Result<Self, anyhow::Error> {
        if algebraic.len() < 4 || algebraic.len() > 5 {
            bail!("Invalid long algebraic string: {}", algebraic);
        }
        let start = Square::from_algebraic(&algebraic[0..2])?;
        let dest = Square::from_algebraic(&algebraic[2..4])?;
        let promotion = {
            match algebraic.len() {
                5 => {
                    let piece_char = &algebraic[4..5];

                    match dest.get_rank() {
                        1 => Some(Piece::from_fen_char(piece_char)?),
                        8 => Some(Piece::from_fen_char(&piece_char.to_ascii_uppercase())?),
                        _ => bail!("Invalid pawn promotion. Must promote on the 1st or 8th ranks."),
                    }
                }
                4 => None,
                _ => bail!(
                    "Invalid long algebraic string - expected 4 or 5 characters: {}",
                    algebraic
                ),
            }
        };

        Ok(CMove {
            start,
            dest,
            promotion,
        })
    }

    pub fn new(from: Square, to: Square, promotion: Option<Piece>) -> Self {
        CMove {
            start: from,
            dest: to,
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
            start: Square::from_algebraic("a2").unwrap(),
            dest: Square::from_algebraic("c3").unwrap(),
            promotion: None,
        };
        assert_eq!(cmove.to_long_algebraic(), String::from("a2c3"));
    }

    #[test]
    fn formats_properly_long_algebraic_promotion() {
        let cmove = CMove {
            start: Square::from_algebraic("g7").unwrap(),
            dest: Square::from_algebraic("g8").unwrap(),
            promotion: Some(Piece::WhiteQueen),
        };
        assert_eq!(cmove.to_long_algebraic(), String::from("g7g8q"));
    }

    #[test]
    fn create_from_long_algebraic() {
        let cmove = CMove::from_long_algebraic("e2e4").unwrap();
        assert_eq!(
            cmove,
            CMove {
                start: Square::from_algebraic("e2").unwrap(),
                dest: Square::from_algebraic("e4").unwrap(),
                promotion: None
            }
        );
    }

    #[test]
    fn create_from_long_algebraic_promotion_black() {
        let cmove = CMove::from_long_algebraic("e2e1q").unwrap();
        assert_eq!(
            cmove,
            CMove {
                start: Square::from_algebraic("e2").unwrap(),
                dest: Square::from_algebraic("e1").unwrap(),
                promotion: Some(Piece::BlackQueen)
            }
        );
    }

    #[test]
    fn create_from_long_algebraic_promotion_white() {
        let cmove = CMove::from_long_algebraic("f7g8r").unwrap();
        assert_eq!(
            cmove,
            CMove {
                start: Square::from_algebraic("f7").unwrap(),
                dest: Square::from_algebraic("g8").unwrap(),
                promotion: Some(Piece::WhiteRook)
            }
        );
    }

    #[test]
    fn errors_with_invalid_promotion_square() {
        let result = CMove::from_long_algebraic("f6f7q");
        assert!(result.is_err());
    }
}
