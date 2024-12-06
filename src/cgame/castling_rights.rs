use anyhow::bail;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CastlingRights(u8);

impl CastlingRights {
    pub const WHITE_KINGSIDE: u8 = 1;
    pub const WHITE_QUEENSIDE: u8 = 2;
    pub const BLACK_KINGSIDE: u8 = 4;
    pub const BLACK_QUEENSIDE: u8 = 8;

    fn from_fen(fen_cr_str: &str) -> Result<Self, anyhow::Error> {
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

    fn set(&mut self, value: u8) {
        self.0 |= value;
    }

    fn unset(&mut self, value: u8) {
        self.0 &= !value;
    }

    fn is_set(&self, value: u8) -> bool {
        self.0 & value > 0
    }
}
