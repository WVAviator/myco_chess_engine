use crate::game::game::Game;
use crate::ml::model::MycoCNNPredictor;

const NN_EVALUATION_WEIGHT: f32 = 150.0;

pub trait NeuralNetEval {
    fn calculate_neural_network_evaluation(&self) -> i32;
}

impl NeuralNetEval for Game {
    fn calculate_neural_network_evaluation(&self) -> i32 {
        let predictor = MycoCNNPredictor::get();

        (predictor.predict(self).unwrap_or_else(|_| {
            println!("failed to get value from nn model");
            0.0
        }) * NN_EVALUATION_WEIGHT) as i32
    }
}
