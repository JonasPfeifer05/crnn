use game_lib::Game;
use ggez::glam::{vec2, Vec2};
use rand::random_range;
use std::f32::consts::{FRAC_PI_4, PI};
use std::time::Duration;

pub const MAX_BOUNCE_ANGLE: f32 = FRAC_PI_4;
pub const PLAYER_HEIGHT: f32 = 0.2;
pub const PLAYER_WIDTH: f32 = 0.02;

pub struct PongGame {
    pub player: (PongPlayer, PongPlayer),
    pub state: PongGameState,
}

pub struct PongGameState {
    pub player_pos: (f32, f32),
    pub score: (usize, usize),
    pub ball_pos: Vec2,
    pub ball_dir: Vec2,
}

impl PongGame {
    pub fn new(player_one: PongPlayer, player_two: PongPlayer) -> Self {
        let start_direction = random_range(0.0..(PI * 2.));

        PongGame {
            player: (player_one, player_two),
            state: PongGameState {
                player_pos: (0.0, 0.0),
                score: (0, 0),
                ball_pos: vec2(0.5, 0.5),
                ball_dir: Vec2::from_angle(start_direction),
            },
        }
    }

    fn reset_ball(&mut self) {
        let start_direction = random_range(0.0..(PI * 2.));

        self.state.ball_pos = vec2(0.5, 0.5);
        self.state.ball_dir = Vec2::from_angle(start_direction);
    }
}

impl Game for PongGame {
    fn tick(&mut self, delta_time: Duration) {
        self.state.player_pos.0 += self
            .player
            .0
            .move_panel(&self.state, self.state.player_pos.0)
            .max(-1.0)
            .min(1.0)
            * delta_time.as_secs_f32();
        self.state.player_pos.1 += self
            .player
            .1
            .move_panel(&self.state, self.state.player_pos.1)
            .max(-1.0)
            .min(1.0)
            * delta_time.as_secs_f32();

        self.state.player_pos.0 = self.state.player_pos.0.max(0.0).min(1.0 - PLAYER_HEIGHT);
        self.state.player_pos.1 = self.state.player_pos.1.max(0.0).min(1.0 - PLAYER_HEIGHT);

        self.state.ball_pos += self.state.ball_dir * delta_time.as_secs_f32();

        if self.state.ball_pos.y > 1. {
            self.state.ball_dir.y = -self.state.ball_dir.y.abs();
        }

        if self.state.ball_pos.y < 0. {
            self.state.ball_dir.y = self.state.ball_dir.y.abs();
        }

        if self.state.ball_pos.x > 1. - PLAYER_WIDTH {
            if self.state.ball_pos.y > self.state.player_pos.1
                && self.state.ball_pos.y < self.state.player_pos.1 + PLAYER_HEIGHT
            {
                let relative_intersect = (self.state.ball_pos.y
                    - (self.state.player_pos.1 + PLAYER_HEIGHT / 2.))
                    / (PLAYER_HEIGHT / 2.);
                let bounce_angle = (relative_intersect) * MAX_BOUNCE_ANGLE;

                let speed = self.state.ball_dir.length(); // Maintain ball speed
                self.state.ball_dir.x = -self.state.ball_dir.x.abs() * bounce_angle.cos();
                self.state.ball_dir.y = bounce_angle.sin();
                self.state.ball_dir = self.state.ball_dir.normalize() * speed;
            }
        }

        if self.state.ball_pos.x < PLAYER_WIDTH {
            if self.state.ball_pos.y > self.state.player_pos.0
                && self.state.ball_pos.y < self.state.player_pos.0 + PLAYER_HEIGHT
            {
                let relative_intersect = (self.state.ball_pos.y
                    - (self.state.player_pos.0 + PLAYER_HEIGHT / 2.))
                    / (PLAYER_HEIGHT / 2.);
                let bounce_angle = (relative_intersect) * MAX_BOUNCE_ANGLE;

                let speed = self.state.ball_dir.length(); // Maintain ball speed
                self.state.ball_dir.x = self.state.ball_dir.x.abs() * bounce_angle.cos();
                self.state.ball_dir.y = bounce_angle.sin();
                self.state.ball_dir = self.state.ball_dir.normalize() * speed;
            }
        }

        if self.state.ball_pos.x > 1. {
            self.state.score.0 += 1;
            self.reset_ball()
        }

        if self.state.ball_pos.x < 0. {
            self.state.score.1 += 1;
            self.reset_ball()
        }
    }
}

pub enum PongPlayer {
    Keyboard {
        up_pressed: bool,
        down_pressed: bool,
    },
    Sync,
    Model,
}

impl PongPlayer {
    pub fn keyboard() -> PongPlayer {
        PongPlayer::Keyboard {
            down_pressed: false,
            up_pressed: false,
        }
    }

    pub fn move_panel(&self, state: &PongGameState, player_pos: f32) -> f32 {
        match self {
            PongPlayer::Keyboard {
                up_pressed: key_up_pressed,
                down_pressed: key_down_pressed,
            } => {
                if key_up_pressed == key_down_pressed {
                    return 0.0;
                };

                if *key_up_pressed {
                    -1.0
                } else {
                    1.0
                }
            }
            PongPlayer::Sync => (state.ball_pos.y - (player_pos + PLAYER_HEIGHT / 2.0)) * 5.,
            PongPlayer::Model => 0.0,
        }
    }
}
