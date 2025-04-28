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
        // ensure we don't have more points than variables
        assert!(points.len() <= self.n_vars);

        let mut new_evaluations = self.evaluations.clone();

        // for each partial point, fold the evaluations in half
        let mut mid_point = new_evaluations.len() / 2;
        for point in points {
            for i in 0..mid_point {
                let left = new_evaluations[i];
                let right = new_evaluations[i + mid_point];
                new_evaluations[i] = match point {
                    a if a.is_zero() => left,
                    a if a.is_one() => right,
                    _ => {
                        // linear interpolation
                        // (1-r) * left + r * right
                        // left - r.left + r.right
                        // left - r (left - right)
                        left - *point * (left - right)
                    }
                }
            }
            mid_point /= 2;
        }

        // truncate and return new polynomial
        let n_vars = self.n_vars - points.len();
        Self {
            evaluations: new_evaluations[..(1 << n_vars)].to_vec(),
            n_vars,
        }
    }

    pub fn evaluate(&self, points: &[F]) -> F {
        // ensure number of points exactly matches number of variables
        assert_eq!(self.n_vars, points.len());
        self.partial_evalute(points).evaluations[0]
    }
}
