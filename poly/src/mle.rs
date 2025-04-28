use p3_field::Field;

pub struct MLE<F: Field> {
    evaluations: Vec<F>,
}

impl<F: Field> MLE<F> {
    pub fn partial_evalute(points: &[F]) -> F {
        todo!()
    }

    pub fn evaluate(points: &[F]) -> F {
        todo!()
    }
}
