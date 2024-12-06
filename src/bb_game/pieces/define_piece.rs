macro_rules! define_piece {
    ($name:ident) => {
        use crate::bb_game::bitboard::Bitboard;

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name(Bitboard);

        impl From<u64> for $name {
            fn from(bits: u64) -> Self {
                $name(Bitboard::from(bits))
            }
        }

        impl Into<u64> for $name {
            fn into(self) -> u64 {
                self.0.into()
            }
        }

        impl From<Bitboard> for $name {
            fn from(bb: Bitboard) -> Self {
                $name(bb)
            }
        }

        impl Into<Bitboard> for $name {
            fn into(self) -> Bitboard {
                self.0
            }
        }
    };
}
