use anyhow::bail;

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct CastlingRights(u8);

impl CastlingRights {
    pub const WHITE_QUEENSIDE: u8 = 1;
    pub const WHITE_KINGSIDE: u8 = 2;
    pub const BLACK_QUEENSIDE: u8 = 4;
    pub const BLACK_KINGSIDE: u8 = 8;

    pub fn from_fen(fen_cr_str: &str) -> Result<Self, anyhow::Error> {
        let mut cr = 0;
        for c in fen_cr_str.chars() {
            match c {
                'K' => cr |= CastlingRights::WHITE_KINGSIDE,
                'Q' => cr |= CastlingRights::WHITE_QUEENSIDE,
                'k' => cr |= CastlingRights::BLACK_KINGSIDE,
                'q' => cr |= CastlingRights::BLACK_QUEENSIDE,
                '-' => {}
                _ => bail!("Invalid castling rights string {}", fen_cr_str),
            }
        }
        Ok(CastlingRights(cr))
    }

    pub fn to_fen(&self) -> String {
        let mut fen_str = String::new();
        if self.is_set(CastlingRights::WHITE_KINGSIDE) {
            fen_str.push('K');
        }
        if self.is_set(CastlingRights::WHITE_QUEENSIDE) {
            fen_str.push('Q');
        }
        if self.is_set(CastlingRights::BLACK_KINGSIDE) {
            fen_str.push('k');
        }
        if self.is_set(CastlingRights::BLACK_QUEENSIDE) {
            fen_str.push('q');
        }

        if fen_str.is_empty() {
            fen_str.push('-');
        }

        fen_str
    }

    #[inline]
    pub fn set(&mut self, value: u8) {
        self.0 |= value;
    }

    #[inline]
    pub fn unset(&mut self, value: u8) {
        self.0 &= !value;
    }

    #[inline]
    pub fn is_set(&self, value: u8) -> bool {
        self.0 & value > 0
    }

    #[inline(always)]
    pub fn forfeit(&mut self, orig: u64) {
        let unset = (orig & 1) // WQ Rook
            | ((orig & 128) >> 6) // WK Rook
            | ((orig & 0x100000000000000) >> 54) // BQ Rook
            | ((orig & 0x8000000000000000) >> 60) // BK Rook
            | ((orig & 16) >> 4) // WK
            | ((orig & 16) >> 3) // WK
            | ((orig & 0x1000000000000000) >> 58) // BK
            | ((orig & 0x1000000000000000) >> 57); // BK
        self.0 &= !(unset as u8);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn moving_rook_forfeits_castling() {
        let mut cr = CastlingRights(0b00001111);
        cr.forfeit(1);
        assert!(!cr.is_set(CastlingRights::WHITE_QUEENSIDE));
        assert!(cr.is_set(CastlingRights::WHITE_KINGSIDE));
        assert!(cr.is_set(CastlingRights::BLACK_QUEENSIDE));
        assert!(cr.is_set(CastlingRights::BLACK_KINGSIDE));
    }

    #[test]
    fn moving_black_rook_forfeits_castling() {
        let mut cr = CastlingRights(0b00001111);
        cr.forfeit(0x8000000000000000);
        assert!(cr.is_set(CastlingRights::WHITE_QUEENSIDE));
        assert!(cr.is_set(CastlingRights::WHITE_KINGSIDE));
        assert!(cr.is_set(CastlingRights::BLACK_QUEENSIDE));
        assert!(!cr.is_set(CastlingRights::BLACK_KINGSIDE));
    }

    #[test]
    fn moving_white_king_forfeits_both() {
        let mut cr = CastlingRights(0b00001111);
        cr.forfeit(16);
        assert!(!cr.is_set(CastlingRights::WHITE_QUEENSIDE));
        assert!(!cr.is_set(CastlingRights::WHITE_KINGSIDE));
        assert!(cr.is_set(CastlingRights::BLACK_QUEENSIDE));
        assert!(cr.is_set(CastlingRights::BLACK_KINGSIDE));
    }

    #[test]
    fn moving_black_king_forfeits_both() {
        let mut cr = CastlingRights(0b00001111);
        cr.forfeit(0x1000000000000000);
        assert!(cr.is_set(CastlingRights::WHITE_QUEENSIDE));
        assert!(cr.is_set(CastlingRights::WHITE_KINGSIDE));
        assert!(!cr.is_set(CastlingRights::BLACK_QUEENSIDE));
        assert!(!cr.is_set(CastlingRights::BLACK_KINGSIDE));
    }
}
