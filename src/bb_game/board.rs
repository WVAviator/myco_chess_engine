use super::bitboard::Bitboard;

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    white_king: Bitboard,
    white_queen: Bitboard,
    white_bishop: Bitboard,
    white_knight: Bitboard,
    white_rook: Bitboard,
    white_pawn: Bitboard,

    black_king: Bitboard,
    black_queen: Bitboard,
    black_bishop: Bitboard,
    black_knight: Bitboard,
    black_rook: Bitboard,
    black_pawn: Bitboard,
}
