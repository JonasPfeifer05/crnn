// use crate::activation_function::ActivationFunction;
// use crate::thinking_layer::ThinkingLayer;
// use rand::Rng;
// 
// pub struct Neuron {
//     bias: f64,
//     delay: usize,
//     activation_function: ActivationFunction,
// }
// 
// impl Neuron {
//     pub fn new(bias: f64, delay: usize, activation_function: ActivationFunction) -> Self {
//         Self {
//             bias,
//             delay,
//             activation_function,
//         }
//     }
// 
//     pub fn random<R: Rng>(rng: &mut R, activation_function: ActivationFunction) -> Self {
//         Self {
//             // TODO remove magic numbers
//             bias: rng.random::<f64>() * 2.0 - 1.0,
//             delay: rng.random_range(1..=2),
//             activation_function,
//         }
//     }
// 
//     pub fn activate(&self, activate_index: usize, thinking_layer: &ThinkingLayer) -> f64 {
//         let value: f64 = (0..thinking_layer.internal_size())
//             .filter(|i| i != &activate_index)
//             .map(|neuron_index| {
//                 let neuron_value = thinking_layer.neuron_states()[neuron_index];
// 
//                 let mut weight_index =
//                     neuron_index * (thinking_layer.internal_size() - 1) + activate_index;
//                 if activate_index >= neuron_index {
//                     weight_index -= 1;
//                 }
//                 let weight = thinking_layer.weights()[weight_index];
// 
//                 neuron_value * weight
//             })
//             .sum();
// 
//         self.activation_function.apply(value + self.bias)
//     }
// 
//     pub fn bias(&self) -> f64 {
//         self.bias
//     }
// 
//     pub fn delay(&self) -> usize {
//         self.delay
//     }
// }
