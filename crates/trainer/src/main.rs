mod model_trainer;

use crate::model_trainer::{ModelTrainer, TrainConfig};
use core_crnn::activation_function::ActivationFunction::Tanh;
use core_crnn::genome::Genome;
use core_crnn::thinking_layer::ThinkingLayer;
use game_lib::GameMetaData;
use ggez::event;
use pong::game::{PongGame, PongPlayer};
use pong::pong::Pong;
use std::fs;
use trainer::persisted_genome::PersistedGenome;

fn main() {
    let mut last_saved = None;
    let mut model = ThinkingLayer::new(
        PongGame::input_nodes(),
        PongGame::input_nodes() + 5 + PongGame::output_nodes(),
        PongGame::output_nodes(),
        Tanh,
    )
    .unwrap();
    if let Ok(bytes) = fs::read("model.json") {
        println!("Loading model...");
        let data: PersistedGenome = serde_json::from_slice(&bytes).unwrap();
        model.load_genome(data.genome);
        last_saved = Some(data.score);
    }

    let mut trainer = ModelTrainer::new(
        model,
        TrainConfig {
            epoch_size: 500,
            sample_size: 10,
            survival_rate: 0.1,
            mutation_probability: 0.05,
            mutation_strength: 0.2,
        },
    );

    for generation_index in 0..100 {
        trainer.train_next_gen::<PongGame>();
        let all_time_best = trainer.overall_best().as_ref().unwrap().score;

        if last_saved.is_none() || (last_saved.is_some() && last_saved.unwrap() < all_time_best) {
            let best_genome = trainer
                .overall_best()
                .as_ref()
                .unwrap()
                .model
                .genome()
                .to_vec();

            println!("Saving new best model...");
            last_saved = Some(all_time_best);
            let json = serde_json::to_string(&PersistedGenome {
                score: all_time_best,
                genome: best_genome,
            })
            .unwrap();
            fs::write("model.json", json).unwrap();
        }

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
