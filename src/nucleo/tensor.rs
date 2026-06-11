use crate::qud::Qud;

pub struct Tensor<T> {
    data: Vec<T>,
    shape: Vec<usize>,
}

impl<T: Clone> Tensor<T> {
    pub fn new(shape: Vec<usize>, data: Vec<T>) -> Self {
        let expected: usize = shape.iter().product();
        assert!(
            data.len() == expected,
            "Tensor data length does not match shape"
        );
        Tensor { data, shape }
    }

    pub fn shape(&self) -> &[usize] {
        &self.shape
    }

    pub fn rank(&self) -> usize {
        self.shape.len()
    }
}

impl Tensor<Qud> {
    pub fn zeros(shape: &[usize]) -> Self {
        let len: usize = shape.iter().product();
        Tensor {
            data: vec![Qud::Zero; len],
            shape: shape.to_vec(),
        }
    }

    pub fn get(&self, indices: &[usize]) -> Qud {
        assert!(indices.len() == self.shape.len(), "Index rank mismatch");
        let mut idx = 0usize;
        for (i, &dim) in self.shape.iter().enumerate() {
            assert!(indices[i] < dim, "Index out of bounds");
            idx = idx * dim + indices[i];
        }
        self.data[idx]
    }

    pub fn set(&mut self, indices: &[usize], value: Qud) {
        assert!(indices.len() == self.shape.len(), "Index rank mismatch");
        let mut idx = 0usize;
        for (i, &dim) in self.shape.iter().enumerate() {
            assert!(indices[i] < dim, "Index out of bounds");
            idx = idx * dim + indices[i];
        }
        self.data[idx] = value;
    }

    pub fn collapse(&self) -> Tensor<Qud> {
        let data: Vec<Qud> = self
            .data
            .iter()
            .map(|&q| match q {
                Qud::Super => Qud::One,
                Qud::Error => Qud::Zero,
                other => other,
            })
            .collect();
        Tensor {
            data,
            shape: self.shape.clone(),
        }
    }

    pub fn add_scalar(&self, scalar: Qud) -> Tensor<Qud> {
        let data: Vec<Qud> = self.data.iter().map(|&q| q + scalar).collect();
        Tensor {
            data,
            shape: self.shape.clone(),
        }
    }

    pub fn times(&self, other: &Tensor<Qud>) -> Tensor<Qud> {
        assert_eq!(self.shape, other.shape, "Tensor::times requires same shape");
        let data: Vec<Qud> = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a * b)
            .collect();
        Tensor {
            data,
            shape: self.shape.clone(),
        }
    }
}
