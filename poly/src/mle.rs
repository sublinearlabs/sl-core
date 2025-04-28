use p3_field::Field;

struct MLE<F: Field> {
    evaluations: Vec<F>,
}

impl<F: Field> MLE<F> {
    fn partial_evalute(points: &[F]) -> F {
        todo!()
    }

    fn evaluate(points: &[F]) -> F {
        todo!()
    }
}
