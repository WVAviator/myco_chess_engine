use std::{
    cmp::{self, max_by, Reverse},
    collections::{BinaryHeap, HashMap},
    time::{Duration, Instant},
};

use crate::{eval::Eval, movegen::MoveGen};

use super::{
    game::{Game, Turn},
    moves::SimpleMove,
};

#[derive(Debug, Clone, PartialEq)]
pub struct SimpleEngine<'a> {
    game: &'a Game,
}

impl<'a> SimpleEngine<'a> {
    pub fn new(game: &'a Game) -> Self {
        Self { game }
    }

    pub fn get_best_move(
        &self,
        depth: u32,
        time_remaining: Duration,
    ) -> Result<SimpleMove, anyhow::Error> {
        match self.game.turn {
            Turn::White => self.get_white_best_move(depth, time_remaining),
            Turn::Black => self.get_black_best_move(depth, time_remaining),
        }
    }

    fn get_white_best_move(
        &self,
        depth: u32,
        time_remaining: Duration,
    ) -> Result<SimpleMove, anyhow::Error> {
        let deadline = Instant::now() + time_remaining;

        if self.game.is_checkmate() {
            return Ok(SimpleMove::empty_evaluation(i32::MIN));
        } else if self.game.is_stalemate() {
            return Ok(SimpleMove::empty_evaluation(0));
        }

        let mut moves_heap: BinaryHeap<SimpleMove> = self
            .game
            .generate_legal_moves()
            .into_iter()
            .map(|lmove| {
                let game = self.game.apply_move(&lmove);
                let mut lmove = lmove.clone();
                lmove.evaluation = game.evaluate_position();
                lmove
            })
            .collect();

        let mut best_move_eval = i32::MIN;
        let mut best_move = moves_heap
            .peek()
            .unwrap_or_else(|| panic!("info string no moves to evaluate"))
            .clone();

        if depth == 0 {
            return Ok(best_move);
        }

        while Instant::now() < deadline {
            if let Some(lmove) = moves_heap.pop() {
                let next_turn = self.game.apply_move(&lmove);
                let next_engine = SimpleEngine::new(&next_turn);
                let time_to_eval = (deadline.duration_since(Instant::now())).div_f32(3.0);
                let best_response = next_engine.get_best_move(depth - 1, time_to_eval)?;

                if best_response.evaluation > best_move_eval {
                    best_move = lmove;
                    best_move_eval = best_response.evaluation;
                    best_move.evaluation = best_response.evaluation;
                }

                if best_move.evaluation == i32::MAX {
                    // Found checkmate, no need to continue
                    break;
                }
            } else {
                break;
            }
        }
        Ok(best_move)
    }

    fn get_black_best_move(
        &self,
        depth: u32,
        time_remaining: Duration,
    ) -> Result<SimpleMove, anyhow::Error> {
        let deadline = Instant::now() + time_remaining;

        if self.game.is_checkmate() {
            return Ok(SimpleMove::empty_evaluation(i32::MAX));
        } else if self.game.is_stalemate() {
            return Ok(SimpleMove::empty_evaluation(0));
        }

        let mut moves_heap: BinaryHeap<Reverse<SimpleMove>> = self
            .game
            .generate_legal_moves()
            .into_iter()
            .map(|lmove| {
                let game = self.game.apply_move(&lmove);
                let mut lmove = lmove.clone();
                lmove.evaluation = game.evaluate_position();
                Reverse(lmove)
            })
            .collect();

        let mut best_move_eval = i32::MAX;
        let mut best_move = moves_heap
            .peek()
            .unwrap_or_else(|| panic!("info string no moves to evaluate"))
            .0
            .clone();

        if depth == 0 {
            return Ok(best_move);
        }

        while Instant::now() < deadline {
            if let Some(Reverse(lmove)) = moves_heap.pop() {
                let next_turn = self.game.apply_move(&lmove);
                let next_engine = SimpleEngine::new(&next_turn);
                let time_to_eval = (deadline.duration_since(Instant::now())).div_f32(3.0);
                let best_response = next_engine.get_best_move(depth - 1, time_to_eval)?;

                if best_response.evaluation < best_move_eval {
                    best_move = lmove;
                    best_move_eval = best_response.evaluation;
                    best_move.evaluation = best_response.evaluation;
                }

                if best_move.evaluation == i32::MIN {
                    // Found checkmate, no need to continue
                    break;
                }
            } else {
                break;
            }
        }
        Ok(best_move)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn taking_queen_best_move() {
        let game = Game::from_fen("8/5kr1/8/8/2R2q2/8/3K4/8 w - - 0 1").unwrap();
        let engine = SimpleEngine::new(&game);

        let best_move = engine.get_best_move(1, Duration::from_millis(100)).unwrap();

        assert_eq!(best_move, SimpleMove::from_algebraic("c4f4").unwrap());
    }

    #[test]
    fn should_not_take_poisoned_queen() {
        let game = Game::from_fen("qN6/R7/r1p3pk/8/8/5P2/1r6/5K2 w - - 0 1").unwrap();
        let engine = SimpleEngine::new(&game);

        let best_move = engine.get_best_move(3, Duration::from_millis(100)).unwrap();

        assert_eq!(best_move, SimpleMove::from_algebraic("a7a6").unwrap());
    }
    #[test]
    fn identifies_skewers() {
        let game = Game::from_fen("5q2/8/8/5k2/8/1R6/1K6/8 w - - 0 1").unwrap();
        let engine = SimpleEngine::new(&game);

        let best_move = engine.get_best_move(2, Duration::from_millis(100)).unwrap();

        assert_eq!(best_move, SimpleMove::from_algebraic("b3f3").unwrap());
    }

    #[test]
    fn identifies_forks() {
        let game = Game::from_fen("5q2/8/1N3k2/8/8/P7/1PP5/1K6 w - - 0 1").unwrap();
        let engine = SimpleEngine::new(&game);

        let best_move = engine.get_best_move(2, Duration::from_millis(100)).unwrap();

        assert_eq!(best_move, SimpleMove::from_algebraic("b6d7").unwrap());
    }
}
