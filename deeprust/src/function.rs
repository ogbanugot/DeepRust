use ndarray::{ArrayBase, OwnedRepr,Ix2};

pub trait Function{
    fn output(mut self, input:ArrayBase<OwnedRepr<f64>, Ix2>) -> ArrayBase<OwnedRepr<f64>, Ix2>;
    fn derivative(mut self, input:ArrayBase<OwnedRepr<f64>, Ix2>) -> ArrayBase<OwnedRepr<f64>, Ix2>;
}
mod non_linear;

pub struct Linear {
    pub output: ArrayBase<OwnedRepr<f64>, Ix2>,
    pub derivative: ArrayBase<OwnedRepr<f64>, Ix2>,
}

impl Function for Linear {
    fn output(mut self, _:ArrayBase<OwnedRepr<f64>, Ix2>) -> ArrayBase<OwnedRepr<f64>, Ix2>{unimplemented!()}
    fn derivative(mut self, _:ArrayBase<OwnedRepr<f64>, Ix2>) -> ArrayBase<OwnedRepr<f64>, Ix2>{unimplemented!()}
}
/*
//Register function here
use crate::function::non_linear::{ReLu, Sigmoid, Tanh};

pub fn get_function(func: &str) -> Box<dyn Function> {
    match func {
        "relu" => {Box::new(ReLu{output:0.0, derivative:0.0})},
        "sigmoid" => {Box::new(Sigmoid{output:0.0, derivative:0.0})},
        "tanh" => {Box::new(Tanh{output:0.0, derivative:0.0})},
        "linear" => {Box::new(Linear{output:0.0, derivative:0.0})},
        _ => {panic!("Function name incorrect, must be one of the following [relu, sigmoid, tanh, linear]")}
    }
}

 */