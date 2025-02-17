#[derive(Clone, Debug)]
pub enum ActivationFunction {
    Tanh,
    Sigmoid,
    Relu,
    Other(fn(f64) -> f64),
}

impl ActivationFunction {
    pub fn apply(&self, x: f64) -> f64 {
        match self {
            ActivationFunction::Tanh => x.tanh(),
            ActivationFunction::Sigmoid => 1.0 / (1.0 + (-x).exp()),
            ActivationFunction::Relu => {
                if x < 0.0 {
                    0.0
                } else {
                    x
                }
            }
            ActivationFunction::Other(function) => function(x),
        }
    }
}
