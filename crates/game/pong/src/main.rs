use crate::game::{PongGame, PongPlayer};
use game_lib::Game;
use ggez::glam::{vec2, Vec2};
use ggez::graphics::{Color, Rect, Text, TextAlign, TextLayout};
use ggez::mint::Vector2;
use ggez::winit::event::VirtualKeyCode;
use ggez::{
    event, graphics,
    input::keyboard::{KeyCode, KeyInput},
    Context, GameError, GameResult,
};

const PLAYER_HEIGHT: f32 = 0.2;
const PLAYER_WIDTH: f32 = 0.02;

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
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from(Color::BLACK));

        canvas.set_screen_coordinates(Rect::new(0., 0., 1., 1.));

        let pos_0 = self.game.state.player_pos.0;
        let pos_1 = self.game.state.player_pos.1;

        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(Rect::new(0., pos_0, PLAYER_WIDTH, PLAYER_HEIGHT))
                .color(Color::WHITE),
        );

        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(Rect::new(
                    1. - PLAYER_WIDTH,
                    pos_1,
                    PLAYER_WIDTH,
                    PLAYER_HEIGHT,
                ))
                .color(Color::WHITE),
        );

        let mb = &mut graphics::MeshBuilder::new();

        mb.circle(
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            0.01,
            0.001,
            Color::RED,
        )?;

        let mesh = graphics::Mesh::from_data(ctx, mb.build());

        canvas.draw(
            &mesh,
            graphics::DrawParam::new().dest(self.game.state.ball_pos),
        );

        let score = self.game.state.score;
        let mut score_display = Text::new(format!("{} / {}", score.0, score.1));
        score_display
            .set_bounds(Vec2::new(f32::INFINITY, f32::INFINITY))
            .set_layout(TextLayout {
                h_align: TextAlign::Middle,
                v_align: TextAlign::Begin,
            });

        canvas.draw(
            &score_display,
            graphics::DrawParam::from([0.5, 0.0])
                .color(Color::WHITE)
                .scale(vec2(0.002, 0.002)),
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
                (VirtualKeyCode::Up, PongPlayer::Keyboard { up_pressed, .. }, _) => {
                    *up_pressed = true
                }
                (VirtualKeyCode::Down, PongPlayer::Keyboard { down_pressed, .. }, _) => {
                    *down_pressed = true
                }
                (VirtualKeyCode::W, _, PongPlayer::Keyboard { up_pressed, .. }) => {
                    *up_pressed = true
                }
                (VirtualKeyCode::S, _, PongPlayer::Keyboard { down_pressed, .. }) => {
                    *down_pressed = true
                }
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
                (VirtualKeyCode::Up, PongPlayer::Keyboard { up_pressed, .. }, _) => {
                    *up_pressed = false
                }
                (VirtualKeyCode::Down, PongPlayer::Keyboard { down_pressed, .. }, _) => {
                    *down_pressed = false
                }
                (VirtualKeyCode::W, _, PongPlayer::Keyboard { up_pressed, .. }) => {
                    *up_pressed = false
                }
                (VirtualKeyCode::S, _, PongPlayer::Keyboard { down_pressed, .. }) => {
                    *down_pressed = false
                }
                _ => {}
            }
        }

        Ok(())
    }
}

fn main() -> GameResult {
    let (ctx, events_loop) = ggez::ContextBuilder::new("pong", "")
        .window_setup(ggez::conf::WindowSetup::default().title("Pong!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(720.0, 720.0))
        .build()?;

    let state = Pong::new(PongPlayer::keyboard(), PongPlayer::Sync);
    event::run(ctx, events_loop, state)
}
