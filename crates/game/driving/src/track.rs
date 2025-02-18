use crate::gui::PIXELS_PER_METER;
use ggez::glam::Vec2;

pub struct Track {
    pub points: Vec<Vec2>,
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
            ],
        }
    }
}
