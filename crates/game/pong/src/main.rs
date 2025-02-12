use ggez::{
    event, graphics,
    input::keyboard::{KeyCode, KeyInput},
    Context, GameError, GameResult,
};

use crate::game::{PongGame, PongPlayer};
use game_lib::Game;
use ggez::graphics::Rect;
use ggez::winit::event::VirtualKeyCode;

mod game;
struct Pong {
    game: PongGame,
}

impl Pong {
    pub fn new(player_one: PongPlayer, player_two: PongPlayer) -> Self {
        Pong {
            game: PongGame::new(player_one, player_two),
        }
    }
}

impl event::EventHandler<GameError> for Pong {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.game.tick(ctx.time.delta());
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.0, 0.0, 0.0, 1.0]));

        canvas.set_screen_coordinates(Rect::new(0., 0., 1., 1.));

        let pos_0 = self.game.state.player_pos.0;
        let pos_1 = self.game.state.player_pos.1;

        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(Rect::new(0., pos_0, 0.05, 0.2))
                .color([1.0, 1.0, 1.0, 1.0]),
        );

        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(Rect::new(0.95, pos_1, 0.05, 0.2))
                .color([1.0, 1.0, 1.0, 1.0]),
        );

        canvas.finish(ctx)?;

        ggez::timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        let player_one = &mut self.game.player.0;
        let player_two = &mut self.game.player.1;

        if let Some(key) = input.keycode {
            match (key, player_one, player_two) {
                (VirtualKeyCode::Up, PongPlayer::Keyboard { key_up_pressed, .. }, _) => {
                    *key_up_pressed = true
                }
                (
                    VirtualKeyCode::Down,
                    PongPlayer::Keyboard {
                        key_down_pressed, ..
                    },
                    _,
                ) => *key_down_pressed = true,
                (VirtualKeyCode::W, _, PongPlayer::Keyboard { key_up_pressed, .. }) => {
                    *key_up_pressed = true
                }
                (
                    VirtualKeyCode::S,
                    _,
                    PongPlayer::Keyboard {
                        key_down_pressed, ..
                    },
                ) => *key_down_pressed = true,
                _ => {}
            }
        }

        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, input: KeyInput) -> Result<(), GameError> {
        let player_one = &mut self.game.player.0;
        let player_two = &mut self.game.player.1;

        if let Some(key) = input.keycode {
            match (key, player_one, player_two) {
                (VirtualKeyCode::Up, PongPlayer::Keyboard { key_up_pressed, .. }, _) => {
                    *key_up_pressed = false
                }
                (
                    VirtualKeyCode::Down,
                    PongPlayer::Keyboard {
                        key_down_pressed, ..
                    },
                    _,
                ) => *key_down_pressed = false,
                (VirtualKeyCode::W, _, PongPlayer::Keyboard { key_up_pressed, .. }) => {
                    *key_up_pressed = false
                }
                (
                    VirtualKeyCode::S,
                    _,
                    PongPlayer::Keyboard {
                        key_down_pressed, ..
                    },
                ) => *key_down_pressed = false,
                _ => {}
            }
        }

        Ok(())
    }
}

fn main() -> GameResult {
    let (ctx, events_loop) = ggez::ContextBuilder::new("pong", "")
        .window_setup(ggez::conf::WindowSetup::default().title("Pong!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(720.0, 480.0))
        .build()?;

    let state = Pong::new(PongPlayer::keyboard(), PongPlayer::keyboard());
    event::run(ctx, events_loop, state)
}
