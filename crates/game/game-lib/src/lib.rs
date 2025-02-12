use std::time::Duration;

pub trait Game {
    fn tick(&mut self, delta_time: Duration);
}
