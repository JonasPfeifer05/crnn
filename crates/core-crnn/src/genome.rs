use crate::thinking_layer::ThinkingLayer;
use itertools::izip;
use rand::{random_iter, rng, Rng};
use rand_distr::Distribution;

pub trait Genome {
    type Genome;
    type Child: Genome;
    fn genome(&self) -> Self::Genome;
    fn load_genome(&mut self, genome: Self::Genome);
    fn mutate(&mut self, mutation_probability: f64, mutation_strength: f64);
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

    fn mutate(&mut self, mutation_probability: f64, mutation_strength: f64) {
        let mut rng = rng();
        self.genome_mut().into_iter().for_each(|gene| {
            if rng.random::<f64>() < mutation_probability {
                *gene = *gene + rand_distr::Normal::new(0.0, mutation_strength).unwrap().sample(&mut rng);
            }
        });
    }

    fn crossover(genome_a: &Self, genome_b: &Self, n_pairs: usize) -> Vec<Self::Child> {
        let genome_len = genome_a.genome().len();
        let input_size = genome_a.input_size();
        let internal_size = genome_a.internal_size();
        let output_size = genome_a.output_size();
        let activation_function = genome_a.activation_function().clone();
        let variations: Vec<f64> = random_iter().take(genome_len).collect();

        let children: Vec<_> = (0..n_pairs)
            .flat_map(|_| {
                let (a, b): (Vec<_>, Vec<_>) =
                    izip!(genome_a.genome(), genome_b.genome(), &variations)
                        .map(|(genome_a, genome_b, variation)| {
                            (
                                genome_a * variation + genome_b * (1.0 - variation),
                                genome_a * (1.0 - variation) + genome_b * variation,
                            )
                        })
                        .unzip();

                vec![a, b]
            })
            .map(|genome| {
                let mut child = ThinkingLayer::new(
                    input_size,
                    internal_size,
                    output_size,
                    activation_function.clone(),
                )
                .unwrap();
                child.load_genome(genome);
                child
            })
            .collect();

        children
    }
}
