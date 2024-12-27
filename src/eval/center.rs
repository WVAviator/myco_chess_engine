use crate::{
    cgame::game::{Game, Turn},
    movegen::MoveGen,
};

const INNER_CENTER: u64 = 0x1818000000;
const OUTER_CENTER: u64 = 0x3c3c3c3c0000;

const INNER_CENTER_OCCUPANCY_BONUS: i32 = 12;
const INNER_CENTER_VISION_BONUS: i32 = 8;
const OUTER_CENTER_VISION_BONUS: i32 = 2;

const ENDGAME_PIECE_COUNT: u32 = 18;

pub trait CenterEval {
    fn calculate_center_value(&self) -> i32;
}

impl CenterEval for Game {
    fn calculate_center_value(&self) -> i32 {
        let mut value = 0;

        if self.board.all().count_ones() < ENDGAME_PIECE_COUNT {
            return value;
        }

        let white_pieces = self.board.white_pieces();
        let black_pieces = self.board.black_pieces();

        let white_vision = self.generate_vision(&Turn::White);
        let black_vision = self.generate_vision(&Turn::Black);

        value += (INNER_CENTER & white_pieces).count_ones() as i32 * INNER_CENTER_OCCUPANCY_BONUS;
        value -= (INNER_CENTER & black_pieces).count_ones() as i32 * INNER_CENTER_OCCUPANCY_BONUS;

        value += (INNER_CENTER & white_vision).count_ones() as i32 * INNER_CENTER_VISION_BONUS;
        value -= (INNER_CENTER & black_vision).count_ones() as i32 * INNER_CENTER_VISION_BONUS;

        value += (OUTER_CENTER & white_vision).count_ones() as i32 * OUTER_CENTER_VISION_BONUS;
        value -= (OUTER_CENTER & black_vision).count_ones() as i32 * OUTER_CENTER_VISION_BONUS;

        value
    }
}
