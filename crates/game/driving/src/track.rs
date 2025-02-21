use crate::gui::{PIXELS_PER_METER, TRACK_WIDTH};
use ggez::glam::Vec2;
use std::f32::consts::{FRAC_PI_2, PI};

pub struct Track {
    pub points: Vec<Vec2>,
}

impl Track {
    pub fn borders(&self) -> (Vec<Vec2>, Vec<Vec2>) {
        let mut left = vec![];
        let mut right = vec![];

        let start_direction = (self.points[1] - self.points[0]).normalize();
        left.push(
            self.points[0] + Vec2::new(start_direction.y, -start_direction.x) * TRACK_WIDTH / 2.0,
        );
        right.push(
            self.points[0] + Vec2::new(-start_direction.y, start_direction.x) * TRACK_WIDTH / 2.0,
        );

        for i in 2..self.points.len() {
            let front_point = self.points[i];
            let center_point = self.points[i - 1];
            let back_point = self.points[i - 2];

            let front_direction = (front_point - center_point).normalize();
            let back_direction = (back_point - center_point).normalize();
            let mut corner_direction = (front_direction + back_direction).normalize_or_zero();
            if corner_direction.length_squared() == 0.0 {
                corner_direction = Vec2::new(front_direction.y, -front_direction.x);
            } else {
                let correction =
                    (back_direction.y * corner_direction.x).abs() +
                        (-back_direction.x * corner_direction.y).abs();
                corner_direction /= correction;
            }

            // let mut corner_angle = (front_direction
            //     .angle_between(Vec2::new(-corner_direction.y,corner_direction.x)));
            // if corner_angle > FRAC_PI_2 {
            //     corner_angle = PI - corner_angle;
            // }
            // println!("{}: {}", i, corner_angle.to_degrees());
            // let max_factor = 2.0f32.sqrt() - 1.0;
            // let corner_factor = 1.0 + max_factor * (corner_angle / FRAC_PI_2);
            // println!("{}: {}", i, corner_factor);

            let signum = match (corner_direction.dot(back_direction)
                > corner_direction.dot(front_direction))
            {
                true => 1.0,
                false => -1.0,
            };

            left.push(center_point + signum * -corner_direction * TRACK_WIDTH / 2.0);
            right.push(center_point + signum * corner_direction * TRACK_WIDTH / 2.0);
        }

        let end_direction =
            (self.points[self.points.len() - 1] - self.points[self.points.len() - 2]).normalize();
        left.push(
            self.points[self.points.len() - 1]
                + Vec2::new(end_direction.y, -end_direction.x) * TRACK_WIDTH / 2.0,
        );
        right.push(
            self.points[self.points.len() - 1]
                + Vec2::new(-end_direction.y, end_direction.x) * TRACK_WIDTH / 2.0,
        );

        (left, right)
    }
}

impl Default for Track {
    fn default() -> Self {
        Track {
            points: vec![
                Vec2::new(0.0, -100.0 * PIXELS_PER_METER), // Start
                Vec2::new(0.0, 200.0 * PIXELS_PER_METER),  // 300m straight
                Vec2::new(-10.0 * PIXELS_PER_METER, 250.0 * PIXELS_PER_METER), // 300m straight
                Vec2::new(-50.0 * PIXELS_PER_METER, 300.0 * PIXELS_PER_METER), // 300m swirl
                Vec2::new(-90.0 * PIXELS_PER_METER, 350.0 * PIXELS_PER_METER), // 300m swirl
                Vec2::new(-100.0 * PIXELS_PER_METER, 400.0 * PIXELS_PER_METER), // 300m swirl
                Vec2::new(-100.0 * PIXELS_PER_METER, 900.0 * PIXELS_PER_METER), // 300m straight
                Vec2::new(-90.0 * PIXELS_PER_METER, 1000.0 * PIXELS_PER_METER), // 300m straight
                Vec2::new(0.0, 1010.0 * PIXELS_PER_METER), //
            ],
        }
    }
}
