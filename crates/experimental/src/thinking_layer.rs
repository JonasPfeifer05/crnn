use crate::connection::Connection;
use crate::neuron::Neuron;
use rand::{Rng, rng};
use std::collections::HashMap;

pub struct ThinkingLayer {
    pub input_count: usize,
    pub input_connections: Vec<Vec<Connection>>,
    pub internal_count: usize,
    pub internal_neurons: Vec<Neuron>,
    pub internal_connections: Vec<Vec<Connection>>,
    pub output_count: usize,
    pub output_connections: Vec<Vec<Connection>>,
    pub pending_activations: HashMap<usize, HashMap<usize, f32>>,
}

impl ThinkingLayer {
    pub fn new(inputs: usize, outputs: usize, internal_neuron_count: usize) -> ThinkingLayer {
        Self {
            input_count: inputs,
            input_connections: (0..inputs)
                .map(|_| {
                    (0..internal_neuron_count)
                        .map(|_| Connection::new(rng().random::<f32>() * 2.0 - 1.0))
                        .collect()
                })
                .collect(),
            internal_count: internal_neuron_count,
            internal_neurons: (0..internal_neuron_count)
                .map(|_| {
                    Neuron::new(
                        rng().random::<f32>() - 0.5,
                        rng().random::<f32>().round() as usize + 1,
                        0.0,
                    )
                })
                .collect(),
            internal_connections: (0..internal_neuron_count)
                .map(|_| {
                    (0..internal_neuron_count-1)
                        .map(|_| Connection::new(rng().random::<f32>() * 2.0 - 1.0))
                        .collect()
                })
                .collect(),
            output_count: outputs,
            output_connections: (0..internal_neuron_count)
                .map(|_| {
                    (0..outputs)
                        .map(|_| Connection::new(rng().random::<f32>() * 2.0 - 1.0))
                        .collect()
                })
                .collect(),
            pending_activations: HashMap::new(),
        }
    }

    pub fn tick(&mut self, input: Option<Vec<f32>>) -> Vec<f32> {
        // Update old pending activations
        self.update_pending();

        let mut tick_result: HashMap<usize, HashMap<usize, f32>> = HashMap::new();
        let mut output_result = vec![0.0; self.output_count];

        // Calculate current input
        if let Some(input_values) = input {
            if input_values.len() != self.input_count {
                // TODO ERROR
            }

            let instant_activations = tick_result.entry(1).or_default();
            for input_index in 0..self.input_count {
                let weighted_input: Vec<_> = self.input_connections[input_index]
                    .iter()
                    .map(|connection| input_values[input_index] * connection.weight)
                    .collect();

                for weighted_value_index in 0..weighted_input.len() {
                    let neuron_value = instant_activations
                        .entry(weighted_value_index)
                        .or_insert(0.0);
                    *neuron_value += weighted_input[weighted_value_index];
                }
            }
        }

        // calculate current activations
        if let Some(activations) = self.pending_activations.remove(&0) {
            for (neuron_index, value) in activations {
                let neuron = &mut self.internal_neurons[neuron_index];
                let activated_value = neuron.activate(value + neuron.bias);
                let internal_weighted_values: Vec<_> = self.internal_connections[neuron_index]
                    .iter()
                    .map(|connection| activated_value * connection.weight)
                    .collect();

                let activations = tick_result.entry(neuron.delay).or_default();
                for neuron_index in 0..neuron_index {
                    let neuron_value = activations.entry(neuron_index).or_insert(0.0);
                    *neuron_value += internal_weighted_values[neuron_index];
                }

                for neuron_index in neuron_index + 1..self.internal_count {
                    let neuron_value = activations.entry(neuron_index).or_insert(0.0);
                    *neuron_value += internal_weighted_values[neuron_index - 1];
                }

                let output_weighted_values: Vec<_> = self.output_connections[neuron_index]
                    .iter()
                    .map(|connection| activated_value * connection.weight)
                    .collect();

                for weighted_output_value_index in 0..self.output_count {
                    output_result[weighted_output_value_index] +=
                        output_weighted_values[weighted_output_value_index];
                }
            }
        }
        for (delay, new_activations) in tick_result {
            let activations = self.pending_activations.entry(delay).or_default();
            for (neuron_index, value) in new_activations {
                let neuron_value = activations.entry(neuron_index).or_insert(0.0);
                *neuron_value += value;
            }
        }

        output_result
    }

    fn update_pending(&mut self) {
        let pending_activations: Vec<_> = self.pending_activations.drain().collect();
        for (key, value) in pending_activations {
            self.pending_activations.insert(key - 1, value);
        }
    }
}
