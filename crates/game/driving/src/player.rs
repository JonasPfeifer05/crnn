use crate::gui::PIXELS_PER_METER;
use core_crnn::thinking_layer::ThinkingLayer;
use ggez::glam::Vec2;
use std::f32::consts::{FRAC_PI_2, PI};

pub const PLAYER_WIDTH: f32 = 2.200 * PIXELS_PER_METER;
pub const PLAYER_HEIGHT: f32 = 5.0 * PIXELS_PER_METER;

const CAR_MAX_SPEED: f32 = 82.22 * PIXELS_PER_METER;
const CAR_DECELERATION: f32 = 15.0 * PIXELS_PER_METER;
const CAR_MAX_ACCELERATION: f32 = CAR_DECELERATION + 8.68 * PIXELS_PER_METER;
const CAR_TURNING_RADIUS: f32 = 10.5 * PIXELS_PER_METER;

pub enum PlayerInput {
    Human { w: bool, a: bool, s: bool, d: bool },
    Ai(ThinkingLayer),
}

impl PlayerInput {
    pub fn human() -> Self {
        Self::Human {
            w: false,
            a: false,
            s: false,
            d: false,
        }
    }

    pub fn ai(thinking_layer: ThinkingLayer) -> Self {
        Self::Ai(thinking_layer)
    }
}

pub struct Player {
    pub input: PlayerInput,
    pub current_position: Vec2,
    pub direction: f32,
    pub velocity: f32,
}

impl Player {
    pub fn new(input: PlayerInput, current_position: Vec2, direction: f32) -> Self {
        Self {
            input,
            current_position,
            direction,
            velocity: 0.0,
        }
    }

    pub fn update_position(&mut self, dt: f32) {
        let (throttle, steering) = match &self.input {
            PlayerInput::Human { w, a, s, d } => (
                (*s as isize - *w as isize) as f64,
                (*d as isize - *a as isize) as f64,
            ),
            PlayerInput::Ai(model) => {
                let mut ai_output = model.output();
                (
                    ai_output.remove(0).min(1.0).max(-1.0),
                    ai_output.remove(0).min(1.0).max(-1.0),
                )
            }
        };

        let factor = if self.velocity != 0.0 {
            1.0 / ((self.velocity / PIXELS_PER_METER + 1.0) / 4.0)
                .abs()
                .sqrt()
        } else {
            1.0
        };

        let max_turning_speed = (self.velocity / CAR_TURNING_RADIUS) * factor * dt;
        self.direction = (self.direction + max_turning_speed * steering as f32) % (2.0 * PI);

        self.velocity = (self.velocity + CAR_MAX_ACCELERATION * dt * throttle as f32)
            .min(CAR_MAX_SPEED)
            .max(-CAR_MAX_SPEED);

        let deceleration = self.velocity.abs().min(CAR_DECELERATION * dt);
        self.velocity -= self.velocity.signum() * deceleration;

        self.current_position += Vec2::from_angle(self.direction + FRAC_PI_2) * self.velocity * dt;
    }
}
