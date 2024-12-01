use core::fmt;

use anyhow::anyhow;

use super::game::Color;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Piece {
    WhitePawn,
    WhiteRook,
    WhiteKnight,
    WhiteBishop,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackRook,
    BlackKnight,
    BlackBishop,
    BlackQueen,
    BlackKing,
}

impl Piece {
    pub fn from_fen_char(ch: &str) -> Result<Self, anyhow::Error> {
        match ch {
          "P" => Ok(Piece::WhitePawn),
          "R" => Ok(Piece::WhiteRook),
          "N" => Ok(Piece::WhiteKnight),
          "B" => Ok(Piece::WhiteBishop),
          "Q" => Ok(Piece::WhiteQueen),
          "K" => Ok(Piece::WhiteKing),
          "p" => Ok(Piece::BlackPawn),
          "r" => Ok(Piece::BlackRook),
          "n" => Ok(Piece::BlackKnight),
          "b" => Ok(Piece::BlackBishop),
          "q" => Ok(Piece::BlackQueen),
          "k" => Ok(Piece::BlackKing),
          _ => Err(anyhow!("Invalid character in FEN string: {} does not match any known chess piece. Should be one of KQBNRPkqbnrp", ch)),
        }
    }

    pub fn to_fen(self) -> String {
        String::from(match self {
            Piece::WhitePawn => "P",
            Piece::WhiteRook => "R",
            Piece::WhiteKnight => "N",
            Piece::WhiteBishop => "B",
            Piece::WhiteQueen => "Q",
            Piece::WhiteKing => "K",
            Piece::BlackPawn => "p",
            Piece::BlackRook => "r",
            Piece::BlackKnight => "n",
            Piece::BlackBishop => "b",
            Piece::BlackQueen => "q",
            Piece::BlackKing => "k",
        })
    }

    pub fn get_color(&self) -> Color {
        match self {
            Piece::WhitePawn => Color::White,
            Piece::WhiteRook => Color::White,
            Piece::WhiteKnight => Color::White,
            Piece::WhiteBishop => Color::White,
            Piece::WhiteQueen => Color::White,
            Piece::WhiteKing => Color::White,
            Piece::BlackPawn => Color::Black,
            Piece::BlackRook => Color::Black,
            Piece::BlackKnight => Color::Black,
            Piece::BlackBishop => Color::Black,
            Piece::BlackQueen => Color::Black,
            Piece::BlackKing => Color::Black,
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Piece::WhitePawn => write!(f, "White Pawn"),
            Piece::WhiteRook => write!(f, "White Rook"),
            Piece::WhiteKnight => write!(f, "White Knight"),
            Piece::WhiteBishop => write!(f, "White Bishop"),
            Piece::WhiteQueen => write!(f, "White Queen"),
            Piece::WhiteKing => write!(f, "White King"),
            Piece::BlackPawn => write!(f, "Black Pawn"),
            Piece::BlackRook => write!(f, "Black Rook"),
            Piece::BlackKnight => write!(f, "Black Knight"),
            Piece::BlackBishop => write!(f, "Black Bishop"),
            Piece::BlackQueen => write!(f, "Black Queen"),
            Piece::BlackKing => write!(f, "Black King"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_correct_pieces() {
        assert_eq!(Piece::from_fen_char("K").unwrap(), Piece::WhiteKing);
        assert_eq!(Piece::from_fen_char("Q").unwrap(), Piece::WhiteQueen);
        assert_eq!(Piece::from_fen_char("R").unwrap(), Piece::WhiteRook);
        assert_eq!(Piece::from_fen_char("N").unwrap(), Piece::WhiteKnight);
        assert_eq!(Piece::from_fen_char("B").unwrap(), Piece::WhiteBishop);
        assert_eq!(Piece::from_fen_char("P").unwrap(), Piece::WhitePawn);

        assert_eq!(Piece::from_fen_char("k").unwrap(), Piece::BlackKing);
        assert_eq!(Piece::from_fen_char("q").unwrap(), Piece::BlackQueen);
        assert_eq!(Piece::from_fen_char("r").unwrap(), Piece::BlackRook);
        assert_eq!(Piece::from_fen_char("n").unwrap(), Piece::BlackKnight);
        assert_eq!(Piece::from_fen_char("b").unwrap(), Piece::BlackBishop);
        assert_eq!(Piece::from_fen_char("p").unwrap(), Piece::BlackPawn);
    }

    #[test]
    fn error_invalid_char() {
        let result = Piece::from_fen_char("g");

        assert!(result.is_err());
    }
}
