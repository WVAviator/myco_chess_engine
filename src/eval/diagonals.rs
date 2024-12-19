use crate::cgame::game::Game;

const DARK_SQUARES: u64 = 0xaa55aa55aa55aa55;
const LIGHT_SQUARES: u64 = 0x55aa55aa55aa55aa;

pub trait DiagonalEval {
    fn calculate_diagonal_value(&self) -> i32;
}

impl DiagonalEval for Game {
    fn calculate_diagonal_value(&self) -> i32 {
        let mut value = 0;

        let black_pieces = self.board.black_pieces();
        let white_pieces = self.board.white_pieces();

        let blocked_dark_squares =
            ((white_pieces | self.board.black_pawns) & DARK_SQUARES).count_ones();
        let blocked_light_squares =
            ((white_pieces | self.board.black_pawns) & LIGHT_SQUARES).count_ones();

        let dark_square_bishops =
            (self.board.white_bishops | self.board.white_queens) & DARK_SQUARES;
        let light_square_bishops =
            (self.board.white_bishops | self.board.white_queens) & LIGHT_SQUARES;

        // TODO: adjust value and repeat for black as negative

        0
    }
}
