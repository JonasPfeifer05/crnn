use core_crnn::activation_function::ActivationFunction;
use core_crnn::thinking_layer::ThinkingLayer;
use rand::{rng, Rng};
use std::time::Instant;

fn main() -> anyhow::Result<()> {
    let input = 8;
    let internal = 1024;
    let output = 1;

    let mut thinking_layer =
        ThinkingLayer::new(input, internal, output, ActivationFunction::Sigmoid)?;

    loop {
        let start = Instant::now();
        let input: Vec<f64> = rng().random_iter().take(input).collect();
        thinking_layer.tick(Some(input.clone()));
        let output = thinking_layer.output();

        println!("{:.4?} -> {:.4?} ({:?})", input, output, start.elapsed());
    }
}
