use p3_field::Field;

pub struct MLE<F: Field> {
    /// The evaluations of the boolean hypercube {0,1}^n_vars
    evaluations: Vec<F>,
    /// Number of variables
    n_vars: usize,
}

impl<F: Field> MLE<F> {
    pub fn new_from_vec(n_vars: usize, evaluations: Vec<F>) -> Self {
        // assert that the number of variables matches the number of evaluations
        assert_eq!(1 << n_vars, evaluations.len());
        Self {
            evaluations,
            n_vars,
        }
    }

    pub fn partial_evalute(&self, points: &[F]) -> Self {
        todo!()
    }

    pub fn evaluate(&self, points: &[F]) -> F {
        todo!()
    }
}
