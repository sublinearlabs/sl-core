use p3_field::Field;

pub struct MLE<F: Field> {
    evaluations: Vec<F>,
}

impl<F: Field> MLE<F> {
    pub fn partial_evalute(&self, points: &[F]) -> Self {
        todo!()
    }

    pub fn evaluate(&self, points: &[F]) -> F {
        todo!()
    }
}
