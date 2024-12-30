use std::{
    cmp,
    time::{Duration, Instant},
};

use rayon::prelude::*;

use crate::{
    cache::eval::EvaluationCache,
    database::{connection::get_connection, retrieve::MoveRetrieval},
    eval::{mvvlva::MVVLVAEval, piece::PieceEval, threats::ThreatEval},
    game::game::{Game, Turn},
    hash::zobrist::ZobristHash,
    movegen::MoveGen,
    moves::simple_move::SimpleMove,
};

pub struct QuiescenceSearch<'a> {
    root: &'a Game,
    max_depth: usize,
    deadline: Instant,
}

impl<'a> QuiescenceSearch<'a> {
    pub fn new(root: &'a Game, max_depth: usize, max_seconds: u64) -> Self {
        let deadline = Instant::now() + Duration::from_secs(max_seconds);
        Self {
            root,
            max_depth,
            deadline,
        }
    }

    pub fn search(&self) -> SimpleMove {
        println!("info score cp {}", self.root.calculate_piece_value());

        if let Ok(Some(database_move)) = self.root.random_database_move(&get_connection()) {
            println!("info string book move {}", database_move);
            return database_move;
        }

        let legal_moves = self.root.generate_legal_moves();

        let mut evaluations: Vec<MoveEvaluation> = legal_moves
            .into_par_iter()
            .map(|lmove| {
                MoveEvaluation(
                    lmove,
                    self.root.evaluate_mvv_lva(lmove) + self.root.evaluate_threats(lmove),
                )
            })
            .collect();

        evaluations.sort_unstable_by(|a, b| b.cmp(a));

        let mut evaluations: Vec<MoveEvaluation> = evaluations
            .into_iter()
            .take_while(|_| Instant::now() < self.deadline)
            .par_bridge()
            .map(|eval| {
                println!("info currmove {}", eval.0.to_algebraic());
                MoveEvaluation(
                    eval.0,
                    self.root.apply_move(eval.0).quiescence_eval(
                        self.max_depth,
                        i32::MIN,
                        i32::MAX,
                    ),
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
        let zobrist = self.zobrist();

        if let Some(eval) = EvaluationCache::get(zobrist) {
            return eval;
        }

        if depth == 0 {
            // Don't insert into the cache in this case since the eval might be premature
            return self.calculate_piece_value();
        }

        let mut alpha = alpha;
        let mut beta = beta;

        let legal_moves = self.generate_legal_moves();
        let white_vision = self.generate_vision(&Turn::White)[6];
        let black_vision = self.generate_vision(&Turn::Black)[6];

        if (white_vision & self.board.black[6]) | (black_vision & self.board.white[6]) == 0 {
            let eval = self.calculate_piece_value();
            EvaluationCache::insert(zobrist, eval);
            return eval;
        }

        match self.turn {
            Turn::White => {
                let mut tactical_moves = legal_moves
                    .into_iter()
                    .map(|lmove| {
                        let eval = self.evaluate_mvv_lva(&lmove) + self.evaluate_threats(&lmove);
                        TacticalEvaluation(lmove, eval)
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
                        let eval = self.evaluate_mvv_lva(&lmove) + self.evaluate_threats(&lmove);
                        TacticalEvaluation(lmove, eval)
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
struct TacticalEvaluation(SimpleMove, i32);

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
