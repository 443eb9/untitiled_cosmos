use bevy::math::DVec2;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use super::NeuralNetwork;

impl NeuralNetwork {
    pub fn inference(&mut self, data: &[f64]) -> &[f64] {
        self.layers[0].think(&data, &mut self.layer_data[0]);
        for i in 1..self.layer_count {
            let [prev, cur, ..] = &mut self.layer_data[i - 1..] else {
                unreachable!()
            };
            self.layers[i].think(&prev.activations, cur);
        }
        &self.layer_data[self.layer_count - 1].activations
    }
}

pub fn inference_initial_vel(seed: u64, galaxy: &str, body: &str) -> DVec2 {
    DVec2::ZERO
}
