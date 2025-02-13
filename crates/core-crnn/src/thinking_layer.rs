use crate::activation_function::ActivationFunction;
use crate::neuron::Neuron;
use anyhow::bail;
use rand::{rng, Rng};

pub struct ThinkingLayer {
    input_count: usize,
    internal_count: usize,
    output_count: usize,

    neurons: Vec<Neuron>,
    neuron_states: Vec<f64>,
    weights: Vec<f64>,

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
            input_count,
            internal_count,
            output_count,
            neurons: (0..internal_count)
                .map(|_| Neuron::random(&mut rng(), activation_function.clone()))
                .collect(),
            neuron_states: vec![0.0; internal_count],
            weights: (0..internal_count * (internal_count - 1))
                .map(|_| rng().random::<f64>() * 2.0 - 1.0)
                .collect(),
            internal_tick: 1,
        })
    }

    pub fn tick(&mut self, input: Option<Vec<f64>>) {
        if self.internal_tick == 0 {
            self.internal_tick = 1;
        }

        if let Some(input) = input {
            self.neuron_states.splice(0..self.input_count, input);
        }

        let exclude_input_range = self.input_count..self.internal_count;

        let new_states = self.neurons[exclude_input_range.clone()]
            .iter()
            .enumerate()
            .map(|(index, neuron)| {
                let index = index + self.input_count;
                if self.internal_tick % neuron.delay() == 0 {
                    neuron.activate(index, &self)
                } else {
                    self.neuron_states[index]
                }
            })
            .collect::<Vec<_>>();

        self.neuron_states.splice(exclude_input_range, new_states);

        self.internal_tick = self.internal_tick.overflowing_add(1).0;
    }

    pub fn output(&self) -> Vec<f64> {
        let output_range = self.internal_count - self.output_count..self.internal_count;
        self.neuron_states[output_range].to_vec()
    }

    pub fn input_count(&self) -> usize {
        self.input_count
    }

    pub fn internal_count(&self) -> usize {
        self.internal_count
    }

    pub fn output_count(&self) -> usize {
        self.output_count
    }

    pub fn neurons(&self) -> &[Neuron] {
        &self.neurons
    }

    pub fn neuron_states(&self) -> &[f64] {
        &self.neuron_states
    }

    pub fn weights(&self) -> &[f64] {
        &self.weights
    }
}
