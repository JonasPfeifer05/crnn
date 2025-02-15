use core_crnn::activation_function::ActivationFunction;
use core_crnn::genome::Genome;
use core_crnn::thinking_layer::ThinkingLayer;

fn main() {
    // 1. Train 100 players
    // 2. Select top 2
    // 3. Make 98 children
    // 4. Mutate children
    // 5. Repeat

    let num_pairs = 49;
    let mutate_probability = 0.02;

    let mut players = vec![
        ThinkingLayer::new(7, 32, 1, ActivationFunction::Sigmoid).unwrap(),
        ThinkingLayer::new(7, 32, 1, ActivationFunction::Sigmoid).unwrap(),
    ];
    players.extend(ThinkingLayer::crossover(
        &players[0],
        &players[1],
        num_pairs,
    ));
    let players: Vec<_> = players
        .into_iter()
        .map(|mut player| {
            player.mutate(mutate_probability);
            player
        })
        .collect();

    println!("Hello, world!");
}
