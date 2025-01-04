use std::{
    cmp,
    time::{Duration, Instant},
};

use rayon::prelude::*;

use crate::{
    cache::{eval::EvaluationCache, killer::KillerCache},
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

        let mut legal_moves: Vec<MoveEvaluation> = root_node
            .get_legal_moves()
            .into_iter()
            .map(|lmove| MoveEvaluation(lmove, self.root.evaluate_mvv_lva(&lmove)))
            .collect();

        legal_moves.sort_unstable_by_key(|eval| eval.1);

        let mut evaluations: Vec<MoveEvaluation> = legal_moves
            .into_par_iter()
            .map(|eval| {
                println!("info currmove {}", eval.0.to_algebraic());
                MoveEvaluation(
                    eval.0,
                    match self.root.turn {
                        Turn::White => root_node.apply_move(eval.0).quiescence_eval(
                            self.max_depth,
                            self.deadline,
                            i32::MIN,
                            i32::MAX,
                        ),
                        Turn::Black => -root_node.apply_move(eval.0).quiescence_eval(
                            self.max_depth,
                            self.deadline,
                            i32::MIN,
                            i32::MAX,
                        ),
                    },
                )
            })
            .collect();

        evaluations.sort_unstable_by_key(|eval| eval.1);

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

        let mut legal_moves: Vec<MoveEvaluation> = self
            .get_legal_moves()
            .into_iter()
            .map(|lmove| {
                MoveEvaluation(
                    lmove,
                    self.game.evaluate_mvv_lva(&lmove)
                        * match KillerCache::is_killer(depth, lmove) {
                            true => 100,
                            false => 1,
                        },
                )
            })
            .collect();

        legal_moves.sort_unstable_by_key(|eval| eval.1);

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
                for eval in legal_moves {
                    let value =
                        self.apply_move(eval.0)
                            .quiescence_eval(depth - 1, deadline, alpha, beta);
                    highest_eval = cmp::max(value, highest_eval);
                    alpha = cmp::max(highest_eval, alpha);
                    if beta <= alpha {
                        // Record as killer if not a capture or in check
                        if eval.0.dest & self.game.board.black[6] != 0
                            && self.get_black_vision()[6] & self.game.board.white[5] == 0
                        {
                            KillerCache::add_killer(depth, eval.0);
                        }
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
                for eval in legal_moves {
                    let value =
                        self.apply_move(eval.0)
                            .quiescence_eval(depth - 1, deadline, alpha, beta);
                    lowest_eval = cmp::min(value, lowest_eval);
                    beta = cmp::min(lowest_eval, beta);
                    if beta <= alpha {
                        // Record as killer if not a capture or in check
                        if eval.0.dest & self.game.board.white[6] != 0
                            && self.get_white_vision()[6] & self.game.board.black[5] == 0
                        {
                            KillerCache::add_killer(depth, eval.0);
                        }
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

#[derive(Debug, PartialEq, Eq, Clone)]
struct MoveEvaluation<'a>(&'a SimpleMove, i32);
