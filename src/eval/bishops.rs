use crate::game::{
    constants::{EIGHTH_RANK, FIRST_RANK},
    game::Game,
};

const DARK_SQUARES: u64 = 0xaa55aa55aa55aa55;
const LIGHT_SQUARES: u64 = 0x55aa55aa55aa55aa;
const LONG_DIAGONAL: u64 = 0x8040201008040201;
const LONG_ANTIDIAGONAL: u64 = 0x102040810204080;

const BISHOP_MOBILITY_FACTOR: i32 = 128;
const LONG_DIAGONAL_BONUS: i32 = 0;

pub trait BishopEval {
    fn calculate_bishop_value(&self) -> i32;
}

impl BishopEval for Game {
    fn calculate_bishop_value(&self) -> i32 {
        let mut value = 0;

        let blocked_dark_squares = ((self.board.white[0] | self.board.black[0])
            & (DARK_SQUARES & !FIRST_RANK & !EIGHTH_RANK))
            .count_ones() as i32;
        let blocked_light_squares = ((self.board.white[0] | self.board.black[0])
            & (LIGHT_SQUARES & !FIRST_RANK & !EIGHTH_RANK))
            .count_ones() as i32;

        // 16 Blocked squares is the maximum, and the bishop of that color is essentially useless
        // Up to BISHOP_MOBILITY_FACTOR points should be docked per bishop in that case

        let dark_square_white_bishops = (self.board.white[3] | self.board.white[4]) & DARK_SQUARES;
        let light_square_white_bishops =
            (self.board.white[3] | self.board.white[4]) & LIGHT_SQUARES;

        value -= blocked_dark_squares
            * (dark_square_white_bishops.count_ones() as i32)
            * (BISHOP_MOBILITY_FACTOR / 16);
        value -= blocked_light_squares
            * (light_square_white_bishops.count_ones() as i32)
            * (BISHOP_MOBILITY_FACTOR / 16);

        let dark_square_black_bishops = (self.board.black[3] | self.board.black[4]) & DARK_SQUARES;
        let light_square_black_bishops =
            (self.board.black[3] | self.board.black[4]) & LIGHT_SQUARES;

        value += blocked_dark_squares
            * (dark_square_black_bishops.count_ones() as i32)
            * (BISHOP_MOBILITY_FACTOR / 16);
        value += blocked_light_squares
            * (light_square_black_bishops.count_ones() as i32)
            * (BISHOP_MOBILITY_FACTOR / 16);

        // Bonus points should be awarded when a bishop is placed on a long diagonal, even if the diagonal is blocked

        value += ((self.board.white[3] | self.board.white[4]) & (LONG_DIAGONAL | LONG_ANTIDIAGONAL))
            .count_ones() as i32
            * LONG_DIAGONAL_BONUS;
        value -= ((self.board.black[3] | self.board.black[4]) & (LONG_DIAGONAL | LONG_ANTIDIAGONAL))
            .count_ones() as i32
            * LONG_DIAGONAL_BONUS;

        value
    }
}
