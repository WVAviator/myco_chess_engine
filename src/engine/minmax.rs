use std::{
    cmp::{max, min},
    i32,
    time::{Duration, Instant},
};

use rayon::prelude::*;

use crate::{
    cgame::game::{Game, Turn},
    eval::Eval,
    movegen::MoveGen,
    moves::simple_move::SimpleMove,
};

pub struct MinmaxEngine<'a> {
    depth: u8,
    game: &'a Game,
    deadline: Instant,
}

impl<'a> MinmaxEngine<'a> {
    pub fn new(game: &'a Game, depth: u8, max_seconds: u64) -> Self {
        let deadline = Instant::now() + Duration::from_secs(max_seconds);
        MinmaxEngine {
            depth,
            game,
            deadline,
        }
    }

    pub fn evaluate_best_move(&self) -> SimpleMove {
        let evaluation = self.game.evaluate_position();
        println!("info string current evaluation {}", evaluation);
        let legal_moves = self.game.generate_legal_moves();
        let mut legal_moves: Vec<MoveEvaluation<'_>> = legal_moves
            .iter()
            .map(|lmove| MoveEvaluation(lmove, self.game.evaluate_position()))
            .collect();

        match self.game.turn {
            Turn::White => {
                legal_moves.sort_unstable_by(|a, b| b.cmp(a));
            }
            Turn::Black => {
                legal_moves.sort_unstable();
            }
        }

        let mut evaluations: Vec<MoveEvaluation<'_>> = legal_moves
            .into_par_iter()
            .map(|move_eval| {
                MoveEvaluation(
                    move_eval.0,
                    self.minmax(
                        self.depth,
                        self.game.apply_move(move_eval.0),
                        i32::MIN,
                        i32::MAX,
                    ),
                )
            })
            .collect();

        evaluations.sort_unstable();

        let evaluation = match self.game.turn {
            Turn::White => {
                println!(
                    "info string selected move evaluation {}",
                    evaluations.last().unwrap().1
                );
                evaluations
                    .last()
                    .expect("info string no legal moves available")
                    .0
                    .clone()
            }
            Turn::Black => {
                println!(
                    "info string selected move evaluation {}",
                    evaluations.first().unwrap().1
                );
                evaluations
                    .first()
                    .expect("info string no legal moves available")
                    .0
                    .clone()
            }
        };

        evaluation
    }

    fn minmax(&self, depth: u8, game: Game, mut alpha: i32, mut beta: i32) -> i32 {
        if depth == 0 || Instant::now() > self.deadline {
            return game.evaluate_position();
        }

        match game.turn {
            Turn::White => {
                let mut value = i32::MIN;
                for lmove in game.generate_pseudolegal_moves() {
                    value = max(
                        value,
                        self.minmax(depth - 1, game.apply_move(&lmove), alpha, beta),
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
                        self.minmax(depth - 1, game.apply_move(&lmove), alpha, beta),
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
