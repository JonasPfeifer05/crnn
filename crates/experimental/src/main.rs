use crate::thinking_layer::ThinkingLayer;
use rand::{Rng, rng};
use tokio::time::Instant;

mod connection;
mod neuron;
mod thinking_layer;

#[tokio::main]
async fn main() {
    let mut thinking_layer = ThinkingLayer::new(4, 1, 256);
    loop {
        let start = Instant::now();
        let input = vec![rng().random::<f32>(); 4];
        let output = thinking_layer.tick(Some(input.clone()));
        println!("{:7.5?} -> {:.5?} ({:.2?})", input, output, start.elapsed());

        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}
