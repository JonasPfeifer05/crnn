use core_crnn::activation_function::ActivationFunction;
use core_crnn::thinking_layer::ThinkingLayer;
use rand::{rng, Rng};
use std::time::Instant;
use core_crnn::genome::Genome;

fn main() -> anyhow::Result<()> {
    let input = 8;
    let internal = 1024;
    let output = 1;

    let mut thinking_layer_a =
        ThinkingLayer::new(input, internal, output, ActivationFunction::Sigmoid)?;
    let mut thinking_layer_b =
        ThinkingLayer::new(input, internal, output, ActivationFunction::Sigmoid)?;
    let thinking_layer_c =
        &mut ThinkingLayer::crossover(&thinking_layer_a, &thinking_layer_b, 1)[0];

    loop {
        let start = Instant::now();
        let input: Vec<f64> = rng().random_iter().take(input).collect();
        let output_a = thinking_layer_a.tick(Some(input.clone()));
        let output_b = thinking_layer_b.tick(Some(input.clone()));
        let output_c = thinking_layer_c.tick(Some(input.clone()));

        println!("A: {:.4?} -> {:.4?} ({:?})", input, output_a, start.elapsed());
        println!("B: {:.4?} -> {:.4?} ({:?})", input, output_b, start.elapsed());
        println!("C: {:.4?} -> {:.4?} ({:?})", input, output_c, start.elapsed());
    }
}
