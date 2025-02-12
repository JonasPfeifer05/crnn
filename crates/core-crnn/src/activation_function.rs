pub trait ActivationFunction {
    fn apply(value: f64) -> f64;
}

pub struct Tanh;
impl ActivationFunction for Tanh {
    fn apply(value: f64) -> f64 {
        value.tanh()
    }
}

pub struct Sigmoid;
impl Sigmoid {
    fn sigmoid(x: f64) -> f64 {
        1.0 / (1.0 + (-x).exp())
    }
}
impl ActivationFunction for Sigmoid {
    fn apply(value: f64) -> f64 {
        Self::sigmoid(value)
    }
}

pub struct Relu;
impl ActivationFunction for Relu {
    fn apply(value: f64) -> f64 {
        if value > 0.0 {
            value
        } else {
            0.0
        }
    }
}
