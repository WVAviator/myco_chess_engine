use std::sync::OnceLock;

use tch::nn::Module;
use tch::CModule;

use crate::cgame::game::{Game, Turn};

use super::tensor::BoardTensor;

const CENTIPAWNS_NORMALIZATION_FACTOR: f32 = 24000.0;

static EMBEDDED_MODEL: &[u8] = include_bytes!("../../ml/chess_eval_model.pt");
static MYCO_PREDICTOR: OnceLock<MycoCNNPredictor> = OnceLock::new();

pub struct MycoCNNPredictor {
    model: CModule,
}

impl MycoCNNPredictor {
    pub fn new(model_path: &str) -> Self {
        let model = tch::CModule::load(model_path).expect("unable to load cmodule");
        Self { model }
    }

    pub fn get() -> &'static Self {
        MYCO_PREDICTOR.get_or_init(|| {
            let tmp_path = "/tmp/myco_model_tmp.pt";
            std::fs::write(tmp_path, EMBEDDED_MODEL)
                .expect("Failed to write embedded model to temporary file");
            Self::new(&tmp_path)
        })
    }

    pub fn predict(&self, game: &Game) -> Result<i32, anyhow::Error> {
        let board_tensor = BoardTensor::from(game);
        let output = self.model.forward(&board_tensor);
        let evaluation = output.double_value(&[0]) as f32;

        Ok(match game.turn {
            Turn::White => (evaluation * CENTIPAWNS_NORMALIZATION_FACTOR) as i32,
            Turn::Black => (evaluation * -CENTIPAWNS_NORMALIZATION_FACTOR) as i32,
        })
    }
}
