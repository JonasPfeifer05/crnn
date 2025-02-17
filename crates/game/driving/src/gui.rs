use crate::game_impl::DrivingGame;
use crate::player::{PlayerInput, PLAYER_HEIGHT, PLAYER_WIDTH};
use game_lib::Game;
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect};
use ggez::input::keyboard::KeyInput;
use ggez::winit::event::VirtualKeyCode;
use ggez::{event, Context, GameError};
use std::default::Default;

pub const PIXELS_PER_METER: f32 = 50.0;
pub const SCREEN_WIDTH: usize = 2000;
pub const SCREEN_HEIGHT: usize = 1000;

pub struct DrivingGui {
    game: DrivingGame,
}

impl DrivingGui {
    pub fn new(game: DrivingGame) -> Self {
        Self { game }
    }

    pub fn draw_player(&self, canvas: &mut Canvas, ctx: &mut Context) -> Result<(), GameError> {
        let current_position = self.game.player.current_position;
        let mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(
                -PLAYER_WIDTH / 2.0,
                -PLAYER_HEIGHT,
                PLAYER_WIDTH,
                PLAYER_HEIGHT,
            ),
            Color::RED,
        )?;

        canvas.draw(
            &mesh,
            DrawParam::default()
                .dest_rect(Rect::new(
                    current_position.x,
                    current_position.y + PLAYER_HEIGHT / 2.0,
                    1.0,
                    1.0,
                ))
                .rotation(self.game.player.direction),
        );
        Ok(())
    }
}

impl event::EventHandler<GameError> for DrivingGui {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        self.game.tick_model();
        self.game.tick(ctx.time.delta());
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = Canvas::from_frame(ctx, Color::WHITE);
        canvas.set_screen_coordinates(Rect::new(
            0.0 - SCREEN_WIDTH as f32 / 2.0,
            0.0 - SCREEN_HEIGHT as f32 / 2.0,
            SCREEN_WIDTH as f32,
            SCREEN_HEIGHT as f32,
        ));

        self.draw_player(&mut canvas, ctx)?;

        canvas.finish(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> Result<(), GameError> {
        let player = &mut self.game.player.input;
        match player {
            PlayerInput::Human { w, a, s, d } => match input.keycode {
                Some(VirtualKeyCode::W) => *w = true,
                Some(VirtualKeyCode::A) => *a = true,
                Some(VirtualKeyCode::S) => *s = true,
                Some(VirtualKeyCode::D) => *d = true,
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, input: KeyInput) -> Result<(), GameError> {
        let player = &mut self.game.player.input;
        match player {
            PlayerInput::Human { w, a, s, d } => match input.keycode {
                Some(VirtualKeyCode::W) => *w = false,
                Some(VirtualKeyCode::A) => *a = false,
                Some(VirtualKeyCode::S) => *s = false,
                Some(VirtualKeyCode::D) => *d = false,
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }
}
