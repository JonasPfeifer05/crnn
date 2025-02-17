mod model_trainer;

use crate::model_trainer::{ModelTrainer, TrainConfig};
use core_crnn::activation_function::ActivationFunction::Tanh;
use core_crnn::thinking_layer::ThinkingLayer;
use game_lib::GameMetaData;
use ggez::event;
use pong::game::{PongGame, PongPlayer};
use pong::pong::Pong;

fn main() {
    // 1. Train 100 players
    // 2. Select top 2
    // 3. Make 98 children
    // 4. Mutate children
    // 5. Repeat

    let mut trainer = ModelTrainer::new(
        ThinkingLayer::new(
            PongGame::input_nodes(),
            6 + 5,
            PongGame::output_nodes(),
            Tanh,
        )
        .unwrap(),
        TrainConfig {
            epoch_size: 500,
            sample_size: 10,
            survival_rate: 0.1,
            mutation_probability: 0.02,
            mutation_strength: 0.05,
        },
    );

    for generation_index in 0..10000 {
        trainer.train_next_gen::<PongGame>();
        println!(
            "Finished generation: {}; Overall best: {:+.3?}; Current best: {:+.3}",
            generation_index,
            trainer.overall_best().as_ref().unwrap().score,
            trainer.last_generation_best().as_ref().unwrap().score
        );
    }

    let (ctx, events_loop) = ggez::ContextBuilder::new("pong", "")
        .window_setup(ggez::conf::WindowSetup::default().title("Pong!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(1500.0, 1500.0))
        .build()
        .unwrap();

    let state = Pong::new(
        PongPlayer::model(trainer.overall_best().as_ref().unwrap().model.clone()),
        PongPlayer::sync(),
    );
    event::run(ctx, events_loop, state)
}
