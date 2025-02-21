use crate::game_impl::DrivingGame;
use crate::player::{PlayerInput, PLAYER_HEIGHT, PLAYER_WIDTH};
use game_lib::Game;
use ggez::glam::{Mat4, Vec2, Vec3};
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, PxScale, Rect, Text};
use ggez::graphics::{Image, MeshBuilder};
use ggez::input::keyboard::KeyInput;
use ggez::winit::event::VirtualKeyCode;
use ggez::{event, Context, GameError};
use std::default::Default;
use std::f32::consts::{FRAC_PI_2, PI};

pub const PIXELS_PER_METER: f32 = 35.0;
pub const SCREEN_WIDTH: usize = 2000;
pub const SCREEN_HEIGHT: usize = 1500;
pub const UI_FONT_SIZE: f32 = 30.0;
pub const TRACK_WIDTH: f32 = 13.0 * PIXELS_PER_METER;
pub const TRACK_BORDER_WIDTH: f32 = 0.5 * PIXELS_PER_METER;
pub const TRACK_SEGMENT_WIDTH: f32 = 0.5 * PIXELS_PER_METER;
pub const TRACK_SEGMENT_LENGTH: f32 = 2.0 * PIXELS_PER_METER;

pub struct DrivingGui {
    game: DrivingGame,
    car_image: Image,
}

impl DrivingGui {
    pub fn new(game: DrivingGame, car_image: Image) -> Self {
        Self { game, car_image }
    }

    pub fn get_player_center(&self) -> Vec2 {
        let current_position = self.game.player.current_position;
        let current_direction = Vec2::from_angle(self.game.player.direction - FRAC_PI_2);
        current_position + current_direction * (PLAYER_HEIGHT / 2.0)
    }

    pub fn draw_player(&self, canvas: &mut Canvas, _ctx: &mut Context) -> Result<(), GameError> {
        let current_position = self.game.player.current_position;

        canvas.draw(
            &self.car_image,
            DrawParam::new()
                .dest_rect(Rect::new(
                    current_position.x,
                    current_position.y,
                    PLAYER_WIDTH / self.car_image.width() as f32,
                    PLAYER_HEIGHT / self.car_image.height() as f32,
                ))
                .offset(Vec2::new(0.5, 1.0))
                .rotation(self.game.player.direction),
        );

        Ok(())
    }

    pub fn draw_track(&self, canvas: &mut Canvas, ctx: &mut Context) -> Result<(), GameError> {
        let mesh = Mesh::new_polyline(
            ctx,
            DrawMode::stroke(TRACK_WIDTH),
            &self.game.track.points,
            Color::new(0.25, 0.25, 0.25, 1.0),
        )?;
        canvas.draw(
            &mesh,
            DrawParam::default().dest_rect(Rect::new(0.0, 0.0, 1.0, 1.0)),
        );

        let (left_border, right_border) = self.game.track.borders();

        let mesh = Mesh::new_polyline(
            ctx,
            DrawMode::stroke(TRACK_BORDER_WIDTH),
            &left_border,
            Color::new(1.0, 0.0, 0.0, 1.0),
        )?;
        canvas.draw(
            &mesh,
            DrawParam::default().dest_rect(Rect::new(0.0, 0.0, 1.0, 1.0)),
        );

        let mesh = Mesh::new_polyline(
            ctx,
            DrawMode::stroke(TRACK_BORDER_WIDTH),
            &right_border,
            Color::new(1.0, 0.0, 0.0, 1.0),
        )?;
        canvas.draw(
            &mesh,
            DrawParam::default().dest_rect(Rect::new(0.0, 0.0, 1.0, 1.0)),
        );

        let mut mesh_builder = MeshBuilder::new();
        for i in 1..self.game.track.points.len() {
            let front = self.game.track.points[i];
            let back = self.game.track.points[i - 1];

            let direction = (front - back).normalize();
            let segment_count = ((front - back).length() / TRACK_SEGMENT_LENGTH / 2.0) as usize;

            for i in 0..segment_count {
                mesh_builder.line(
                    &[
                        back + direction * (i as f32 * 2.0) * TRACK_SEGMENT_LENGTH,
                        back + direction * (i as f32 * 2.0 + 1.0) * TRACK_SEGMENT_LENGTH,
                    ],
                    TRACK_SEGMENT_WIDTH,
                    Color::WHITE,
                )?;
            }
        }
        canvas.draw(
            &Mesh::from_data(ctx, mesh_builder.build()),
            DrawParam::default().dest_rect(Rect::new(0.0, 0.0, 1.0, 1.0)),
        );

        Ok(())
    }

    pub fn draw_point(
        &self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        position: Vec2,
        radius: f32,
        color: Color,
    ) -> Result<(), GameError> {
        let mesh = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            radius,
            1.0,
            color,
        )?;
        canvas.draw(
            &mesh,
            DrawParam::default().dest_rect(Rect::new(position.x, position.y, 1.0, 1.0)),
        );
        Ok(())
    }

    pub fn draw_car_origin(&self, canvas: &mut Canvas, ctx: &mut Context) -> Result<(), GameError> {
        let current_position = self.game.player.current_position;
        self.draw_point(canvas, ctx, current_position, 7.0, Color::BLUE)
    }

    pub fn draw_car_center(&self, canvas: &mut Canvas, ctx: &mut Context) -> Result<(), GameError> {
        let player_center = self.get_player_center();
        self.draw_point(canvas, ctx, player_center, 7.0, Color::RED)
    }

    pub fn draw_screen_center(
        &self,
        canvas: &mut Canvas,
        ctx: &mut Context,
    ) -> Result<(), GameError> {
        self.draw_point(
            canvas,
            ctx,
            Vec2::new(SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0),
            7.0,
            Color::GREEN,
        )
    }

    pub fn draw_stats(&self, canvas: &mut Canvas, _ctx: &mut Context) -> Result<(), GameError> {
        let velocity = self.game.player.velocity;

        let mut velocity_display = Text::new(format!(
            "Current speed: {:.2} km/h",
            velocity.abs() / PIXELS_PER_METER * 3.6
        ));
        velocity_display.set_scale(PxScale {
            x: UI_FONT_SIZE,
            y: UI_FONT_SIZE,
        });

        canvas.draw(&velocity_display, DrawParam::default().color(Color::BLACK));

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

        let camera_position = self.get_player_center();
        let camera_rotation = -(self.game.player.direction - PI);

        // let camera_position = Vec2::new(0.0, 0.0);
        // let camera_rotation = 0.0;

        let transformation = Mat4::IDENTITY
            * Mat4::from_scale((2.0 / SCREEN_WIDTH as f32, 2.0 / SCREEN_HEIGHT as f32, 1.0).into())
            * Mat4::from_rotation_z(camera_rotation)
            * Mat4::from_translation((-camera_position, 0.0).into());
        canvas.set_projection(transformation);

        self.draw_track(&mut canvas, ctx)?;
        self.draw_player(&mut canvas, ctx)?;
        self.draw_car_origin(&mut canvas, ctx)?;
        self.draw_car_center(&mut canvas, ctx)?;

        canvas.set_projection(
            Mat4::IDENTITY
                * Mat4::from_translation(Vec3::new(-1.0, 1.0, 0.0))
                * Mat4::from_scale(
                    (2.0 / SCREEN_WIDTH as f32, -2.0 / SCREEN_HEIGHT as f32, 1.0).into(),
                ),
        );

        self.draw_stats(&mut canvas, ctx)?;
        self.draw_screen_center(&mut canvas, ctx)?;

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
