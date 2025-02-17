use core_crnn::activation_function::ActivationFunction;
use core_crnn::thinking_layer::ThinkingLayer;
use game_lib::{Game, GameMetaData};
use ggez::glam::{vec2, Vec2};
use ggez::graphics::{Canvas, Color, Rect, Text, TextAlign, TextLayout};
use ggez::winit::event::VirtualKeyCode;
use ggez::{event, graphics, input::keyboard::KeyInput, Context, GameError, GameResult};
use core_crnn::activation_function::ActivationFunction::Sigmoid;
use pong::game::{PongGame, PongPlayer, PongPlayerInput, PLAYER_HEIGHT, PLAYER_WIDTH};
use pong::pong::Pong;

fn main() -> GameResult {
    println!("{:+.2}", 0.12345);
    
    let (ctx, events_loop) = ggez::ContextBuilder::new("pong", "")
        .window_setup(ggez::conf::WindowSetup::default().title("Pong!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(720.0, 720.0))
        .build()?;

    let model = ThinkingLayer::new(
        PongGame::input_nodes(),
        100,
        PongGame::output_nodes(),
        ActivationFunction::Tanh,
    )
    .unwrap();

    let state = Pong::new(PongPlayer::model(model), PongPlayer::sync());
    event::run(ctx, events_loop, state)
}
