use crate::game_impl::DrivingGame;
use crate::gui::{DrivingGui, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::player::{Player, PlayerInput};
use core_crnn::activation_function::ActivationFunction;
use core_crnn::activation_function::ActivationFunction::Tanh;
use core_crnn::thinking_layer::ThinkingLayer;
use game_lib::GameMetaData;
use ggez::glam::Vec2;
use ggez::{event, GameResult};
use std::f32::consts::FRAC_PI_2;

mod game_impl;
mod gui;
mod player;

fn main() -> GameResult {
    let (ctx, events_loop) = ggez::ContextBuilder::new("Driving Game", "Jonas Pfeifer")
        .window_setup(ggez::conf::WindowSetup::default().title("Fun Driving Game!"))
        .window_mode(
            ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32),
        )
        .build()?;

    let model = ThinkingLayer::new(
        DrivingGame::input_nodes(),
        DrivingGame::input_nodes() + 10 + DrivingGame::output_nodes(),
        DrivingGame::output_nodes(),
        Tanh,
    )
    .unwrap();

    let input = PlayerInput::ai(
        ThinkingLayer::new(
            DrivingGame::input_nodes(),
            DrivingGame::input_nodes() + 10 + DrivingGame::output_nodes(),
            DrivingGame::output_nodes(),
            Tanh,
        )
        .unwrap(),
    );

    let input = PlayerInput::human();

    let state = DrivingGui::new(DrivingGame::new(Player {
        input,
        current_position: Default::default(),
        direction: 0.0,
        velocity: 0.0,
    }));
    event::run(ctx, events_loop, state)
}
