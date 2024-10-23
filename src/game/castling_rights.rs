use crate::game::invalid_fen_error::InvalidFENStringError;

#[derive(Debug, Clone, PartialEq)]
pub struct CastlingRights {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

impl CastlingRights {
    pub fn from_fen_castling_str(castling_str: &str) -> Result<Self, InvalidFENStringError> {
        if castling_str.len() > 4 || castling_str.len() == 0 {
            return Err(InvalidFENStringError::new("Invalid number of characters in castling rights string."));
        }

        let white_kingside = castling_str.contains("K");
        let white_queenside = castling_str.contains("Q");
        let black_kingside = castling_str.contains("k");
        let black_queenside = castling_str.contains("q");

        Ok(CastlingRights {
            white_kingside,
            white_queenside,
            black_kingside,
            black_queenside,
        })
    }

    pub fn to_fen_castling_str(&self) -> String {
        let mut castling_string = String::new();

        if self.white_kingside {
            castling_string.push('K');
        }

        if self.white_queenside {
            castling_string.push('Q');
        }

        if self.black_kingside {
            castling_string.push('k');
        }

        if self.black_queenside {
            castling_string.push('q');
        }

        if castling_string.len() == 0 {
            castling_string.push('-');
        }

        castling_string
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn converts_full_castling_str() {
        let castling_str = "KQkq";
        let castling_rights = CastlingRights::from_fen_castling_str(castling_str).unwrap();

        assert_eq!(castling_rights, CastlingRights { white_kingside: true, white_queenside: true, black_kingside: true, black_queenside: true });
    }

    #[test]
    fn converts_partial_castling_str() {
        let castling_str = "Kq";
        let castling_rights = CastlingRights::from_fen_castling_str(castling_str).unwrap();

        assert_eq!(castling_rights, CastlingRights { white_kingside: true, white_queenside: false, black_kingside: false, black_queenside: true });
    }

    #[test]
    fn converts_to_castling_str() {
        let castling_rights = CastlingRights { white_kingside: false, white_queenside: true, black_kingside:true, black_queenside: false };
        let castling_string = castling_rights.to_fen_castling_str();

        assert_eq!(castling_string, "Qk")
    }

    #[test]
    fn converts_empty_rights() {
        let castling_str = "-";
        let castling_rights = CastlingRights::from_fen_castling_str(castling_str).unwrap();

        assert_eq!(castling_rights, CastlingRights { white_kingside: false, white_queenside: false, black_kingside: false, black_queenside: false });
    }
}