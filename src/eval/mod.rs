use bishops::BishopEval;
use center::CenterEval;
use development::DevelopmentEval;
use king_safety::KingSafetyEval;
use knights::KnightEval;
use piece::PieceEval;
use rooks::RookEval;

use crate::cgame::game::Game;

mod bishops;
mod center;
mod development;
mod king_safety;
mod knights;
mod piece;
mod rooks;

pub mod minimax;

pub trait Eval:
    BishopEval + RookEval + DevelopmentEval + KingSafetyEval + PieceEval + CenterEval + KnightEval
{
    fn evaluate_position(&self) -> i32;
}

impl Eval for Game {
    fn evaluate_position(&self) -> i32 {
        let mut value = 0;

        value += self.calculate_piece_value();
        // value += self.calculate_bishop_value();
        // value += self.calculate_rook_value();
        // value += self.calculate_development_value();
        value += self.calculate_king_safety_value();
        // value += self.calculate_center_value();
        // value += self.calculate_knights_value();

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
