use std::{
    cmp,
    sync::OnceLock,
    time::{Duration, Instant},
};

use arrayvec::ArrayVec;
use dashmap::{DashMap, DashSet};
use rayon::prelude::*;

use crate::{
    cache::eval::EvaluationCache,
    database::{connection::get_connection, retrieve::MoveRetrieval},
    eval::{mvvlva::MVVLVAEval, piece::PieceEval},
    game::game::{Game, Turn},
    hash::zobrist::ZobristHash,
    movegen::MoveGen,
    moves::simple_move::SimpleMove,
};

use super::node::Node;

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

    pub fn search(&self) -> Option<SimpleMove> {
        println!("info score cp {}", self.root.calculate_piece_value());

        if let Ok(connection) = get_connection() {
            if let Ok(Some(database_move)) = self.root.random_database_move(&connection) {
                println!("info string book move {}", database_move);
                return Some(database_move);
            }
        }

        let root_node = Node::new(*self.root);

        let legal_moves = root_node.get_legal_moves();

        let mut evaluations: Vec<MoveEvaluation> = legal_moves
            .into_par_iter()
            .map(|lmove| {
                println!("info currmove {}", lmove.to_algebraic());
                MoveEvaluation(
                    lmove,
                    match self.root.turn {
                        Turn::White => root_node.apply_move(&lmove).quiescence_eval(
                            self.max_depth,
                            self.deadline,
                            i32::MIN,
                            i32::MAX,
                        ),
                        Turn::Black => -root_node.apply_move(&lmove).quiescence_eval(
                            self.max_depth,
                            self.deadline,
                            i32::MIN,
                            i32::MAX,
                        ),
                    },
                )
            })
            .collect();

        evaluations.sort_unstable();

        evaluations.last().map(|eval| *eval.0)
    }
}

pub trait QuiescenceEval {
    fn quiescence_eval(&self, depth: usize, deadline: Instant, alpha: i32, beta: i32) -> i32;
}

impl QuiescenceEval for Node {
    fn quiescence_eval(&self, depth: usize, deadline: Instant, alpha: i32, beta: i32) -> i32 {
        if let Some(eval) = EvaluationCache::get(*self.get_zobrist()) {
            return eval;
        }

        if depth == 0 || Instant::now() > deadline {
            return *self.get_static_eval();
        }

        let mut alpha = alpha;
        let mut beta = beta;

        let legal_moves = self.get_legal_moves();

        match self.game.turn {
            Turn::White => {
                if self.get_white_vision()[6] & self.game.board.black[6] == 0 {
                    // Quiet position
                    // TODO: Also verify no checks
                    let eval = *self.get_static_eval();
                    EvaluationCache::insert(*self.get_zobrist(), eval);
                    return eval;
                }

                let mut highest_eval = -200000;
                for lmove in legal_moves {
                    let eval =
                        self.apply_move(&lmove)
                            .quiescence_eval(depth - 1, deadline, alpha, beta);
                    highest_eval = cmp::max(eval, highest_eval);
                    alpha = cmp::max(highest_eval, alpha);
                    if beta <= alpha {
                        // TODO: Record this as a killer move
                        break;
                    }
                }

                if depth > 4 {
                    EvaluationCache::insert(*self.get_zobrist(), highest_eval);
                }

                highest_eval
            }
            Turn::Black => {
                if self.get_black_vision()[6] & self.game.board.white[6] == 0 {
                    // Quiet position
                    // TODO: Also verify no checks
                    let eval = *self.get_static_eval();
                    EvaluationCache::insert(*self.get_zobrist(), eval);
                    return eval;
                }

                let mut lowest_eval = 200000;
                for lmove in legal_moves {
                    let eval =
                        self.apply_move(&lmove)
                            .quiescence_eval(depth - 1, deadline, alpha, beta);
                    lowest_eval = cmp::min(eval, lowest_eval);
                    beta = cmp::min(lowest_eval, beta);
                    if beta <= alpha {
                        // TODO: Record this as a killer move
                        break;
                    }
                }

                if depth > 4 {
                    EvaluationCache::insert(*self.get_zobrist(), lowest_eval);
                }

                lowest_eval
            }
        }
    }
}

impl QuiescenceEval for Game {
    fn quiescence_eval(&self, depth: usize, deadline: Instant, alpha: i32, beta: i32) -> i32 {
        let zobrist = self.zobrist();

        if let Some(eval) = EvaluationCache::get(zobrist) {
            return eval;
        }

        if depth == 0 || Instant::now() > deadline {
            // Don't insert into the cache in this case since the eval might be premature
            return self.calculate_piece_value();
        }

        let mut alpha = alpha;
        let mut beta = beta;

        let pseudolegal_moves = self.generate_pseudolegal_moves();

        match self.turn {
            Turn::White => {
                if self.board.white[5] == 0 {
                    return -200000;
                }

                if self.generate_vision(&Turn::White)[6] & self.board.black[6] == 0 {
                    let eval = self.calculate_piece_value();
                    EvaluationCache::insert(zobrist, eval);
                    return eval;
                }

                let mut tactical_moves = pseudolegal_moves
                    .into_iter()
                    .map(|lmove| {
                        let eval = self.evaluate_mvv_lva(&lmove);
                        TacticalEvaluation(lmove, eval)
                    })
                    .collect::<Vec<TacticalEvaluation>>();

                tactical_moves.sort_unstable_by(|a, b| b.cmp(a));

                let mut highest_eval = i32::MIN;

                for tmove in tactical_moves {
                    let eval =
                        self.apply_move(&tmove.0)
                            .quiescence_eval(depth - 1, deadline, alpha, beta);
                    highest_eval = cmp::max(eval, highest_eval);
                    alpha = cmp::max(highest_eval, alpha);
                    if beta <= alpha {
                        break;
                    }
                }

                highest_eval
            }

            Turn::Black => {
                if self.board.black[5] == 0 {
                    return 200000;
                }

                if self.generate_vision(&Turn::Black)[6] & self.board.white[6] == 0 {
                    let eval = self.calculate_piece_value();
                    EvaluationCache::insert(zobrist, eval);
                    return eval;
                }

                let mut tactical_moves = pseudolegal_moves
                    .into_iter()
                    .map(|lmove| {
                        let eval = self.evaluate_mvv_lva(&lmove);
                        TacticalEvaluation(lmove, eval)
                    })
                    .collect::<Vec<TacticalEvaluation>>();

                tactical_moves.sort_unstable_by(|a, b| b.cmp(a));

                let mut lowest_eval = i32::MAX;

                for tmove in tactical_moves {
                    let eval =
                        self.apply_move(&tmove.0)
                            .quiescence_eval(depth - 1, deadline, alpha, beta);
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

impl PartialOrd for MoveEvaluation<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.1.cmp(&other.1))
    }
}

impl Ord for MoveEvaluation<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct TacticalEvaluation(SimpleMove, i32);

impl PartialOrd for TacticalEvaluation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.1.cmp(&other.1))
    }
}

impl Ord for TacticalEvaluation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1)
    }
}
