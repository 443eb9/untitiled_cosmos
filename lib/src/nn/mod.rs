use rand::Rng;
use rand_distr::Uniform;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

pub mod inferencing;
mod training;

pub fn activate(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

pub fn deriv_activate(x: f64) -> f64 {
    let a = activate(x);
    a * (1.0 - a)
}

pub struct NeuralNetwork {
    layer_count: usize,
    layers: Vec<Layer>,
    layer_data: Vec<LayerData>,
}

impl NeuralNetwork {
    pub fn new(
        input: usize,
        output: usize,
        hidden_layer_sizes: &[usize],
        rng: &mut impl Rng,
    ) -> Self {
        let mut layers = Vec::with_capacity(hidden_layer_sizes.len());
        let mut layer_data = Vec::with_capacity(hidden_layer_sizes.len());
        let mut prev_size = input;

        let mut i = 1;

        for &size in hidden_layer_sizes {
            layers.push(Layer::new(prev_size, size, rng, i));
            layer_data.push(LayerData::new(prev_size, size, i));
            prev_size = size;
            i += 1;
        }

        layers.push(Layer::new(prev_size, output, rng, i));
        layer_data.push(LayerData::new(prev_size, output, i));

        NeuralNetwork {
            layer_count: layers.len(),
            layers,
            layer_data,
        }
    }
}

pub struct Layer {
    index: usize,
    inputs: usize,
    outputs: usize,
    weights: Vec<f64>,
    biases: Vec<f64>,
}

impl Layer {
    pub fn new(inputs: usize, outputs: usize, rng: &mut impl Rng, index: usize) -> Self {
        let distr = Uniform::new(0., 1.);
        Layer {
            index,
            inputs,
            outputs,
            weights: (0..inputs * outputs)
                .into_iter()
                .map(|_| rng.sample(distr))
                .collect(),
            biases: vec![0.0; outputs],
        }
    }

    pub fn think(&self, inputs: &[f64], output: &mut LayerData) {
        output
            .weighted_inputs
            .par_iter_mut()
            .zip(output.activations.par_iter_mut())
            .enumerate()
            .for_each(|(output_index, (weighted_input, activation))| {
                *weighted_input = self.biases[output_index];
                self.weight_slice(output_index).iter().for_each(|weight| {
                    *weighted_input += weight * inputs[output_index];
                });
                *activation = activate(*weighted_input);
            });
    }

    pub fn weight(&self, input: usize, output: usize) -> f64 {
        self.weights[output * self.inputs + input]
    }

    pub fn weight_slice(&self, output: usize) -> &[f64] {
        &self.weights[output * self.inputs..(output + 1) * self.inputs]
    }
}

pub struct LayerData {
    index: usize,
    /// z_n = a_{n-1} * w_n + b_n
    weighted_inputs: Vec<f64>,
    /// a_n = Activation(z_n)
    activations: Vec<f64>,
    /// ∂c/∂a_n * ∂a_n/∂z_n
    backprop: Vec<f64>,
    grad_w: Vec<f64>,
    grad_b: Vec<f64>,
}

impl LayerData {
    pub fn new(inputs: usize, outputs: usize, index: usize) -> Self {
        LayerData {
            index,
            weighted_inputs: vec![0.; outputs],
            activations: vec![0.; outputs],
            backprop: vec![0.; outputs],
            grad_w: vec![0.; outputs * inputs],
            grad_b: vec![0.; outputs],
        }
    }

    pub fn flatten_index(&self, input: usize, output: usize) -> usize {
        output * self.activations.len() + input
    }
}

pub struct DataPoint {
    pub inputs: Vec<f64>,
    pub targets: Vec<f64>,
}

pub fn print_slice(slice: &[f64]) {
    slice.iter().for_each(|x| print!("{:.2} ", x));
    println!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_network() {
        let mut rng = rand::thread_rng();
        let mut network = NeuralNetwork::new(2, 2, &[2], &mut rng);
        let data = gen_test_data();

        network
            .layers
            .iter()
            .enumerate()
            .for_each(|(index, layer)| {
                println!("=========================");
                println!("Layer {}", index);
                println!("Weights:");
                print_slice(&layer.weights);
                println!("Biases:");
                print_slice(&layer.biases);
                println!("=========================");
            });

        network.learn(&data, 100, 0.01);

        network
            .layers
            .iter()
            .enumerate()
            .for_each(|(index, layer)| {
                println!("=========================");
                println!("Layer {}", index);
                println!("Weights:");
                print_slice(&layer.weights);
                println!("Biases:");
                print_slice(&layer.biases);
                println!("=========================");
            });
        let func = |x: f64| -x.powi(3) * 0.5 + 1. + x.powi(2) - 1.;

        (0..10).into_iter().for_each(|_| {
            let x = rng.gen_range(0f64..1f64);
            let y = rng.gen_range(0f64..1f64);
            let result = network.inference(&[x, y]);
            let is_poison = if y < func(x) { 1. } else { 0. };
            println!(
                "{:.2} {:.2} -> {:.2} {:.2} [{:.2}, {:.2}]",
                x,
                y,
                result[0],
                result[1],
                is_poison,
                1. - is_poison
            );
        });
    }

    fn gen_test_data() -> Vec<DataPoint> {
        let mut rng = rand::thread_rng();
        let func = |x: f64| -x.powi(3) * 0.5 + 1. + x.powi(2) - 1.;

        (0..100)
            .into_iter()
            .map(|_| {
                let x = rng.gen_range(0f64..1f64);
                let y = rng.gen_range(0f64..1f64);
                let is_poison = if y < func(x) { 1. } else { 0. };
                DataPoint {
                    inputs: vec![x, y],
                    targets: vec![is_poison, 1. - is_poison],
                }
            })
            .collect()
    }
}
