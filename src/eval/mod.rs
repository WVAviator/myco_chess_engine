use bishops::BishopEval;
use development::DevelopmentEval;
use king_safety::KingSafetyEval;
use piece::PieceEval;
use rooks::RookEval;

use crate::cgame::game::{Game, Turn};

mod bishops;
mod center;
mod development;
mod king_safety;
mod piece;
mod rooks;

pub trait Eval: BishopEval + RookEval + DevelopmentEval + KingSafetyEval + PieceEval {
    fn evaluate_position(&self) -> i32;
}

impl Eval for Game {
    fn evaluate_position(&self) -> i32 {
        let mut value = 0;
        value += self.calculate_piece_value();
        value += self.calculate_bishop_value();
        value += self.calculate_rook_value();
        value += self.calculate_development_value();
        value += self.calculate_king_safety_value();
        value += self.calculate_center_value();

        value += match self.turn {
            Turn::White => 24,
            Turn::Black => -24,
        };

        value
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn evaluates_starting_position() {
        let game = Game::new_default();
        let eval = game.evaluate_position();

        assert!(eval > 0); // White should always start slightly ahead
    }
}
