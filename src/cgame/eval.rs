use super::game::{Game, Turn};

pub struct SimpleEvaluator {}

const KING_VALUE: u32 = 1000000;
const QUEEN_VALUE: u32 = 900;
const ROOK_VALUE: u32 = 500;
const BISHOP_VALUE: u32 = 350;
const KNIGHT_VALUE: u32 = 300;
const PAWN_VALUE: u32 = 100;

const THREAT_VALUE: u32 = 25;
const DEFENSE_VALUE: u32 = 20;

const SCOPE_VALUE: u32 = 5;
const TURN_VALUE: u32 = 20;

impl SimpleEvaluator {
    pub fn evaluate(game: &Game) -> i32 {
        let mut centipawns: i32 = 0;

        if game.is_checkmate() {
            return match game.turn {
                Turn::White => i32::MIN,
                Turn::Black => i32::MAX,
            };
        } else if game.is_stalemate() {
            return 0;
        }

        let white_vision = game.get_white_vision();
        let black_vision = game.get_black_vision();

        let white_pieces = game.board.white_pieces();
        let black_pieces = game.board.black_pieces();

        // Scope represents the number of squares each side's pieces see.
        centipawns += (white_vision.count_ones() * SCOPE_VALUE) as i32;
        centipawns -= (black_vision.count_ones() * SCOPE_VALUE) as i32;

        // Threats are pieces that could be taken now or on the next turn
        centipawns += ((white_vision & black_pieces).count_ones() * THREAT_VALUE) as i32;
        centipawns -= ((black_vision & white_pieces).count_ones() * THREAT_VALUE) as i32;

        // Defense is the number of pieces each side defends with other pieces
        centipawns += ((white_vision & white_pieces).count_ones() * DEFENSE_VALUE) as i32;
        centipawns -= ((black_vision & black_pieces).count_ones() * DEFENSE_VALUE) as i32;

        // The player whose turn it is currently has a slight advantage
        match game.turn {
            Turn::White => centipawns += TURN_VALUE as i32,
            Turn::Black => centipawns -= TURN_VALUE as i32,
        };

        // Piece values
        centipawns += (game.board.white_king.count_ones() * KING_VALUE) as i32;
        centipawns -= (game.board.black_king.count_ones() * KING_VALUE) as i32;

        centipawns += (game.board.white_queens.count_ones() * QUEEN_VALUE) as i32;
        centipawns -= (game.board.black_queens.count_ones() * QUEEN_VALUE) as i32;

        centipawns += (game.board.white_rooks.count_ones() * ROOK_VALUE) as i32;
        centipawns -= (game.board.black_rooks.count_ones() * ROOK_VALUE) as i32;

        centipawns += (game.board.white_knights.count_ones() * KNIGHT_VALUE) as i32;
        centipawns -= (game.board.black_knights.count_ones() * KNIGHT_VALUE) as i32;

        centipawns += (game.board.white_bishops.count_ones() * BISHOP_VALUE) as i32;
        centipawns -= (game.board.black_bishops.count_ones() * BISHOP_VALUE) as i32;

        centipawns += (game.board.white_pawns.count_ones() * PAWN_VALUE) as i32;
        centipawns -= (game.board.black_pawns.count_ones() * PAWN_VALUE) as i32;

        centipawns
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn evaluates_simple_position() {
        let game = Game::from_fen("rk6/8/8/8/8/8/5P2/6KR w - - 0 1").unwrap();

        let advantage = SimpleEvaluator::evaluate(&game);

        assert!(advantage > 0); // White has the advantage
    }
}
