use crate::cgame::{
    constants::{EIGHTH_RANK, FIRST_RANK},
    game::Game,
};

const DARK_SQUARES: u64 = 0xaa55aa55aa55aa55;
const LIGHT_SQUARES: u64 = 0x55aa55aa55aa55aa;
const LONG_DIAGONAL: u64 = 0x8040201008040201;
const LONG_ANTIDIAGONAL: u64 = 0x102040810204080;

const BISHOP_MOBILITY_FACTOR: i32 = 256;
const LONG_DIAGONAL_BONUS: i32 = 32;

pub trait BishopEval {
    fn calculate_bishop_value(&self) -> i32;
}

impl BishopEval for Game {
    fn calculate_bishop_value(&self) -> i32 {
        let mut value = 0;

        let black_pieces = self.board.black_pieces();
        let white_pieces = self.board.white_pieces();

        let blocked_dark_squares = ((self.board.white_pawns | self.board.black_pawns)
            & (DARK_SQUARES & !FIRST_RANK & !EIGHTH_RANK))
            .count_ones();
        let blocked_light_squares = ((self.board.white_pawns | self.board.black_pawns)
            & (LIGHT_SQUARES & !FIRST_RANK & !EIGHTH_RANK))
            .count_ones();

        // 16 Blocked squares is the maximum, and the bishop of that color is essentially useless
        // Up to BISHOP_MOBILITY_FACTOR points should be docked per bishop in that case

        let dark_square_white_bishops =
            (self.board.white_bishops | self.board.white_queens) & DARK_SQUARES;
        let light_square_white_bishops =
            (self.board.white_bishops | self.board.white_queens) & LIGHT_SQUARES;

        value -= blocked_dark_squares * dark_square_white_bishops * (BISHOP_MOBILITY_FACTOR / 16);
        value -= blocked_light_squares * light_square_white_bishops * (BISHOP_MOBILITY_FACTOR / 16);

        let dark_square_black_bishops =
            (self.board.black_bishops | self.board.black_queens) & DARK_SQUARES;
        let light_square_black_bishops =
            (self.board.black_bishops | self.board.black_queens) & LIGHT_SQUARES;

        value += blocked_dark_squares * dark_square_black_bishops * (BISHOP_MOBILITY_FACTOR / 16);
        value += blocked_light_squares * light_square_black_bishops * (BISHOP_MOBILITY_FACTOR / 16);

        // Bonus points should be awarded when a bishop is placed on a long diagonal, even if the diagonal is blocked

        value += ((self.board.white_bishops | self.board.white_queens)
            & (LONG_DIAGONAL | LONG_ANTIDIAGONAL))
            * LONG_DIAGONAL_BONUS;
        value -= ((self.board.black_bishops | self.board.black_queens)
            & (LONG_DIAGONAL | LONG_ANTIDIAGONAL))
            * LONG_DIAGONAL_BONUS;

        value
    }
}
