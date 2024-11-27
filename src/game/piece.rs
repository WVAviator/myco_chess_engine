use anyhow::anyhow;

#[derive(Debug, Clone, PartialEq)]
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
    pub fn from_fen_char(ch: char) -> Result<Self, anyhow::Error> {
        match ch {
          'P' => Ok(Piece::WhitePawn),
          'R' => Ok(Piece::WhiteRook),
          'N' => Ok(Piece::WhiteKnight),
          'B' => Ok(Piece::WhiteBishop),
          'Q' => Ok(Piece::WhiteQueen),
          'K' => Ok(Piece::WhiteKing),
          'p' => Ok(Piece::BlackPawn),
          'r' => Ok(Piece::BlackRook),
          'n' => Ok(Piece::BlackKnight),
          'b' => Ok(Piece::BlackBishop),
          'q' => Ok(Piece::BlackQueen),
          'k' => Ok(Piece::BlackKing),
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_correct_pieces() {
        assert_eq!(Piece::from_fen_char('K').unwrap(), Piece::WhiteKing);
        assert_eq!(Piece::from_fen_char('Q').unwrap(), Piece::WhiteQueen);
        assert_eq!(Piece::from_fen_char('R').unwrap(), Piece::WhiteRook);
        assert_eq!(Piece::from_fen_char('N').unwrap(), Piece::WhiteKnight);
        assert_eq!(Piece::from_fen_char('B').unwrap(), Piece::WhiteBishop);
        assert_eq!(Piece::from_fen_char('P').unwrap(), Piece::WhitePawn);

        assert_eq!(Piece::from_fen_char('k').unwrap(), Piece::BlackKing);
        assert_eq!(Piece::from_fen_char('q').unwrap(), Piece::BlackQueen);
        assert_eq!(Piece::from_fen_char('r').unwrap(), Piece::BlackRook);
        assert_eq!(Piece::from_fen_char('n').unwrap(), Piece::BlackKnight);
        assert_eq!(Piece::from_fen_char('b').unwrap(), Piece::BlackBishop);
        assert_eq!(Piece::from_fen_char('p').unwrap(), Piece::BlackPawn);
    }

    #[test]
    fn error_invalid_char() {
        let result = Piece::from_fen_char('g');

        assert!(result.is_err());
    }
}
