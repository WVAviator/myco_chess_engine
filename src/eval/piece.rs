use std::simd::{
    num::{SimdInt, SimdUint},
    Simd,
};

use crate::{
    game::game::{Game, Turn},
    moves::common::PieceType,
    util::simd::SimdCountOnes,
};

include!("./piece_tables.rs");

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

const ZERO: Simd<u64, 8> = Simd::from_array([0, 0, 0, 0, 0, 0, 0, 0]);
const ONE: Simd<u64, 8> = Simd::from_array([1, 1, 1, 1, 1, 1, 1, 1]);

const PIECE_TABLE_MASK: Simd<u64, 8> = Simd::from_array([1, 1, 1, 1, 1, 1, 0, 0]);
const PIECE_TABLE_OFFSETS: Simd<u64, 8> = Simd::from_array([0, 65, 130, 195, 260, 325, 0, 0]);
const ENDGAME_OFFSET: Simd<u64, 8> = Simd::from_array([390, 390, 390, 390, 390, 390, 0, 0]);

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

        let mut remaining_white = self.board.white * PIECE_TABLE_MASK;
        while remaining_white != ZERO {
            let index = remaining_white.trailing_zeros()
                + PIECE_TABLE_OFFSETS
                + match is_endgame {
                    true => ENDGAME_OFFSET,
                    false => ZERO,
                };

            let result = Simd::gather_or(&WHITE_PIECE_TABLES, index.cast(), ZERO.cast());
            value += result.reduce_sum();

            remaining_white &= remaining_white - ONE;
        }

        let mut remaining_black = self.board.black * PIECE_TABLE_MASK;
        while remaining_black != ZERO {
            let index = remaining_black.trailing_zeros()
                + PIECE_TABLE_OFFSETS
                + match is_endgame {
                    true => ENDGAME_OFFSET,
                    false => ZERO,
                };

            let result = Simd::gather_or(&BLACK_PIECE_TABLES, index.cast(), ZERO.cast());
            value -= result.reduce_sum();

            remaining_black &= remaining_black - ONE;
        }

        value
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn zero_indicies_set() {
        assert_eq!(WHITE_PIECE_TABLES[64], 0);
        assert_eq!(WHITE_PIECE_TABLES[129], 0);
        assert_eq!(WHITE_PIECE_TABLES[194], 0);
        assert_eq!(WHITE_PIECE_TABLES[259], 0);
        assert_eq!(WHITE_PIECE_TABLES[519], 0);
        assert_eq!(WHITE_PIECE_TABLES[779], 0);
    }

    #[test]
    fn evaluates_simultaneous_piece_values() {
        // An endgame with two kings and two pawns
        let game = Game::from_fen("8/5k2/8/1p6/7P/2K5/8/8 w - - 0 1").unwrap();

        // Material value is zero since both sides have the same material
        // White king on c3: 11 * 1 (white)
        // White pawn on h4: -1 * 1 (white)
        // Black king on f7: 4 * -1 (black)
        // Black pawn on b5: 9 * -1 (black)

        let eval = game.calculate_piece_value();
        assert_eq!(eval, -3);
    }

    #[test]
    fn evaluates_symmetrical_position() {
        let game = Game::from_fen("b7/5k2/8/3p3p/3P3P/8/5K2/B7 w - - 0 1").unwrap();

        let eval = game.calculate_piece_value();
        assert_eq!(eval, 0);
    }
}
