use crate::cgame::game::Game;

pub trait CenterEval {
    fn calculate_center_value(&self) -> i32;
}

impl CenterEval for Game {
    fn calculate_center_value(&self) -> i32 {}
}
