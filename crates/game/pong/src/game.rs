use crate::{Pong, PLAYER_HEIGHT};
use game_lib::Game;
use std::time::Duration;

pub struct PongGame {
    pub player: (PongPlayer, PongPlayer),
    pub state: PongGameState,
}

pub struct PongGameState {
    pub player_pos: (f32, f32),
    pub score: (usize, usize),
}

impl PongGame {
    pub fn new(player_one: PongPlayer, player_two: PongPlayer) -> Self {
        PongGame {
            player: (player_one, player_two),
            state: PongGameState {
                player_pos: (0.0, 0.0),
                score: (0, 0),
            },
        }
    }
}

impl Game for PongGame {
    fn tick(&mut self, delta_time: Duration) {
        self.state.player_pos.0 += self.player.0.move_panel(&self.state) * delta_time.as_secs_f32();
        self.state.player_pos.1 += self.player.1.move_panel(&self.state) * delta_time.as_secs_f32();

        self.state.player_pos.0 = self.state.player_pos.0.max(0.0).min(1.0 - PLAYER_HEIGHT);
        self.state.player_pos.1 = self.state.player_pos.1.max(0.0).min(1.0 - PLAYER_HEIGHT);
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

    pub fn move_panel(&self, state: &PongGameState) -> f32 {
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
            PongPlayer::Sync => 0.0,
            PongPlayer::Model => 0.0,
        }
    }
}
