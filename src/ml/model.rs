use anyhow::{anyhow, Context};
use tch::nn::Module;
use tch::{CModule, Device};

use crate::cgame::game::{Game, Turn};

use super::tensor::BoardTensor;

const CENTIPAWNS_NORMALIZATION_FACTOR: f32 = 24000.0;

pub struct MycoCNNPredictor {
    model: CModule,
    // _vs: VarStore,
}

impl MycoCNNPredictor {
    pub fn new(model_path: &str, device: Device) -> Result<Self, anyhow::Error> {
        // let mut vs = VarStore::new(device);
        let model = tch::CModule::load(model_path)
            .with_context(|| anyhow!("unable to load cmodule from model path: {}", model_path))?;
        // vs.load(model_path).with_context(|| {
        //     anyhow!(
        //         "unable to load variable store from model path: {}",
        //         model_path
        //     )
        // })?;
        Ok(Self {
            model,
            // _vs: vs
        })
    }

    pub fn predict(&self, game: &Game) -> Result<i32, anyhow::Error> {
        let board_tensor = BoardTensor::from(game);
        // let output = self.model.forward_ts(&[&*board_tensor])?;
        let output = self.model.forward(&board_tensor);
        let evaluation = output.double_value(&[0]) as f32;

        Ok(match game.turn {
            Turn::White => (evaluation * CENTIPAWNS_NORMALIZATION_FACTOR) as i32,
            Turn::Black => (evaluation * -CENTIPAWNS_NORMALIZATION_FACTOR) as i32,
        })
    }
}
