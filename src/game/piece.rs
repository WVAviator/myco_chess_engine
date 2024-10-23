use crate::game::invalid_fen_error::InvalidFENStringError;

#[derive(Debug, Clone, PartialEq)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl Piece {
    pub fn from_fen_char(char: &str) -> Result<Piece, InvalidFENStringError> {
        match char {
            "p" => Ok(Piece { color: Color::Black, piece_type: PieceType::Pawn }),
            "r" => Ok(Piece { color: Color::Black, piece_type: PieceType::Rook }),
            "n" => Ok(Piece { color: Color::Black, piece_type: PieceType::Knight }),
            "b" => Ok(Piece { color: Color::Black, piece_type: PieceType::Bishop }),
            "q" => Ok(Piece { color: Color::Black, piece_type: PieceType::Queen }),
            "k" => Ok(Piece { color: Color::Black, piece_type: PieceType::King }),
            "P" => Ok(Piece { color: Color::White, piece_type: PieceType::Pawn }),
            "R" => Ok(Piece { color: Color::White, piece_type: PieceType::Rook }),
            "N" => Ok(Piece { color: Color::White, piece_type: PieceType::Knight }),
            "B" => Ok(Piece { color: Color::White, piece_type: PieceType::Bishop }),
            "Q" => Ok(Piece { color: Color::White, piece_type: PieceType::Queen }),
            "K" => Ok(Piece { color: Color::White, piece_type: PieceType::King }),
            _ => Err(InvalidFENStringError::new(format!("Invalid piece character '{}'", char).as_str()))
        }
    }

    pub fn to_fen_char(&self) -> String {
        let char = match self.piece_type {
            PieceType::Pawn => "p",
            PieceType::Rook => "r",
            PieceType::Knight => "n",
            PieceType::Bishop => "b",
            PieceType::Queen => "q",
            PieceType::King => "k",
        };

        match self.color {
            Color::Black => char.to_string(),
            Color::White => char.to_string().to_uppercase(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn creates_black_piece_from_fen_char() {
        let piece = Piece::from_fen_char("b").unwrap();
        assert_eq!(piece, Piece { color: Color::Black, piece_type: PieceType::Bishop });
    }

    #[test]
    fn creates_white_piece_from_fen_char() {
        let piece = Piece::from_fen_char("N").unwrap();
        assert_eq!(piece, Piece { color: Color::White, piece_type: PieceType::Knight });
    }

    #[test]
    fn converts_black_to_correct_fen_char() {
        let piece = Piece { color: Color::Black, piece_type: PieceType::Rook };
        let char = piece.to_fen_char();
        assert_eq!(char, "r");
    }

    #[test]
    fn converts_white_to_correct_fen_char() {
        let piece = Piece { color: Color::White, piece_type: PieceType::King };
        let char = piece.to_fen_char();
        assert_eq!(char, "K");
    }

    #[test]
    fn throws_error_invalid_char() {
        let piece = Piece::from_fen_char("j");
        assert!(piece.is_err());
    }

}