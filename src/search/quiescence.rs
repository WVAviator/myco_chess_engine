use std::cmp;

use rayon::prelude::*;

use crate::{
    eval::piece::PieceEval,
    game::game::{Game, Turn},
    movegen::MoveGen,
    moves::simple_move::SimpleMove,
};

pub struct QuiescenceSearch<'a> {
    root: &'a Game,
    max_depth: usize,
}

impl<'a> QuiescenceSearch<'a> {
    pub fn new(root: &'a Game, max_depth: usize) -> Self {
        Self { root, max_depth }
    }

    pub fn search(&self) -> SimpleMove {
        let legal_moves = self.root.generate_legal_moves();
        let mut evaluations: Vec<MoveEvaluation> = legal_moves
            .into_par_iter()
            .map(|lmove| {
                MoveEvaluation(
                    lmove,
                    self.root
                        .apply_move(lmove)
                        .quiescence_eval(self.max_depth, i32::MIN, i32::MAX),
                )
            })
            .collect();

        evaluations.sort_unstable();

        match self.root.turn {
            Turn::White => evaluations
                .last()
                .expect("no moves available for position")
                .0
                .clone(),
            Turn::Black => evaluations
                .first()
                .expect("no moves available for position")
                .0
                .clone(),
        }
    }
}

pub trait QuiescenceEval {
    fn quiescence_eval(&self, depth: usize, alpha: i32, beta: i32) -> i32;
}

impl QuiescenceEval for Game {
    fn quiescence_eval(&self, depth: usize, alpha: i32, beta: i32) -> i32 {
        if depth == 0 {
            return self.calculate_piece_value();
        }

        let mut alpha = alpha;
        let mut beta = beta;

        let legal_moves = self.generate_legal_moves();
        let white_vision = self.generate_vision(&Turn::White);
        let black_vision = self.generate_vision(&Turn::Black);

        if (white_vision & self.board.black[6]) | (black_vision & self.board.white[6]) == 0 {
            return self.calculate_piece_value();
        }

        match self.turn {
            Turn::White => {
                let mut tactical_moves = legal_moves
                    .into_iter()
                    .map(|lmove| {
                        let mut tactical_activity = 0;
                        // Captures
                        tactical_activity += (lmove.dest & self.board.black[6]).count_ones();
                        // Saves
                        tactical_activity += (lmove.orig & black_vision).count_ones();
                        // Sacrifices
                        tactical_activity += (lmove.dest & black_vision).count_ones();

                        TacticalEvaluation(lmove, tactical_activity)
                    })
                    .collect::<Vec<TacticalEvaluation>>();

                tactical_moves.sort_unstable_by(|a, b| b.cmp(a));

                let mut highest_eval = i32::MIN;

                for tmove in tactical_moves {
                    let eval = self
                        .apply_move(&tmove.0)
                        .quiescence_eval(depth - 1, alpha, beta);
                    highest_eval = cmp::max(eval, highest_eval);
                    alpha = cmp::max(highest_eval, alpha);
                    if beta <= alpha {
                        break;
                    }
                }

                highest_eval
            }

            Turn::Black => {
                let mut tactical_moves = legal_moves
                    .into_iter()
                    .map(|lmove| {
                        let mut tactical_activity = 0;
                        // Captures
                        tactical_activity += (lmove.dest & self.board.white[6]).count_ones();
                        // Saves
                        tactical_activity += (lmove.orig & white_vision).count_ones();
                        // Sacrifices
                        tactical_activity += (lmove.dest & white_vision).count_ones();

                        TacticalEvaluation(lmove, tactical_activity)
                    })
                    .collect::<Vec<TacticalEvaluation>>();

                tactical_moves.sort_unstable_by(|a, b| b.cmp(a));

                let mut lowest_eval = i32::MAX;

                for tmove in tactical_moves {
                    let eval = self
                        .apply_move(&tmove.0)
                        .quiescence_eval(depth - 1, alpha, beta);
                    lowest_eval = cmp::min(eval, lowest_eval);
                    beta = cmp::min(lowest_eval, beta);
                    if beta <= alpha {
                        break;
                    }
                }

                lowest_eval
            }
        }

        //
        // let is_check = self.king_in_check();
        // let tactical_activity = self.evaluate_tactical_activity() + if is_check { 1 } else { 0 };
        //
        // if tactical_activity == 0 {
        //     return self.calculate_piece_value();
        // }
        //
        // let mut tactical_evals = self
        //     .generate_legal_moves()
        //     .into_iter()
        //     .map(|lmove| {
        //         let game = self.apply_move(&lmove);
        //         let tactical_eval = game.evaluate_tactical_activity();
        //         IntermediateEvaluation(game, tactical_eval as i32)
        //     })
        //     .collect::<Vec<IntermediateEvaluation>>();
        //
        // tactical_evals.sort_unstable_by(|a, b| b.cmp(a));
        //
        // match self.turn {
        //     Turn::White => tactical_evals
        //         .into_iter()
        //         .map(|eval| eval.0.quiescence_eval(depth - 1))
        //         .max()
        //         .unwrap_or(match is_check {
        //             true => -200000,
        //             false => 0,
        //         }),
        //     Turn::Black => tactical_evals
        //         .into_iter()
        //         .map(|eval| eval.0.quiescence_eval(depth - 1))
        //         .max()
        //         .unwrap_or(match is_check {
        //             true => 200000,
        //             false => 0,
        //         }),
        // }
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

#[derive(Debug, PartialEq, Eq, Clone)]
struct TacticalEvaluation(SimpleMove, u32);

impl PartialOrd for TacticalEvaluation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.1.partial_cmp(&other.1)
    }
}

impl Ord for TacticalEvaluation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1)
    }
}
