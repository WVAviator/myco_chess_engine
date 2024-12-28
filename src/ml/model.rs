use std::{env, path::PathBuf, sync::OnceLock};

use tch::nn::Module;
use tch::CModule;

use crate::game::game::{Game, Turn};

use super::tensor::BoardTensor;

static MYCO_PREDICTOR: OnceLock<MycoCNNPredictor> = OnceLock::new();

pub struct MycoCNNPredictor {
    model: CModule,
}

impl MycoCNNPredictor {
    pub fn new(model_path: PathBuf) -> Self {
        let model = tch::CModule::load(model_path).expect("unable to load cmodule");
        Self { model }
    }

    pub fn get() -> &'static Self {
        MYCO_PREDICTOR.get_or_init(|| Self::new(get_model_path()))
    }

    pub fn predict(&self, game: &Game) -> Result<f32, anyhow::Error> {
        let board_tensor = BoardTensor::from(game);
        let output = self.model.forward(&board_tensor);
        let evaluation = output.double_value(&[0]) as f32;

        Ok(match game.turn {
            Turn::White => evaluation,
            Turn::Black => -evaluation,
        })
    }
}

fn get_model_path() -> PathBuf {
    if cfg!(debug_assertions) {
        PathBuf::from("resources/myco_eval_model.pt")
    } else {
        let mut exe_path = env::current_exe().expect("failed to get current executable path");
        exe_path.pop();
        exe_path.push("resources/myco_eval_model.pt");
        exe_path
    }
}
