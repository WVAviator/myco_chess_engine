use crate::cgame::game::Game;

const WHITE_ROOK_STARTING_POSITIONS: u64 = 0x81;
const WHITE_KNIGHT_STARTING_POSITIONS: u64 = 0x42;
const WHITE_BISHOP_STARTING_POSITIONS: u64 = 0x24;
const WHITE_QUEEN_STARTING_POSITION: u64 = 0x8;
const BLACK_ROOK_STARTING_POSITIONS: u64 = 0x8100000000000000;
const BLACK_KNIGHT_STARTING_POSITIONS: u64 = 0x4200000000000000;
const BLACK_BISHOP_STARTING_POSITIONS: u64 = 0x2400000000000000;
const BLACK_QUEEN_STARTING_POSITION: u64 = 0x800000000000000;

// Each array item represents 3 full moves (both sides go) of the game.
const UNDEVELOPED_ROOK_PENALTY: [i32; 10] = [0, 0, 0, 0, 16, 32, 48, 48, 64, 64];
const UNDEVELOPED_KNIGHT_PENALTY: [i32; 10] = [0, 16, 16, 32, 32, 64, 64, 64, 64, 64];
const UNDEVELOPED_BISHOP_PENALTY: [i32; 10] = [0, 0, 16, 16, 32, 32, 64, 64, 64, 64];
const UNDEVELOPED_QUEEN_PENALTY: [i32; 10] = [-64, -32, -16, 0, 0, 16, 16, 32, 32, 64]; // Negative values to punish early development

// Below this number, the endgame has started, development does not matter anymore
pub const ENDGAME_PIECE_COUNT: u32 = 18;

pub trait DevelopmentEval {
    fn calculate_development_value(&self) -> i32;
}

impl DevelopmentEval for Game {
    fn calculate_development_value(&self) -> i32 {
        let mut value = 0;

        if self.board.occupied().count_ones() < ENDGAME_PIECE_COUNT {
            return value;
        }

        let turn_index: usize = std::cmp::min((self.fullmove_number / 6) as usize, 9);

        value -= (self.board.white_rooks & WHITE_ROOK_STARTING_POSITIONS).count_ones() as i32
            * UNDEVELOPED_ROOK_PENALTY[turn_index];
        value -= (self.board.white_knights & WHITE_KNIGHT_STARTING_POSITIONS).count_ones() as i32
            * UNDEVELOPED_KNIGHT_PENALTY[turn_index];
        value -= (self.board.white_bishops & WHITE_BISHOP_STARTING_POSITIONS).count_ones() as i32
            * UNDEVELOPED_BISHOP_PENALTY[turn_index];
        value -= (self.board.white_queens & WHITE_QUEEN_STARTING_POSITION).count_ones() as i32
            * UNDEVELOPED_QUEEN_PENALTY[turn_index];

        value += (self.board.black_rooks & BLACK_ROOK_STARTING_POSITIONS).count_ones() as i32
            * UNDEVELOPED_ROOK_PENALTY[turn_index];
        value += (self.board.black_knights & BLACK_KNIGHT_STARTING_POSITIONS).count_ones() as i32
            * UNDEVELOPED_KNIGHT_PENALTY[turn_index];
        value += (self.board.black_bishops & BLACK_BISHOP_STARTING_POSITIONS).count_ones() as i32
            * UNDEVELOPED_BISHOP_PENALTY[turn_index];
        value += (self.board.black_queens & BLACK_QUEEN_STARTING_POSITION).count_ones() as i32
            * UNDEVELOPED_QUEEN_PENALTY[turn_index];

        value
    }
}
