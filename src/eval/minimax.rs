use rayon::prelude::*;

use crate::{
    cgame::game::{Game, Turn},
    movegen::MoveGen,
};

use super::Eval;

pub trait MinimaxEval: Eval {
    fn minimax_eval(&self, depth: u8) -> i32;
}

impl MinimaxEval for Game {
    fn minimax_eval(&self, depth: u8) -> i32 {
        if depth == 0 {
            return self.evaluate_position();
        }

        match self.turn {
            Turn::White => self
                .generate_legal_moves()
                .into_par_iter()
                .map(|m| self.apply_move(&m).minimax_eval(depth - 1))
                .max()
                .unwrap_or(-200000),
            Turn::Black => self
                .generate_legal_moves()
                .into_par_iter()
                .map(|m| self.apply_move(&m).minimax_eval(depth - 1))
                .min()
                .unwrap_or(200000),
        }
    }
}
