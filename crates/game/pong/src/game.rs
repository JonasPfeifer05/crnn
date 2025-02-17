use game_lib::{Game, GameMetaData};
use ggez::glam::{vec2, Vec2};
use rand::random_range;
use std::f32::consts::FRAC_PI_4;
use std::time::Duration;

pub const MAX_BOUNCE_ANGLE: f32 = FRAC_PI_4;
pub const PLAYER_HEIGHT: f32 = 0.2;
pub const PLAYER_WIDTH: f32 = 0.02;

pub enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn orient_vec2(&self, vec: &mut Vec2) {
        match self {
            Direction::Left => vec.x = -vec.x.abs(),
            Direction::Right => vec.x = vec.x.abs(),
        }
    }
}

pub struct PongGame {
    pub player: (PongPlayer, PongPlayer),
    pub state: PongGameState,
    score: f32,
}

pub struct PongGameState {
    pub score: (usize, usize),
    pub ball_pos: Vec2,
    pub ball_dir: Vec2,
}

pub fn random_ball_direction(direction: Direction) -> Vec2 {
    let angle = random_range(-FRAC_PI_4..FRAC_PI_4);
    let mut dir = Vec2::from_angle(angle);
    direction.orient_vec2(&mut dir);
    dir
}

impl PongGame {
    pub fn new(player_one: PongPlayer, player_two: PongPlayer) -> Self {
        let ball_dir = random_ball_direction(Direction::Left);

        PongGame {
            player: (player_one, player_two),
            state: PongGameState {
                score: (0, 0),
                ball_pos: vec2(0.5, 0.5),
                ball_dir,
            },
            score: 0.0,
        }
    }

    fn reset_ball(&mut self, direction: Direction) {
        self.state.ball_pos = vec2(0.5, 0.5);

        self.state.ball_dir = random_ball_direction(direction);
    }
}

impl GameMetaData for PongGame {
    fn from_model(model: core_crnn::thinking_layer::ThinkingLayer) -> Self {
        PongGame::new(PongPlayer::model(model), PongPlayer::sync())
    }
    fn input_nodes() -> usize {
        5
    }

    fn output_nodes() -> usize {
        1
    }
}

impl Game for PongGame {
    fn extract_model(self) -> Option<core_crnn::thinking_layer::ThinkingLayer> {
        match self.player.0.input {
            PongPlayerInput::Model(model) => Some(model),
            _ => None,
        }
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
            self.state.ball_dir = self
                .player
                .1
                .get_bounce_dir(self.state.ball_pos.y, Direction::Left);
        }

        if self.state.ball_pos.x < PLAYER_WIDTH
            && self.player.0.does_intersect_ball(&self.state.ball_pos)
        {
            self.state.ball_dir = self
                .player
                .0
                .get_bounce_dir(self.state.ball_pos.y, Direction::Right);
        }

        // handle player loose
        if self.state.ball_pos.x > 1. {
            self.state.score.0 += 1;
            self.score += 1.0;
            self.reset_ball(Direction::Left);
        }

        if self.state.ball_pos.x < 0. {
            self.state.score.1 += 1;
            self.score -= 1.0 + (self.state.ball_pos.y - self.player.0.pos).abs();
            self.reset_ball(Direction::Right)
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
        self.score
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

    pub fn get_bounce_dir(&self, ball_y: f32, direction: Direction) -> Vec2 {
        let relative_intersect = (ball_y - (self.pos + PLAYER_HEIGHT / 2.)) / (PLAYER_HEIGHT / 2.);
        let mut dir = Vec2::from_angle((relative_intersect) * MAX_BOUNCE_ANGLE);
        direction.orient_vec2(&mut dir);
        dir
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
