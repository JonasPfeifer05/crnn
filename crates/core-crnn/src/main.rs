use core_crnn::activation_function::{Sigmoid, Tanh};
use core_crnn::thinking_layer::ThinkingLayer;
use rand::{rng, Rng};
use std::time::{Duration, Instant};

fn main() -> anyhow::Result<()> {
    let input = 4;
    let internal = 8;
    let output = 1;

    let mut thinking_layer = ThinkingLayer::<Sigmoid>::new(input, internal, output)?;

    loop {
        let start = Instant::now();
        let input: Vec<f64> = rng().random_iter().take(input).collect();
        let output = thinking_layer.tick(Some(input.clone()));

        println!("{:.4?} -> {:.4?} ({:?})", input, output, start.elapsed());

        std::thread::sleep(Duration::from_millis(100));
    }
}
