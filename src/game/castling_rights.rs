use anyhow::anyhow;

#[derive(Debug, Clone, PartialEq)]
pub struct CastlingRights(u8);

impl CastlingRights {
    pub const BLACK_KINGSIDE: u8 = 1 << 0;
    pub const BLACK_QUEENSIDE: u8 = 1 << 1;
    pub const WHITE_KINGSIDE: u8 = 1 << 2;
    pub const WHITE_QUEENSIDE: u8 = 1 << 3;

    pub fn new() -> Self {
        CastlingRights(0)
    }

    pub fn from_fen(fen_castling_str: &str) -> Result<Self, anyhow::Error> {
        let mut castling_rights = CastlingRights::new();

        if fen_castling_str.len() > 4 {
            return Err(anyhow!(
                "Invalid castling rights string. Too many characters: '{}'",
                fen_castling_str
            ));
        }

        if fen_castling_str == "-" {
            return Ok(castling_rights);
        }

        for ch in fen_castling_str.chars() {
            match ch {
                'K' => castling_rights.set(CastlingRights::WHITE_KINGSIDE, true),
                'Q' => castling_rights.set(CastlingRights::WHITE_QUEENSIDE, true),
                'k' => castling_rights.set(CastlingRights::BLACK_KINGSIDE, true),
                'q' => castling_rights.set(CastlingRights::BLACK_QUEENSIDE, true),
                _ => {
                    return Err(anyhow!(
                        "Invalid character in castling rights string: Character {} in string {} should be one of 'KQkq",
                        ch,
                        fen_castling_str
                    ))
                }
            }
        }

        if fen_castling_str.contains("K") {
            castling_rights.set(CastlingRights::WHITE_KINGSIDE, true);
        }
        if fen_castling_str.contains("Q") {
            castling_rights.set(CastlingRights::WHITE_QUEENSIDE, true);
        }
        if fen_castling_str.contains("k") {
            castling_rights.set(CastlingRights::BLACK_KINGSIDE, true);
        }
        if fen_castling_str.contains("q") {
            castling_rights.set(CastlingRights::BLACK_QUEENSIDE, true);
        }

        return Ok(castling_rights);
    }

    pub fn set(&mut self, right: u8, value: bool) {
        if value {
            self.0 |= right;
        } else {
            self.0 &= !right;
        }
    }

    pub fn is_set(&self, right: u8) -> bool {
        (self.0 & right) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_castling_rights() {
        let rights = CastlingRights::new();
        assert_eq!(rights.is_set(CastlingRights::BLACK_KINGSIDE), false);
        assert_eq!(rights.is_set(CastlingRights::BLACK_QUEENSIDE), false);
        assert_eq!(rights.is_set(CastlingRights::WHITE_KINGSIDE), false);
        assert_eq!(rights.is_set(CastlingRights::WHITE_QUEENSIDE), false);
    }

    #[test]
    fn set_castling_rights() {
        let mut rights = CastlingRights::new();
        rights.set(CastlingRights::BLACK_KINGSIDE, true);
        assert_eq!(rights.is_set(CastlingRights::BLACK_KINGSIDE), true);
        assert_eq!(rights.is_set(CastlingRights::BLACK_QUEENSIDE), false);

        rights.set(CastlingRights::BLACK_QUEENSIDE, true);
        assert_eq!(rights.is_set(CastlingRights::BLACK_QUEENSIDE), true);
    }

    #[test]
    fn unset_castling_rights() {
        let mut rights = CastlingRights::new();
        rights.set(CastlingRights::WHITE_KINGSIDE, true);
        rights.set(CastlingRights::WHITE_QUEENSIDE, true);

        assert_eq!(rights.is_set(CastlingRights::WHITE_KINGSIDE), true);
        assert_eq!(rights.is_set(CastlingRights::WHITE_QUEENSIDE), true);

        rights.set(CastlingRights::WHITE_KINGSIDE, false);
        assert_eq!(rights.is_set(CastlingRights::WHITE_KINGSIDE), false);
        assert_eq!(rights.is_set(CastlingRights::WHITE_QUEENSIDE), true);
    }

    #[test]
    fn multiple_castling_rights() {
        let mut rights = CastlingRights::new();
        rights.set(CastlingRights::BLACK_KINGSIDE, true);
        rights.set(CastlingRights::WHITE_QUEENSIDE, true);

        assert_eq!(rights.is_set(CastlingRights::BLACK_KINGSIDE), true);
        assert_eq!(rights.is_set(CastlingRights::BLACK_QUEENSIDE), false);
        assert_eq!(rights.is_set(CastlingRights::WHITE_KINGSIDE), false);
        assert_eq!(rights.is_set(CastlingRights::WHITE_QUEENSIDE), true);
    }

    #[test]
    fn castling_rights_from_fen() {
        let fen_str = "KQk";
        let rights = CastlingRights::from_fen(fen_str).unwrap();

        assert_eq!(rights.is_set(CastlingRights::WHITE_KINGSIDE), true);
        assert_eq!(rights.is_set(CastlingRights::WHITE_QUEENSIDE), true);
        assert_eq!(rights.is_set(CastlingRights::BLACK_KINGSIDE), true);
        assert_eq!(rights.is_set(CastlingRights::BLACK_QUEENSIDE), false);
    }

    #[test]
    fn empty_castling_rights_from_fen() {
        let fen_str = "-";
        let rights = CastlingRights::from_fen(fen_str).unwrap();

        assert_eq!(rights.is_set(CastlingRights::WHITE_KINGSIDE), false);
        assert_eq!(rights.is_set(CastlingRights::WHITE_QUEENSIDE), false);
        assert_eq!(rights.is_set(CastlingRights::BLACK_KINGSIDE), false);
        assert_eq!(rights.is_set(CastlingRights::BLACK_QUEENSIDE), false);
    }

    #[test]
    fn fails_with_bad_fen() {
        let fen_str = "K-";
        let rights_result = CastlingRights::from_fen(fen_str);

        assert!(rights_result.is_err());
    }
}
