use game_lib::Game;
use ggez::glam::{vec2, Vec2};
use rand::random_range;
use std::f32::consts::FRAC_PI_4;
use std::time::Duration;

pub const MAX_BOUNCE_ANGLE: f32 = FRAC_PI_4;
pub const PLAYER_HEIGHT: f32 = 0.2;
pub const PLAYER_WIDTH: f32 = 0.02;

pub struct PongGame {
    pub player: (PongPlayer, PongPlayer),
    pub state: PongGameState,
}

pub struct PongGameState {
    pub score: (usize, usize),
    pub ball_pos: Vec2,
    pub ball_dir: Vec2,
}

pub fn random_ball_direction() -> f32 {
    random_range(-FRAC_PI_4..FRAC_PI_4)
}

impl PongGame {
    pub fn new(player_one: PongPlayer, player_two: PongPlayer) -> Self {
        let mut ball_dir = Vec2::from_angle(random_ball_direction());
        ball_dir.x *= -1.0;

        PongGame {
            player: (player_one, player_two),
            state: PongGameState {
                score: (0, 0),
                ball_pos: vec2(0.5, 0.5),
                ball_dir,
            },
        }
    }

    fn reset_ball(&mut self, direction_mul: i32) {
        let start_direction = random_ball_direction();

        self.state.ball_pos = vec2(0.5, 0.5);
        self.state.ball_dir = Vec2::from_angle(start_direction);
        self.state.ball_dir.x *= direction_mul as f32;
    }
}

impl Game for PongGame {
    fn from_model(model: core_crnn::thinking_layer::ThinkingLayer) -> Self {
        PongGame::new(PongPlayer::model(model), PongPlayer::sync())
    }

    fn input_nodes() -> usize {
        5
    }

    fn output_nodes() -> usize {
        1
    }

    fn tick(&mut self, delta_time: Duration) {
        // update positions
        self.player.0.update_pos(&self.state, &delta_time);
        self.player.1.update_pos(&self.state, &delta_time);

        self.state.ball_pos += self.state.ball_dir * delta_time.as_secs_f32();

        // bounce ball on ceiling / floor
        if self.state.ball_pos.y > 1. {
            self.state.ball_dir.y = -self.state.ball_dir.y.abs();
        }

        if self.state.ball_pos.y < 0. {
            self.state.ball_dir.y = self.state.ball_dir.y.abs();
        }

        // bounce ball on panels
        if self.state.ball_pos.x > 1. - PLAYER_WIDTH
            && self.player.1.does_intersect_ball(&self.state.ball_pos)
        {
            let bounce_angle = self.player.1.get_bounce_angle(self.state.ball_pos.y);

            self.state.ball_dir = Vec2::from_angle(bounce_angle);
            self.state.ball_dir.y *= -1.;
        }

        if self.state.ball_pos.x < PLAYER_WIDTH
            && self.player.0.does_intersect_ball(&self.state.ball_pos)
        {
            let bounce_angle = self.player.0.get_bounce_angle(self.state.ball_pos.y);

            self.state.ball_dir = Vec2::from_angle(bounce_angle);
        }

        // handle player loose
        if self.state.ball_pos.x > 1. {
            self.state.score.0 += 1;
            self.reset_ball(-1)
        }

        if self.state.ball_pos.x < 0. {
            self.state.score.1 += 1;
            self.reset_ball(1)
        }
    }

    fn tick_model(&mut self) {
        if let PongPlayerInput::Model(model) = &mut self.player.0.input {
            let input = vec![
                self.player.0.pos as f64,
                self.state.ball_pos.x as f64,
                self.state.ball_pos.y as f64,
                self.state.ball_dir.x as f64,
                self.state.ball_dir.y as f64,
            ];

            model.tick(Some(input));
        }

        if let PongPlayerInput::Model(model) = &mut self.player.1.input {
            let input = vec![
                self.player.1.pos as f64,
                self.state.ball_pos.x as f64,
                self.state.ball_pos.y as f64,
                self.state.ball_dir.x as f64,
                self.state.ball_dir.y as f64,
            ];

            model.tick(Some(input));
        }
    }

    fn score(&self) -> f32 {
        self.state.score.0 as f32 - self.state.score.1 as f32
    }
}

pub struct PongPlayer {
    input: PongPlayerInput,
    pos: f32,
}

impl PongPlayer {
    pub fn keyboard() -> PongPlayer {
        PongPlayer {
            input: PongPlayerInput::Keyboard {
                down_pressed: false,
                up_pressed: false,
            },
            pos: 0.5,
        }
    }

    pub fn sync() -> PongPlayer {
        PongPlayer {
            input: PongPlayerInput::Sync,
            pos: 0.5,
        }
    }

    pub fn model(model: core_crnn::thinking_layer::ThinkingLayer) -> PongPlayer {
        PongPlayer {
            input: PongPlayerInput::Model(model),
            pos: 0.5,
        }
    }

    pub fn update_pos(&mut self, state: &PongGameState, delta_time: &Duration) {
        self.pos += self.input.normalized_tick(state, self.pos) * delta_time.as_secs_f32();
        self.pos = self.pos.clamp(0.0, 1.0 - PLAYER_HEIGHT);
    }

    pub fn does_intersect_ball(&self, ball_pos: &Vec2) -> bool {
        ball_pos.y > self.pos && ball_pos.y < self.pos + PLAYER_HEIGHT
    }

    pub fn get_bounce_angle(&self, ball_y: f32) -> f32 {
        let relative_intersect = (ball_y - (self.pos + PLAYER_HEIGHT / 2.)) / (PLAYER_HEIGHT / 2.);
        (relative_intersect) * MAX_BOUNCE_ANGLE
    }

    pub fn input_mut(&mut self) -> &mut PongPlayerInput {
        &mut self.input
    }
    pub fn pos(&self) -> f32 {
        self.pos
    }
}

pub enum PongPlayerInput {
    Keyboard {
        up_pressed: bool,
        down_pressed: bool,
    },
    Sync,
    Model(core_crnn::thinking_layer::ThinkingLayer),
}

impl PongPlayerInput {
    pub fn normalized_tick(&self, state: &PongGameState, player_pos: f32) -> f32 {
        self.tick(state, player_pos).clamp(-1.0, 1.0)
    }

    fn tick(&self, state: &PongGameState, player_pos: f32) -> f32 {
        match self {
            PongPlayerInput::Keyboard {
                up_pressed,
                down_pressed,
            } => match (*up_pressed, *down_pressed) {
                (true, false) => -1.0,
                (false, true) => 1.0,
                _ => 0.0,
            },
            PongPlayerInput::Sync => (state.ball_pos.y - (player_pos + PLAYER_HEIGHT / 2.0)) * 5.,
            PongPlayerInput::Model(model) => *model.output().first().unwrap() as f32,
        }
    }
}
