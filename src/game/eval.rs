use super::{game::Game, piece::Piece};

#[derive(Debug, Clone, PartialEq)]
pub struct SimpleEvaluator {}

impl SimpleEvaluator {
    pub const BASE_ROOK_VALUE: i32 = 500;
    pub const BASE_KNIGHT_VALUE: i32 = 300;
    pub const BASE_BISHOP_VALUE: i32 = 300;
    pub const BASE_QUEEN_VALUE: i32 = 900;
    pub const BASE_PAWN_VALUE: i32 = 100;
    pub const BASE_KING_VALUE: i32 = 100000;

    pub fn evaluate(game: &Game) -> i32 {
        let mut total_score = 0;
        for i in 0..64 {
            match game.board.at_index(i) {
                Some(Piece::BlackRook) => total_score -= SimpleEvaluator::BASE_ROOK_VALUE,
                Some(Piece::WhiteRook) => total_score += SimpleEvaluator::BASE_ROOK_VALUE,
                Some(Piece::BlackKnight) => total_score -= SimpleEvaluator::BASE_KNIGHT_VALUE,
                Some(Piece::WhiteKnight) => total_score += SimpleEvaluator::BASE_KNIGHT_VALUE,
                Some(Piece::BlackBishop) => total_score -= SimpleEvaluator::BASE_BISHOP_VALUE,
                Some(Piece::WhiteBishop) => total_score += SimpleEvaluator::BASE_BISHOP_VALUE,
                Some(Piece::BlackQueen) => total_score -= SimpleEvaluator::BASE_QUEEN_VALUE,
                Some(Piece::WhiteQueen) => total_score += SimpleEvaluator::BASE_QUEEN_VALUE,
                Some(Piece::BlackKing) => total_score -= SimpleEvaluator::BASE_KING_VALUE,
                Some(Piece::WhiteKing) => total_score += SimpleEvaluator::BASE_KING_VALUE,
                Some(Piece::BlackPawn) => total_score -= SimpleEvaluator::BASE_PAWN_VALUE,
                Some(Piece::WhitePawn) => total_score += SimpleEvaluator::BASE_PAWN_VALUE,
                None => {}
            }
        }

        total_score
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_evaluates_starting_position() {
        let game = Game::new_default();
        let score = SimpleEvaluator::evaluate(&game);
        assert_eq!(score, 0);
    }

    #[test]
    fn simple_evalues_uneven_position() {
        let game = Game::from_fen("8/8/8/8/8/8/8/Kqkr4 w - - 0 25").unwrap();
        let score = SimpleEvaluator::evaluate(&game);
        assert_eq!(score, -1400);
    }
}
