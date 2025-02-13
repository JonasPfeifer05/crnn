use core_crnn::activation_function::ActivationFunction;
use core_crnn::thinking_layer::ThinkingLayer;
use game_lib::{Game, GameSettings};
use pong::game::PongGame;
use std::time::Instant;

fn main() {
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
