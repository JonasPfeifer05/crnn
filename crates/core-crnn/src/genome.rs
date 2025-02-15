use crate::thinking_layer::ThinkingLayer;
use rand::{random_range, rng, Rng};

pub trait Genome {
    type Genome;
    type Child: Genome;
    fn genome(&self) -> Self::Genome;
    fn load_genome(&mut self, genome: Self::Genome);
    fn mutate(&mut self, mutation_probability: f64);
    fn crossover(genome_a: &Self, genome_b: &Self, n_pairs: usize) -> Vec<Self::Child>;
}

impl Genome for ThinkingLayer {
    type Genome = Vec<f64>;
    type Child = ThinkingLayer;

    fn genome(&self) -> Self::Genome {
        self.genome().to_vec()
    }

    fn load_genome(&mut self, genome: Self::Genome) {
        self.set_genome(genome);
    }

    fn mutate(&mut self, mutation_probability: f64) {
        let mut rng = rng();
        self.genome_mut().into_iter().for_each(|mut gene| {
            if rng.random::<f64>() > mutation_probability {
                *gene *= rng.random_range(-mutation_probability..mutation_probability);
            }
        });
    }

    fn crossover(genome_a: &Self, genome_b: &Self, n_pairs: usize) -> Vec<Self::Child> {
        // TODO Check for equal sizes
        let genome_len = genome_a.genome().len();
        let input_size = genome_a.input_size();
        let internal_size = genome_a.internal_size();
        let output_size = genome_a.output_size();
        let activation_function = genome_a.activation_function().clone();

        (0..n_pairs)
            .flat_map(|_| {
                let crossover_index = random_range(0..genome_len);
                vec![
                    vec![
                        &genome_a.genome()[0..crossover_index],
                        &genome_b.genome()[crossover_index..genome_len],
                    ]
                    .concat(),
                    vec![
                        &genome_b.genome()[0..crossover_index],
                        &genome_a.genome()[crossover_index..genome_len],
                    ]
                    .concat(),
                ]
            })
            .map(|genome| {
                let mut thinking_layer = Self::Child::new(
                    input_size,
                    internal_size,
                    output_size,
                    activation_function.clone(),
                )
                .unwrap();
                thinking_layer.load_genome(genome);
                thinking_layer
            })
            .collect()
    }
}
