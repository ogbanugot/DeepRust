use ndarray::{ArrayBase, OwnedRepr,Ix1, Ix3};

#[cfg(test)]
mod tests {

    #[test]
    fn layer_forward() {
    }
}
pub trait Layer: Layer1d + Layer2d {
}

pub trait Layer1d {
    fn forward1d(&mut self, input: ArrayBase<OwnedRepr<f64>, Ix1>)->ArrayBase<OwnedRepr<f64>, Ix1>;
    fn backward1d(&mut self, gradient:ArrayBase<OwnedRepr<f64>, Ix1>)->ArrayBase<OwnedRepr<f64>, Ix1>;
    fn length1d(&self) -> usize;
}
pub trait Layer2d {
    fn forward2d(&self, input: ArrayBase<OwnedRepr<f64>, Ix3>);
    fn backward2d(&self, gradient:ArrayBase<OwnedRepr<f64>, Ix3>);
    fn length2d(&self) -> usize;
}
pub mod neuron{
    pub mod output;
    pub mod hidden;
}
