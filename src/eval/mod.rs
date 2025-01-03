use king_safety::KingSafetyEval;
use pawn_structure::PawnStructureEval;
use piece::PieceEval;

use crate::game::game::Game;

mod king_safety;

#[cfg(feature = "pytorch")]
pub mod nn;

pub mod mvvlva;
mod pawn_structure;
pub mod piece;

pub mod minimax;

pub trait Eval {
    fn evaluate_position(&self) -> i32;
    fn evaluate_position_ml(&self) -> i32;
}

impl Eval for Game {
    fn evaluate_position(&self) -> i32 {
        let mut value = 0;

        value += self.calculate_piece_value();
        value += self.calculate_king_safety_value();
        value += self.calculate_pawn_structure_value();

        value
    }

    fn evaluate_position_ml(&self) -> i32 {
        // Separate because it's slower

        #[allow(unused_mut)]
        let mut value = 0;

        #[cfg(feature = "pytorch")]
        {
            use nn::NeuralNetEval;
            value += self.calculate_neural_network_evaluation();
        }

        value
    }
}
