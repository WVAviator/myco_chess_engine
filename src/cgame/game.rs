use anyhow::bail;

use super::{board::Board, castling_rights::CastlingRights};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    board: Board,
    turn: Turn,
    castling_rights: CastlingRights,
    en_passant: u64,
    halfmove_clock: u32,
    fullmove_number: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Turn {
    White,
    Black,
}
