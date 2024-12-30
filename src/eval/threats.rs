use crate::{
    game::game::{Game, Turn},
    movegen::MoveGen,
    moves::simple_move::SimpleMove,
};

pub trait ThreatEval {
    fn evaluate_threats(&self, lmove: &SimpleMove) -> i32;
}

impl ThreatEval for Game {
    fn evaluate_threats(&self, lmove: &SimpleMove) -> i32 {
        match self.turn {
            Turn::White => {
                let black_vision = self.generate_vision(&Turn::Black);
            }
            Turn::Black => {
                let white_vision = self.generate_vision(&Turn::White);
            }
        }
        0
    }
}
