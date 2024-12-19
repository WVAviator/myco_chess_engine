use std::{
    cmp::{max, min},
    i32,
};

use rayon::prelude::*;

use crate::{
    cgame::{
        game::{Game, Turn},
        moves::SimpleMove,
    },
    eval::Eval,
    movegen::MoveGen,
};

pub struct MinmaxEngine<'a> {
    depth: u8,
    game: &'a Game,
}

impl<'a> MinmaxEngine<'a> {
    pub fn new(game: &'a Game, depth: u8) -> Self {
        MinmaxEngine { depth, game }
    }

    pub fn evaluate_best_move(&self) -> SimpleMove {
        let legal_moves = self.game.generate_legal_moves();

        let mut evaluations: Vec<MoveEvaluation<'_>> = legal_moves
            .into_par_iter()
            .map(|lmove| {
                MoveEvaluation(
                    lmove,
                    Self::minmax(self.depth, self.game.apply_move(&lmove), i32::MIN, i32::MAX),
                )
            })
            .collect();

        evaluations.sort_unstable();

        match self.game.turn {
            Turn::White => evaluations
                .last()
                .expect("info string no legal moves available")
                .0
                .clone(),
            Turn::Black => evaluations
                .first()
                .expect("info string no legal moves available")
                .0
                .clone(),
        }
    }

    fn minmax(depth: u8, game: Game, mut alpha: i32, mut beta: i32) -> i32 {
        if depth == 0 {
            return game.evaluate_position();
        }

        match game.turn {
            Turn::White => {
                let mut value = i32::MIN;
                for lmove in game.generate_pseudolegal_moves() {
                    value = max(
                        value,
                        Self::minmax(depth - 1, game.apply_move(&lmove), alpha, beta),
                    );
                    alpha = max(alpha, value);
                    if alpha >= beta {
                        break;
                    }
                }
                value
            }
            Turn::Black => {
                let mut value = i32::MAX;
                for lmove in game.generate_pseudolegal_moves() {
                    value = min(
                        value,
                        Self::minmax(depth - 1, game.apply_move(&lmove), alpha, beta),
                    );
                    beta = min(beta, value);
                    if beta <= alpha {
                        break;
                    }
                }
                value
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct MoveEvaluation<'a>(&'a SimpleMove, i32);

impl<'a> PartialOrd for MoveEvaluation<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.1.partial_cmp(&other.1)
    }
}

impl<'a> Ord for MoveEvaluation<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1)
    }
}
