use crate::game::error::InvalidAlgebraicNotationError;

pub struct Square {
    row: u8,
    col: u8,
}

impl Square {
    pub fn from_an(an: &str) -> Result<Self, InvalidAlgebraicNotationError> {
        if an.len() != 2 {
            return Err(InvalidAlgebraicNotationError::new("Expected square only."));
        }

        let mut an_iter = an.chars();

        // Unwrap ok here since already confirmed len is 2
        let file = an_iter.next().unwrap();
        let rank = an_iter.next().unwrap()
            .to_digit(10)
            .ok_or_else(|rank| { InvalidAlgebraicNotationError::new(format!("Provided rank {} does not exist on a chessboard.", rank).as_str()) })?;

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
            _ => return Err(InvalidAlgebraicNotationError::new(format!("Provided file {} does not exist on a chessboard.", file).as_str())),
        };

        Ok(Square {
            row,
            col
        })
    }
}

