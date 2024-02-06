use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use super::{DataPoint, Layer, LayerData, NeuralNetwork};

fn cross_entropy(activations: f64, targets: f64) -> f64 {
    -targets * activations.ln() - (1. - targets) * (1. - activations).ln()
}

fn deriv_cross_entropy(activations: f64, targets: f64) -> f64 {
    (-activations + targets) / (activations * (1. - activations))
}

impl NeuralNetwork {
    pub fn learn(&mut self, data: &[DataPoint], batch_size: usize, learn_rate: f64) {
        for batch in data.chunks(batch_size) {
            self.learn_batch(batch, learn_rate);
        }
    }

    fn learn_batch(&mut self, batch: &[DataPoint], learn_rate: f64) {
        batch.iter().for_each(|data| {
            self.think(data);
            self.calc_backprop_cache(data);
            self.calc_grad(data);
        });

        self.apply_and_clear_grad(learn_rate);
    }

    fn think(&mut self, data: &DataPoint) {
        self.layers[0].think(&data.inputs, &mut self.layer_data[0]);
        for i in 1..self.layer_count {
            let [prev, cur, ..] = &mut self.layer_data[i - 1..] else {
                unreachable!()
            };
            self.layers[i].think(&prev.activations, cur);
        }
    }

    fn calc_backprop_cache(&mut self, data: &DataPoint) {
        self.layer_data[self.layer_count - 1].calc_backprop_cache_output(&data.targets);
        for i in (0..self.layer_count - 1).rev() {
            let [cur, next, ..] = &mut self.layer_data[i..] else {
                unreachable!()
            };
            cur.calc_backprop_cache_hidden(&self.layers[i + 1], &next);
        }
    }

    fn calc_grad(&mut self, data: &DataPoint) {
        self.layer_data[0].calc_grad(&data.inputs);
        for i in 1..self.layer_count {
            let [prev, cur, ..] = &mut self.layer_data[i - 1..] else {
                unreachable!()
            };
            cur.calc_grad(&prev.activations);
        }
    }

    fn apply_and_clear_grad(&mut self, learn_rate: f64) {
        self.layers
            .par_iter_mut()
            .zip(self.layer_data.par_iter_mut())
            .for_each(|(layer, data)| {
                data.apply_and_clear_grad(layer, learn_rate);
            });
    }
}

impl LayerData {
    pub fn calc_backprop_cache_output(&mut self, targets: &[f64]) {
        self.backprop
            .par_iter_mut()
            .enumerate()
            .for_each(|(neuron_index, backprop)| {
                let cost_deriv =
                    deriv_cross_entropy(self.activations[neuron_index], targets[neuron_index]);
                let acti_deriv = super::deriv_activate(self.weighted_inputs[neuron_index]);
                *backprop = cost_deriv * acti_deriv;
            });
    }

    pub fn calc_backprop_cache_hidden(&mut self, next: &Layer, next_data: &LayerData) {
        self.backprop
            .par_iter_mut()
            .enumerate()
            .for_each(|(neuron_index, backprop)| {
                *backprop = next_data
                    .backprop
                    .iter()
                    .map(|next_backprop| *next_backprop * next.weights[neuron_index])
                    .sum::<f64>()
                    * super::deriv_activate(self.weighted_inputs[neuron_index]);
            });
    }

    pub fn calc_grad(&mut self, inputs: &[f64]) {
        for output in 0..self.activations.len() {
            for input in 0..inputs.len() {
                self.grad_w[output * inputs.len() + input] += inputs[input] * self.backprop[output];
                self.grad_b[output] += self.backprop[output];
            }
        }
    }

    pub fn apply_and_clear_grad(&mut self, layer: &mut Layer, learn_rate: f64) {
        self.grad_w
            .par_iter_mut()
            .zip(layer.weights.par_iter_mut())
            .for_each(|(grad, weight)| {
                *weight -= learn_rate * *grad;
                *grad = 0.;
            });

        self.grad_b
            .par_iter_mut()
            .zip(layer.biases.par_iter_mut())
            .for_each(|(grad, bias)| {
                *bias -= learn_rate * *grad;
                *grad = 0.;
            });
    }
}
