use std::simd::{
    num::{SimdInt, SimdUint},
    Simd,
};

use crate::{
    game::game::{Game, Turn},
    moves::simple_move::SimpleMove,
    util::simd::SimdCountOnes,
};

const PIECE_VALUE: Simd<i32, 8> = Simd::from_array([100, 500, 300, 325, 900, 20000, 0, 0]);

pub trait MVVLVAEval {
    /// Provides an evaluation, in centipawns, of a move based on the value of the victim (piece at
    /// dest) and the value of the attacker (piece at orig). TO be used when sorting a list of
    /// potential moves by MVV (Most Valuable Victim) - LVA (Least Valuable Attacker).
    ///
    /// Intended for use in initial move ordering, not as a positional evaluation.
    fn evaluate_mvv_lva(&self, lmove: &SimpleMove) -> i32;
}

impl MVVLVAEval for Game {
    fn evaluate_mvv_lva(&self, lmove: &SimpleMove) -> i32 {
        let orig = Simd::splat(lmove.orig);
        let dest = Simd::splat(lmove.dest);

        match self.turn {
            Turn::White => {
                let attacker = (self.board.white & orig).count_ones().cast::<i32>();
                let victim = (self.board.black & dest).count_ones().cast::<i32>();
                ((victim - attacker) * PIECE_VALUE).reduce_sum()
            }
            Turn::Black => {
                let attacker = (self.board.black & orig).count_ones().cast::<i32>();
                let victim = (self.board.white & dest).count_ones().cast::<i32>();
                ((victim - attacker) * PIECE_VALUE).reduce_sum()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn evaluates_white_pawn_takes_queen() {
        let game = Game::from_fen("8/6k1/8/8/4q3/5P2/1K6/8 w - - 0 1").unwrap();
        let lmove = SimpleMove::from_algebraic("f3e4").unwrap();

        let eval = game.evaluate_mvv_lva(&lmove);
        assert_eq!(eval, 800);
    }

    #[test]
    fn evaluates_black_knight_takes_rook() {
        let game = Game::from_fen("8/6k1/8/3n4/5R2/8/1K6/8 b - - 0 1").unwrap();
        let lmove = SimpleMove::from_algebraic("d5f4").unwrap();

        let eval = game.evaluate_mvv_lva(&lmove);
        assert_eq!(eval, 200);
    }

    #[test]
    fn evaluates_white_bishop_takes_bishop() {
        let game = Game::from_fen("8/6k1/2b5/8/8/8/1K6/7B w - - 0 1").unwrap();
        let lmove = SimpleMove::from_algebraic("h1c6").unwrap();

        let eval = game.evaluate_mvv_lva(&lmove);
        assert_eq!(eval, 0);
    }

    #[test]
    fn no_attack_evaluates_negative() {
        let game = Game::from_fen("8/6k1/2b5/8/8/8/1K6/7B w - - 0 1").unwrap();
        let lmove = SimpleMove::from_algebraic("h1d5").unwrap();

        let eval = game.evaluate_mvv_lva(&lmove);
        assert_eq!(eval, -325);
    }
}
