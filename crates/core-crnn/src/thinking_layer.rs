use crate::activation_function::ActivationFunction;
use anyhow::bail;
use rand::{random_iter, rng, Rng};

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
        let mut rng = rng();
        Ok(Self {
            input_size: input_count,
            internal_size: internal_count,
            output_size: output_count,
            genome: (0..internal_count)
                .flat_map(|_| {
                    let mut data = vec![
                        rng.random::<f64>() / 10.0 - 0.05, // Random bias from -0.1 to 0.1 (1xf64)
                        1.0 + rng.random::<f64>() * 2.0,   // Random delay from 1 to 3. (1xf64)
                    ];
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

    pub fn bias(&self) -> Vec<&f64> {
        self.genome
            .iter()
            .skip(0) // Bias is the first element
            .step_by(self.neuron_data_length())
            .collect()
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
        let mut weights = self.input_weights(neuron_index).iter();
        let states = self.neuron_states();

        let mut sum = 0.0;

        for state in &states[..neuron_index] {
            sum += weights.next().unwrap() * state;
        }

        for state in &states[(neuron_index + 1)..] {
            sum += weights.next().unwrap() * state;
        }

        let bias = *self.bias()[neuron_index];

        self.activation_function.apply(sum + bias)
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
