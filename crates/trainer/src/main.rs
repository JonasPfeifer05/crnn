use core_crnn::activation_function::ActivationFunction;
use core_crnn::genome::Genome;
use core_crnn::thinking_layer::ThinkingLayer;
use game_lib::{Game, GameSettings};
use pong::game::PongGame;
use std::time::Instant;

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

    loop {
        let input_nodes = PongGame::input_nodes();
        let output_nodes = PongGame::output_nodes();

        let model =
            ThinkingLayer::new(input_nodes, 50, output_nodes, ActivationFunction::Tanh).unwrap();

        let mut game = PongGame::from_model(model);

        let instant = Instant::now();

        game.run(GameSettings::default());

        println!("{} - {:?}", game.score(), instant.elapsed());
    }
}
