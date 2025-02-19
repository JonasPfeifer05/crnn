use crate::activation_function::ActivationFunction;
use anyhow::bail;
use rand::{random_iter, random_range};
use std::simd::num::SimdFloat;
use std::simd::{f64x4, Simd};

#[derive(Debug, Clone)]
pub struct ThinkingLayer {
    input_size: usize,
    internal_size: usize,
    output_size: usize,

    activation_function: ActivationFunction,

    genome: Vec<f64>,
    neuron_states: Vec<f64>,

    internal_tick: usize,
}

impl ThinkingLayer {
    pub fn new(
        input_count: usize,
        internal_count: usize,
        output_count: usize,
        activation_function: ActivationFunction,
    ) -> anyhow::Result<Self> {
        if input_count + output_count > internal_count {
            bail!("Cannot create thinking layer with fewer neurons than input and output values")
        }

        Ok(Self {
            input_size: input_count,
            internal_size: internal_count,
            output_size: output_count,
            genome: (0..internal_count)
                .flat_map(|_| {
                    let mut data = vec![random_range(-0.1..0.1), random_range(1.0..3.0)];
                    data.extend(
                        // Random weights in from -0.1 to 0.1 (n-1xf64)
                        random_iter::<f64>()
                            .take(internal_count - 1)
                            .map(|x| x / 10.0 - 0.05)
                            .collect::<Vec<_>>(),
                    );
                    data
                })
                .collect(),
            neuron_states: vec![0.0; internal_count],
            activation_function,
            internal_tick: 1,
        })
    }

    pub fn tick(&mut self, input: Option<Vec<f64>>) {
        if self.internal_tick == 0 {
            self.internal_tick = 1;
        }

        if let Some(input) = input {
            self.neuron_states.splice(0..self.input_size, input);
        }

        let exclude_input_range = self.input_size..self.internal_size;

        let new_states: Vec<_> = self.delays()[exclude_input_range.clone()]
            .into_iter()
            .enumerate()
            .map(|(neuron_index, delay)| {
                let neuron_index = neuron_index + self.input_size;
                if self.internal_tick % delay.round().max(1.0) as usize == 0 {
                    self.activate_neuron(neuron_index)
                } else {
                    self.neuron_states[neuron_index]
                }
            })
            .collect();

        self.neuron_states.splice(exclude_input_range, new_states);

        self.internal_tick = self.internal_tick.overflowing_add(1).0;
    }

    pub fn output(&self) -> Vec<f64> {
        let output_range = self.internal_size - self.output_size..self.internal_size;
        self.neuron_states[output_range].to_vec()
    }

    pub fn input_size(&self) -> usize {
        self.input_size
    }

    pub fn internal_size(&self) -> usize {
        self.internal_size
    }

    pub fn output_size(&self) -> usize {
        self.output_size
    }

    pub fn neuron_states(&self) -> &[f64] {
        &self.neuron_states
    }

    pub fn bias(&self, index: usize) -> f64 {
        self.genome[index * self.neuron_data_length()] // Bias is the first element
    }

    pub fn delays(&self) -> Vec<&f64> {
        self.genome
            .iter()
            .skip(1) // delay is the second element
            .step_by(self.neuron_data_length())
            .collect()
    }

    pub fn input_weights(&self, neuron_index: usize) -> &[f64] {
        let start = 2 + self.neuron_data_length() * neuron_index;
        let end = self.neuron_data_length() * (neuron_index + 1);
        &self.genome[start..end]
    }

    fn neuron_data_length(&self) -> usize {
        2 + self.internal_size - 1
    }

    fn activate_neuron(&self, neuron_index: usize) -> f64 {
        let mut weights = self.input_weights(neuron_index);
        let states = self.neuron_states();

        let mut sum = 0.0;

        sum += Self::simd_dot_product(&states[..neuron_index], &weights[..neuron_index]);
        sum += Self::simd_dot_product(&states[(neuron_index + 1)..], &weights[neuron_index..]);

        let bias = self.bias(neuron_index);

        self.activation_function.apply(sum + bias)
    }

    fn simd_dot_product(states: &[f64], weights: &[f64]) -> f64 {
        let mut sum = f64x4::splat(0.0);
        let chunk_size = 4;

        let len = states.len().min(weights.len()); // Ensure both slices are the same length
        let chunks = len / chunk_size * chunk_size; // Align to SIMD width

        for i in (0..chunks).step_by(chunk_size) {
            let state_chunk = Simd::from_slice(&states[i..i + chunk_size]);
            let weight_chunk = Simd::from_slice(&weights[i..i + chunk_size]);
            sum += state_chunk * weight_chunk;
        }

        let mut scalar_sum: f64 = sum.reduce_sum(); // Reduce SIMD lanes

        // Handle remaining elements (tail processing)
        for i in chunks..len {
            scalar_sum += states[i] * weights[i];
        }

        scalar_sum
    }

    pub fn genome(&self) -> &[f64] {
        &self.genome
    }

    pub fn genome_mut(&mut self) -> &mut [f64] {
        &mut self.genome
    }

    pub fn set_genome(&mut self, genome: Vec<f64>) {
        self.genome = genome;
    }

    pub fn activation_function(&self) -> &ActivationFunction {
        &self.activation_function
    }
}
