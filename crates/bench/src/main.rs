use core_crnn::activation_function::ActivationFunction;
use core_crnn::thinking_layer::ThinkingLayer;
use std::time::Instant;

fn main() {
    let mut model = ThinkingLayer::new(8, 256, 8, ActivationFunction::Other(|x| x)).unwrap();

    let now = Instant::now();

    for _ in 0..100_000 {
        model.tick(None);
    }

    println!("{:?}", now.elapsed());
}
