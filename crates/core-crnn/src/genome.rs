use crate::activation_function::ActivationFunction;
use crate::thinking_layer::ThinkingLayer;

pub struct Genome {
    pub bias: Vec<f64>,
    pub delay: Vec<usize>,
    pub weights: Vec<f64>,
}

pub struct GenomeMetaData {
    pub input_size: usize,
    pub internal_size: usize,
    pub output_size: usize,
    pub activation_functions: Vec<ActivationFunction>,
}

impl Genome {
    pub fn from_thinking_layer(layer: &ThinkingLayer) -> Genome {
        todo!()
    }
    
    pub fn crossover(genome_a: &Genome, genome_b: &Genome) -> (Genome, Genome) {
        todo!()
    }
    
    pub fn mutate(&mut self, mutate_probability: f64) {
        // 0.02
        
        todo!()
    }
}

