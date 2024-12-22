use tch::nn::{self, Module, VarStore};
use tch::{Device, Tensor};

use crate::cgame::game::{Game, Turn};

use super::tensor::BoardTensor;

const CENTIPAWNS_NORMALIZATION_FACTOR: f32 = 24000.0;

pub struct MycoCNNPredictor<'a> {
    model: MycoCNNModel<'a>,
    _vs: VarStore,
}

impl<'a> MycoCNNPredictor<'a> {
    pub fn new(model_path: &str, device: Device) -> Result<Self, anyhow::Error> {
        let mut vs = VarStore::new(device);
        let model = MycoCNNModel::new(&vs.root());
        vs.load(model_path)?;
        Ok(Self { model, _vs: vs })
    }

    pub fn predict(&self, game: &Game) -> i32 {
        let board_tensor = BoardTensor::from(game);
        let output = self.model.forward(&board_tensor);
        let evaluation = output.double_value(&[0]) as f32;

        match game.turn {
            Turn::White => (evaluation * CENTIPAWNS_NORMALIZATION_FACTOR) as i32,
            Turn::Black => (evaluation * -CENTIPAWNS_NORMALIZATION_FACTOR) as i32,
        }
    }
}

#[derive(Debug)]
struct MycoCNNModel<'a> {
    conv1: nn::Conv2D,
    conv2: nn::Conv2D,
    fc1: nn::Linear,
    fc2: nn::Linear,
    relu: nn::Func<'a>,
    pool: nn::Func<'a>,
}

impl<'a> MycoCNNModel<'a> {
    pub fn new(vs: &nn::Path) -> Self {
        let conv1 = nn::conv2d(
            vs,
            7,
            32,
            3,
            nn::ConvConfig {
                padding: 1,
                ..Default::default()
            },
        );
        let conv2 = nn::conv2d(
            vs,
            32,
            64,
            3,
            nn::ConvConfig {
                padding: 1,
                ..Default::default()
            },
        );
        let fc1 = nn::linear(vs, 64 * 4 * 4, 128, Default::default());
        let fc2 = nn::linear(vs, 128, 1, Default::default());
        let relu = nn::func(|x| x.relu());
        let pool = nn::func(|x| x.max_pool2d_default(2));
        Self {
            conv1,
            conv2,
            fc1,
            fc2,
            relu,
            pool,
        }
    }
}

impl<'a> Module for MycoCNNModel<'a> {
    fn forward(&self, x: &Tensor) -> Tensor {
        let x = x.apply(&self.conv1).apply(&self.relu);
        let x = x.apply(&self.conv2).apply(&self.relu).apply(&self.pool);
        let x = x.flatten(1, -1);
        let x = x.apply(&self.fc1).apply(&self.relu);
        x.apply(&self.fc2)
    }
}
