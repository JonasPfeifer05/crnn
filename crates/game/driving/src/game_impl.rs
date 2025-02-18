use crate::player::{Player, PlayerInput};
use crate::track::Track;
use core_crnn::thinking_layer::ThinkingLayer;
use game_lib::{Game, GameMetaData};
use ggez::glam::Vec2;
use std::f32::consts::FRAC_PI_2;
use std::time::Duration;

pub struct DrivingGame {
    pub score: f32,
    pub player: Player,
    pub track: Track,
}

impl DrivingGame {
    pub fn new(player: Player, track: Track) -> Self {
        Self {
            score: 0.0,
            player,
            track,
        }
    }
}

impl GameMetaData for DrivingGame {
    fn from_model(model: ThinkingLayer) -> Self {
        DrivingGame {
            score: 0.0,
            player: Player::new(PlayerInput::Ai(model), Vec2::default(), FRAC_PI_2),
            track: Track::default(),
        }
    }

    fn input_nodes() -> usize {
        9 + 1 + 1 // 9 rays (distances), current direction, current_velocity
    }

    fn output_nodes() -> usize {
        1 + 1 // throttle, steering
    }
}

impl Game for DrivingGame {
    fn extract_model(self) -> Option<ThinkingLayer> {
        match self.player.input {
            PlayerInput::Ai(model) => Some(model),
            _ => None,
        }
    }

    fn tick(&mut self, delta_time: Duration) {
        let dt = delta_time.as_secs_f32();
        self.player.update_position(dt);
    }

    fn tick_model(&mut self) {
        let input = vec![
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            self.player.direction as f64,
            self.player.velocity as f64,
        ];
        match &mut self.player.input {
            PlayerInput::Ai(model) => model.tick(Some(input)),
            _ => {}
        }
    }

    fn score(&self) -> f32 {
        self.score
    }
}
