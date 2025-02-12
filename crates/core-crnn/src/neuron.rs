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

    pub fn activate(&self, thinking_layer: &ThinkingLayer<Activation>) -> f64 {
        todo!()
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
