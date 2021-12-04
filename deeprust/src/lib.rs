use ndarray::{ArrayBase, OwnedRepr, Ix1,Ix2,Ix3,Ix4,ArcArray};
mod layer;
mod weight;
mod function;

pub type Tensor1 = ArrayBase<OwnedRepr<f64>, Ix1>;
pub type Tensor2 = ArrayBase<OwnedRepr<f64>, Ix2>;
pub type Tensor3 = ArrayBase<OwnedRepr<f64>, Ix3>;
pub type Tensor4 = ArrayBase<OwnedRepr<f64>, Ix4>;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        unimplemented!()
    }
}