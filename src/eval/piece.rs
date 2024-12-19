use crate::cgame::game::Game;

const KING_VALUE: i32 = 10000000;
const QUEEN_VALUE: i32 = 900;
const ROOK_VALUE: i32 = 500;
const BISHOP_VALUE: i32 = 325;
const KNIGHT_VALUE: i32 = 300;
const PAWN_VALUE: i32 = 100;

pub trait PieceEval {
    fn calculate_piece_value(&self) -> i32;
}

impl PieceEval for Game {
    fn calculate_piece_value(&self) -> i32 {
        let mut value = 0;
        value += self.board.white_king.count_ones() as i32 * KING_VALUE;
        value += self.board.white_queens.count_ones() as i32 * QUEEN_VALUE;
        value += self.board.white_rooks.count_ones() as i32 * ROOK_VALUE;
        value += self.board.white_bishops.count_ones() as i32 * BISHOP_VALUE;
        value += self.board.white_knights.count_ones() as i32 * KNIGHT_VALUE;
        value += self.board.white_pawns.count_ones() as i32 * PAWN_VALUE;

        value -= self.board.black_king.count_ones() as i32 * KING_VALUE;
        value -= self.board.black_queens.count_ones() as i32 * QUEEN_VALUE;
        value -= self.board.black_rooks.count_ones() as i32 * ROOK_VALUE;
        value -= self.board.black_bishops.count_ones() as i32 * BISHOP_VALUE;
        value -= self.board.black_knights.count_ones() as i32 * KNIGHT_VALUE;
        value -= self.board.black_pawns.count_ones() as i32 * PAWN_VALUE;

        value
    }
}
