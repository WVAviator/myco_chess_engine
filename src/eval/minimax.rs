use std::{
    cmp,
    collections::HashMap,
    sync::{OnceLock, RwLock},
};

use rayon::prelude::*;

use crate::{
    game::game::{Game, Turn},
    hash::zobrist::ZobristHash,
    movegen::MoveGen,
    moves::simple_move::SimpleMove,
};

use super::Eval;

static EVAL_MAP: OnceLock<RwLock<HashMap<u64, i32>>> = OnceLock::new();

fn get_eval(zobrist: u64) -> Option<i32> {
    let map = EVAL_MAP
        .get_or_init(|| RwLock::new(HashMap::new()))
        .read()
        .unwrap();
    map.get(&zobrist).cloned()
}

fn write_eval(zobrist: u64, eval: i32) {
    let mut map = EVAL_MAP
        .get_or_init(|| RwLock::new(HashMap::new()))
        .write()
        .unwrap();
    map.insert(zobrist, eval);
}

fn get_or_write_eval(game: &Game) -> i32 {
    let zobrist = game.zobrist();
    get_eval(zobrist).unwrap_or_else(|| {
        let eval = game.evaluate_position();
        write_eval(zobrist, eval);
        eval
    })
}

pub trait MinimaxEval: Eval {
    fn minimax_eval(&self, depth: u8, alpha: i32, beta: i32) -> i32;
}

impl MinimaxEval for Game {
    fn minimax_eval(&self, depth: u8, alpha: i32, beta: i32) -> i32 {
        if self.board.king(&self.turn) == 0 {
            return match self.turn {
                Turn::White => -200000,
                Turn::Black => 200000,
            };
        }

        if depth == 0 {
            return get_or_write_eval(self);
        }

        let mut alpha = alpha;
        let mut beta = beta;

        let pseudolegal_moves = self.generate_pseudolegal_moves();

        let mut first_pass_evaluations = pseudolegal_moves
            .par_iter()
            .map(|lmove| MoveEvaluation(&self, get_or_write_eval(&self.apply_move(lmove))))
            .collect::<Vec<MoveEvaluation>>();

        match self.turn {
            Turn::White => {
                let mut highest_val = i32::MIN;
                first_pass_evaluations.sort_unstable_by(|a, b| b.cmp(a));

                for eval in first_pass_evaluations
                    .iter().take(3 + depth as usize)
                {
                    let value = eval.0
                        .minimax_eval(depth - 1, alpha, beta);
                    highest_val = cmp::max(value, highest_val);
                    alpha = cmp::max(highest_val, alpha);
                    if beta <= alpha {
                        break;
                    }
                }
                highest_val
            }
            Turn::Black => {
                let mut lowest_val = i32::MAX;
                first_pass_evaluations.sort_unstable();

                for eval in first_pass_evaluations
                    .iter().take(3 + depth as usize)
                {
                    let value = eval.0                        
                        .minimax_eval(depth - 1, alpha, beta);
                    lowest_val = cmp::min(value, lowest_val);
                    beta = cmp::min(lowest_val, beta);
                    if beta <= alpha {
                        break;
                    }
                }
                lowest_val
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct MoveEvaluation<'a>(&'a Game, i32);

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
