use std::time::Duration;
use core_crnn::thinking_layer::ThinkingLayer;
use game_lib::Game;

pub struct DrivingGame {}

impl Game for DrivingGame {
    fn extract_model(self) -> Option<ThinkingLayer> {
        todo!()
    }

    fn tick(&mut self, delta_time: Duration) {
        todo!()
    }

    fn tick_model(&mut self) {
        todo!()
    }

    fn score(&self) -> f32 {
        todo!()
    }
}