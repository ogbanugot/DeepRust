use crate::function::Function;
use crate::Tensor2;
use ndarray::{ArrayBase, OwnedRepr, Ix2};

pub struct ReLu {
}
impl ReLu{
    pub fn new(){}
}
pub struct ReLuData{
    pub output: Tensor2,
    pub derivative: Tensor2,
}
impl Function for ReLuData {
    fn output(mut self, input:ArrayBase<OwnedRepr<f64>, Ix2>) -> ArrayBase<OwnedRepr<f64>, Ix2>{
        self.output = input.mapv(|input| {
            if input>0.0 {1.0}else{0.0}
        });
        self.derivative(input);
        self.output
    }
    fn derivative(mut self, input:ArrayBase<OwnedRepr<f64>, Ix2>) -> ArrayBase<OwnedRepr<f64>, Ix2>{
        self.derivative = input.mapv(|input| {
            if input<0.0 {0.0}else{input}
        });
        self.derivative
    }
}
pub struct Sigmoid {
    pub output: Tensor2,
    pub derivative: Tensor2,
}

pub struct Tanh {
    pub output: Tensor2,
    pub derivative: Tensor2,
}


impl Function for Sigmoid {
    fn output(mut self, _:Tensor2) -> Tensor2{
        unimplemented!()
    }
    fn derivative(mut self, _:Tensor2) -> Tensor2{
        unimplemented!()
    }
}

impl Function for Tanh {
    fn output(mut self, _:Tensor2) -> Tensor2{
        unimplemented!()
    }
    fn derivative(&mut self, _:Tensor2) -> Tensor2{
        unimplemented!()
    }
}
