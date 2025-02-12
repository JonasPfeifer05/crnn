use crate::activation_function::ActivationFunction;
use crate::neuron::Neuron;
use anyhow::bail;
use rand::{rng, Rng};

pub struct ThinkingLayer<Activation: ActivationFunction> {
    pub input_count: usize,
    pub internal_count: usize,
    pub output_count: usize,

    pub neurons: Vec<Neuron<Activation>>,
    pub neuron_states: Vec<f64>,
    pub weights: Vec<f64>,

    pub internal_tick: usize,
}

impl<Activation: ActivationFunction> ThinkingLayer<Activation> {
    pub fn new(
        input_count: usize,
        internal_count: usize,
        output_count: usize,
    ) -> anyhow::Result<Self> {
        if input_count + output_count > internal_count {
            bail!("Cannot create thinking layer with fewer neurons than input and output values")
        }
        Ok(Self {
            input_count,
            internal_count,
            output_count,
            neurons: (0..internal_count)
                .map(|_| Neuron::random(&mut rng()))
                .collect(),
            neuron_states: vec![0.0; internal_count],
            weights: (0..internal_count * (internal_count - 1))
                .map(|_| rng().random::<f64>() * 2.0 - 1.0)
                .collect(),
            internal_tick: 1,
        })
    }

    pub fn tick(&mut self, input: Option<Vec<f64>>) -> Vec<f64> {
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

        self.neuron_states.splice(
            exclude_input_range,
            new_states
        );
        
        self.internal_tick = self.internal_tick.overflowing_add(self.internal_count).0;
        
        let output_range = self.internal_count - self.output_count..self.internal_count;
        self.neuron_states[output_range].to_vec()
    }
}
