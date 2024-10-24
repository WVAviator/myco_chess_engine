use crate::game::error::InvalidAlgebraicNotationError;

#[derive(Debug, Clone, PartialEq)]
pub struct Square {
    pub row: u8,
    pub col: u8,
}

impl Square {
    pub fn from_an(an: &str) -> Result<Self, InvalidAlgebraicNotationError> {
        if an.len() != 2 {
            return Err(InvalidAlgebraicNotationError::new("Expected square only."));
        }

        let mut an_iter = an.chars();

        // Unwrap ok here since already confirmed len is 2
        let file = an_iter.next().unwrap();
        let rank_chr = an_iter.next().unwrap();
        let rank = rank_chr.to_digit(10).ok_or_else(|| {
            InvalidAlgebraicNotationError::new(
                format!("Provided rank {} does not exist on a chessboard.", rank_chr).as_str(),
            )
        })?;

        if rank > 8 {
            return Err(InvalidAlgebraicNotationError::new(
                format!("Provided rank {} does not exist on a chessboard.", rank).as_str()
            ));
        }

        let row = (8 - rank) as u8;
        let col = match file {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => {
                return Err(InvalidAlgebraicNotationError::new(
                    format!("Provided file {} does not exist on a chessboard.", file).as_str(),
                ))
            }
        };

        Ok(Square { row, col })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn properly_interprets_an_b7() {
        let an = "b7";
        let square = Square::from_an(an).unwrap();
        assert_eq!(square, Square { row: 1, col: 1 })
    }

    #[test]
    fn properly_interprets_an_h8() {
        let an = "h8";
        let square = Square::from_an(an).unwrap();
        assert_eq!(square, Square { row: 0, col: 7 })
    }

    #[test]
    fn properly_interprets_an_a1() {
        let an = "a1";
        let square = Square::from_an(an).unwrap();
        assert_eq!(square, Square { row: 7, col: 0 })
    }

    #[test]
    fn properly_interprets_an_a8() {
        let an = "a8";
        let square = Square::from_an(an).unwrap();
        assert_eq!(square, Square { row: 0, col: 0 })
    }

    #[test]
    fn fails_for_invalid_an() {
        let an = "j9";
        let square = Square::from_an(an);
        assert!(square.is_err());
    }
}
