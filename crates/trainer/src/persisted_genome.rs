use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PersistedGenome {
    pub genome: Vec<f64>,
    pub score: f32,
}
