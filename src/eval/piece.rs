use std::simd::{num::SimdUint, Simd};

use crate::{
    game::game::{Game, Turn},
    moves::common::PieceType,
    util::simd::SimdCountOnes,
};

const KING_VALUE: u64 = 10000000;
const QUEEN_VALUE: u64 = 900;
const ROOK_VALUE: u64 = 500;
const BISHOP_VALUE: u64 = 325;
const KNIGHT_VALUE: u64 = 300;
const PAWN_VALUE: u64 = 100;

const PIECE_VALUES: Simd<u64, 8> = Simd::from_array([
    PAWN_VALUE,
    ROOK_VALUE,
    KNIGHT_VALUE,
    BISHOP_VALUE,
    QUEEN_VALUE,
    KING_VALUE,
    0,
    0,
]);

include!("./piece_tables.rs");

pub trait PieceEval {
    fn calculate_piece_value(&self) -> i32;
}

impl PieceEval for Game {
    fn calculate_piece_value(&self) -> i32 {
        let mut value = 0;

        value += (self.board.white.count_ones() * PIECE_VALUES).reduce_sum() as i32;
        value -= (self.board.black.count_ones() * PIECE_VALUES).reduce_sum() as i32;

        let is_endgame = self.board.all().count_ones() < 14
            || (self.board.all().count_ones() < 20
                && (self.board.white[4] | self.board.black[4]).count_ones() == 0);

        value += calculate_ps_value(
            self.board.white[0],
            &PieceType::Pawn,
            &Turn::White,
            is_endgame,
        );
        value -= calculate_ps_value(
            self.board.black[0],
            &PieceType::Pawn,
            &Turn::Black,
            is_endgame,
        );
        value += calculate_ps_value(
            self.board.white[2],
            &PieceType::Knight,
            &Turn::White,
            is_endgame,
        );
        value -= calculate_ps_value(
            self.board.black[2],
            &PieceType::Knight,
            &Turn::Black,
            is_endgame,
        );
        value += calculate_ps_value(
            self.board.white[3],
            &PieceType::Bishop,
            &Turn::White,
            is_endgame,
        );
        value -= calculate_ps_value(
            self.board.black[3],
            &PieceType::Bishop,
            &Turn::Black,
            is_endgame,
        );
        value += calculate_ps_value(
            self.board.white[1],
            &PieceType::Rook,
            &Turn::White,
            is_endgame,
        );
        value -= calculate_ps_value(
            self.board.black[1],
            &PieceType::Rook,
            &Turn::Black,
            is_endgame,
        );
        value += calculate_ps_value(
            self.board.white[4],
            &PieceType::Queen,
            &Turn::White,
            is_endgame,
        );
        value -= calculate_ps_value(
            self.board.black[4],
            &PieceType::Queen,
            &Turn::Black,
            is_endgame,
        );
        value += calculate_ps_value(
            self.board.white[5],
            &PieceType::King,
            &Turn::White,
            is_endgame,
        );
        value -= calculate_ps_value(
            self.board.black[5],
            &PieceType::King,
            &Turn::Black,
            is_endgame,
        );

        value
    }
}

fn calculate_ps_value(bitboard: u64, piece_type: &PieceType, turn: &Turn, endgame: bool) -> i32 {
    let mut eval: i32 = 0;

    let mut bb = bitboard;
    while bb != 0 {
        let index = bb.trailing_zeros() as usize;
        eval += match (piece_type, turn, endgame) {
            (PieceType::Pawn, Turn::White, false) => PAWN_MG_PS_TABLE[index],
            (PieceType::Pawn, Turn::White, true) => PAWN_EG_PS_TABLE[index],
            (PieceType::Pawn, Turn::Black, false) => PAWN_MG_PS_TABLE[64 - index - 1],
            (PieceType::Pawn, Turn::Black, true) => PAWN_EG_PS_TABLE[64 - index - 1],

            (PieceType::Knight, Turn::White, false) => KNIGHT_MG_PS_TABLE[index],
            (PieceType::Knight, Turn::White, true) => KNIGHT_EG_PS_TABLE[index],
            (PieceType::Knight, Turn::Black, false) => KNIGHT_MG_PS_TABLE[64 - index - 1],
            (PieceType::Knight, Turn::Black, true) => KNIGHT_EG_PS_TABLE[64 - index - 1],

            (PieceType::Bishop, Turn::White, false) => BISHOP_MG_PS_TABLE[index],
            (PieceType::Bishop, Turn::White, true) => BISHOP_EG_PS_TABLE[index],
            (PieceType::Bishop, Turn::Black, false) => BISHOP_MG_PS_TABLE[64 - index - 1],
            (PieceType::Bishop, Turn::Black, true) => BISHOP_EG_PS_TABLE[64 - index - 1],

            (PieceType::Rook, Turn::White, false) => ROOK_MG_PS_TABLE[index],
            (PieceType::Rook, Turn::White, true) => ROOK_EG_PS_TABLE[index],
            (PieceType::Rook, Turn::Black, false) => ROOK_MG_PS_TABLE[64 - index - 1],
            (PieceType::Rook, Turn::Black, true) => ROOK_EG_PS_TABLE[64 - index - 1],

            (PieceType::Queen, Turn::White, false) => QUEEN_MG_PS_TABLE[index],
            (PieceType::Queen, Turn::White, true) => QUEEN_EG_PS_TABLE[index],
            (PieceType::Queen, Turn::Black, false) => QUEEN_MG_PS_TABLE[64 - index - 1],
            (PieceType::Queen, Turn::Black, true) => QUEEN_EG_PS_TABLE[64 - index - 1],

            (PieceType::King, Turn::White, false) => KING_MG_PS_TABLE[index],
            (PieceType::King, Turn::White, true) => KING_EG_PS_TABLE[index],
            (PieceType::King, Turn::Black, false) => KING_MG_PS_TABLE[64 - index - 1],
            (PieceType::King, Turn::Black, true) => KING_EG_PS_TABLE[64 - index - 1],
        };

        bb &= bb - 1;
    }

    eval
}
