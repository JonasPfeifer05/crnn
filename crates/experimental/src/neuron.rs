#[derive(Clone)]
pub struct Neuron {
    pub bias: f32,
    pub delay: usize,
    pub threshold: f32,
}

impl Neuron {
    pub fn new(bias: f32, delay: usize, threshold: f32) -> Neuron {
        Neuron {
            bias,
            delay,
            threshold,
        }
    }

    pub fn activate(&self, value: f32) -> f32 {
        let activated_value = value.tanh();
        if activated_value.abs() < self.threshold {
            0.0
        } else {
            activated_value
        }
    }

}
