use core_crnn::thinking_layer::ThinkingLayer;
use std::time::Duration;

pub trait GameMetaData{
    fn from_model(model: ThinkingLayer) -> Self;
    fn input_nodes() -> usize;
    fn output_nodes() -> usize;
}

pub trait Game {
    fn extract_model(self) -> Option<ThinkingLayer>;
    
    fn run(&mut self, game_settings: GameSettings) -> f32 {
        let tick_count =
            game_settings.duration.as_millis() / game_settings.tick_duration.as_millis();

        for _ in 0..game_settings.pre_ticks {
            self.tick_model()
        }

        for _ in 0..tick_count {
            for _ in 0..game_settings.think_steps {
                self.tick_model()
            }

            self.tick(game_settings.tick_duration)
        }

        self.score()
    }

    fn tick(&mut self, delta_time: Duration);
    fn tick_model(&mut self);
    fn score(&self) -> f32;
}

pub struct GameSettings {
    duration: Duration,
    pre_ticks: usize,
    tick_duration: Duration,
    think_steps: usize,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            duration: Duration::from_secs_f32(30.),
            pre_ticks: 0,
            tick_duration: Duration::from_secs_f32(1. / 60.), // 60 fps
            think_steps: 1,
        }
    }
}

impl GameSettings {
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn pre_ticks(mut self, pre_ticks: usize) -> Self {
        self.pre_ticks = pre_ticks;
        self
    }

    pub fn tick_duration(mut self, tick_duration: Duration) -> Self {
        self.tick_duration = tick_duration;
        self
    }

    pub fn think_steps(mut self, think_steps: usize) -> Self {
        self.think_steps = think_steps;
        self
    }
}
