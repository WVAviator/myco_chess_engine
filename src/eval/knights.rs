use crate::{
    cgame::game::{Game, Turn},
    movegen::MoveGen,
};

const BOARD_EDGE_PENALTY: i32 = 48;
const OUTPOST_BONUS: i32 = 64;

const BOARD_EDGE: u64 = 0xff818181818181ff;
const OUTPOST_SQUARES_NORTH: u64 = 0x7e7e7e00000000;
const OUTPOST_SQUARES_SOUTH: u64 = 0x7e7e7e00;

pub trait KnightEval {
    fn calculate_knights_value(&self) -> i32;
}

impl KnightEval for Game {
    fn calculate_knights_value(&self) -> i32 {
        let mut value = 0;

        let white_vision = self.generate_vision(&Turn::White);
        let black_vision = self.generate_vision(&Turn::Black);

        value -= (self.board.white_knights & BOARD_EDGE).count_ones() as i32 * BOARD_EDGE_PENALTY;
        value += (self.board.black_knights & BOARD_EDGE).count_ones() as i32 * BOARD_EDGE_PENALTY;

        value += (self.board.white_knights & OUTPOST_SQUARES_NORTH & white_vision).count_ones()
            as i32
            * OUTPOST_BONUS;
        value -= (self.board.black_knights & OUTPOST_SQUARES_SOUTH & black_vision).count_ones()
            as i32
            * OUTPOST_BONUS;

        value
    }
}