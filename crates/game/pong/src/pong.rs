use game_lib::{Game};
use ggez::glam::{vec2, Vec2};
use ggez::graphics::{Canvas, Color, Rect, Text, TextAlign, TextLayout};
use ggez::winit::event::VirtualKeyCode;
use ggez::{event, graphics, input::keyboard::KeyInput, Context, GameError, GameResult};
use crate::game::{PongGame, PongPlayer, PongPlayerInput, PLAYER_HEIGHT, PLAYER_WIDTH};

pub struct Pong {
    game: PongGame,
}

impl Pong {
    pub fn new(player_one: PongPlayer, player_two: PongPlayer) -> Self {
        Pong {
            game: PongGame::new(player_one, player_two),
        }
    }

    fn draw_players(&mut self, canvas: &mut Canvas) {
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(Rect::new(
                    0.,
                    self.game.player.0.pos(),
                    PLAYER_WIDTH,
                    PLAYER_HEIGHT,
                ))
                .color(Color::WHITE),
        );

        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(Rect::new(
                    1. - PLAYER_WIDTH,
                    self.game.player.1.pos(),
                    PLAYER_WIDTH,
                    PLAYER_HEIGHT,
                ))
                .color(Color::WHITE),
        );
    }

    fn draw_ball(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> Result<(), GameError> {
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

        Ok(())
    }

    fn draw_score(&mut self, canvas: &mut Canvas) {
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
    }
}

impl event::EventHandler<GameError> for Pong {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.game.tick_model();
        self.game.tick(ctx.time.delta());
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
        canvas.set_screen_coordinates(Rect::new(0., 0., 1., 1.));

        self.draw_players(&mut canvas);
        self.draw_ball(ctx, &mut canvas)?;
        self.draw_score(&mut canvas);

        canvas.finish(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        let player_one = &mut self.game.player.0.input_mut();
        let player_two = &mut self.game.player.1.input_mut();

        if let Some(key) = input.keycode {
            match (key, player_one, player_two) {
                (VirtualKeyCode::Up, PongPlayerInput::Keyboard { up_pressed, .. }, _) => {
                    *up_pressed = true
                }
                (VirtualKeyCode::Down, PongPlayerInput::Keyboard { down_pressed, .. }, _) => {
                    *down_pressed = true
                }
                (VirtualKeyCode::W, _, PongPlayerInput::Keyboard { up_pressed, .. }) => {
                    *up_pressed = true
                }
                (VirtualKeyCode::S, _, PongPlayerInput::Keyboard { down_pressed, .. }) => {
                    *down_pressed = true
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, input: KeyInput) -> Result<(), GameError> {
        let player_one = &mut self.game.player.0.input_mut();
        let player_two = &mut self.game.player.1.input_mut();

        if let Some(key) = input.keycode {
            match (key, player_one, player_two) {
                (VirtualKeyCode::Up, PongPlayerInput::Keyboard { up_pressed, .. }, _) => {
                    *up_pressed = false
                }
                (VirtualKeyCode::Down, PongPlayerInput::Keyboard { down_pressed, .. }, _) => {
                    *down_pressed = false
                }
                (VirtualKeyCode::W, _, PongPlayerInput::Keyboard { up_pressed, .. }) => {
                    *up_pressed = false
                }
                (VirtualKeyCode::S, _, PongPlayerInput::Keyboard { down_pressed, .. }) => {
                    *down_pressed = false
                }
                _ => {}
            }
        }

        Ok(())
    }
}