use std::simd::{
    num::{SimdInt, SimdUint},
    Simd,
};

use crate::{
    game::game::{Game, Turn},
    movegen::MoveGen,
    moves::simple_move::SimpleMove,
    util::simd::SimdCountOnes,
};

const PIECE_VALUES: Simd<u64, 8> = Simd::from_array([50, 250, 150, 160, 450, 20000, 0, 0]);

pub trait ThreatEval {
    fn evaluate_threats(&self, lmove: &SimpleMove) -> i32;
}

impl ThreatEval for Game {
    fn evaluate_threats(&self, lmove: &SimpleMove) -> i32 {
        let orig = Simd::splat(lmove.orig);
        let dest = Simd::splat(lmove.dest);
        match self.turn {
            Turn::White => {
                let black_vision = self.generate_vision(&Turn::Black);

                let mover_value =
                    ((self.board.white & orig).count_ones() * PIECE_VALUES).cast::<i32>();
                let defense_value =
                    ((black_vision & dest).count_ones() * PIECE_VALUES).cast::<i32>();

                (defense_value - mover_value).reduce_sum()
            }
            Turn::Black => {
                let white_vision = self.generate_vision(&Turn::White);

                let mover_value =
                    ((self.board.black & orig).count_ones() * PIECE_VALUES).cast::<i32>();
                let defense_value =
                    ((white_vision & dest).count_ones() * PIECE_VALUES).cast::<i32>();

                (defense_value - mover_value).reduce_sum()
            }
        }
    }
}
