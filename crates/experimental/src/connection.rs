#[derive(Clone)]
pub struct Connection {
    pub weight: f32
}

impl Connection {
    pub fn new(weight: f32) -> Connection {
        Connection {
            weight
        }
    }
}