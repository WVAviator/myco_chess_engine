use anyhow::bail;

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CastlingRights(u8);

impl CastlingRights {
    pub const WHITE_KINGSIDE: u8 = 1;
    pub const WHITE_QUEENSIDE: u8 = 2;
    pub const BLACK_KINGSIDE: u8 = 4;
    pub const BLACK_QUEENSIDE: u8 = 8;

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

    pub fn set(&mut self, value: u8) {
        self.0 |= value;
    }

    pub fn unset(&mut self, value: u8) {
        self.0 &= !value;
    }

    pub fn is_set(&self, value: u8) -> bool {
        self.0 & value > 0
    }
}
