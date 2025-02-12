use crate::activation_function::ActivationFunction;
use crate::thinking_layer::ThinkingLayer;
use rand::Rng;
use std::marker::PhantomData;

pub struct Neuron<Activation: ActivationFunction> {
    bias: f64,
    delay: usize,
    phantom: PhantomData<Activation>,
}

impl<Activation: ActivationFunction> Neuron<Activation> {
    pub fn new(bias: f64, delay: usize) -> Self {
        Self {
            bias,
            delay,
            phantom: PhantomData,
        }
    }

    pub fn random<R: Rng>(&self, rng: &mut R) -> Self {
        Self {
            bias: rng.random::<f64>() * 2.0 - 1.0,
            delay: rng.random_range(1..=5),
            phantom: PhantomData,
        }
    }

    pub fn activate(
        &self,
        activate_index: usize,
        thinking_layer: &ThinkingLayer<Activation>,
    ) -> f64 {
        let values: f64 = (0..thinking_layer.internal_count - 1)
            .map(|i| if i >= activate_index { i + 1 } else { i })
            .map(|neuron_index| {
                let neuron_value = thinking_layer.neuron_states[neuron_index];

                let mut weight_index =
                    neuron_index * thinking_layer.internal_count + activate_index;
                if activate_index >= neuron_index {
                    weight_index -= 1;
                }
                let weight = thinking_layer.weights[weight_index];

                neuron_value * weight
            })
            .sum();

        values
    }

    fn process_input(value: f64) -> f64 {
        Activation::call(value)
    }

    pub fn bias(&self) -> f64 {
        self.bias
    }

    pub fn delay(&self) -> usize {
        self.delay
    }
}
