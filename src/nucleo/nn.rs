use crate::nucleo::tensor::Tensor;
use crate::qud::Qud;

pub fn relu(q: Qud) -> Qud {
    match q {
        Qud::Zero => Qud::Zero,
        Qud::One => Qud::One,
        Qud::Super => Qud::One,
        Qud::Error => Qud::Zero,
    }
}

pub fn step(q: Qud) -> Qud {
    match q {
        Qud::Zero => Qud::Zero,
        Qud::One => Qud::One,
        Qud::Super => Qud::One,
        Qud::Error => Qud::Zero,
    }
}

pub fn dense_layer(inputs: &Tensor<Qud>, weights: &Tensor<Qud>, bias: Qud) -> Tensor<Qud> {
    let prod = inputs.times(weights);
    prod.add_scalar(bias)
}
