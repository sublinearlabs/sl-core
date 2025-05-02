use p3_field::{ExtensionField, Field};

use crate::Fields;

// Evaluate a univariate polynomial in evaluation form
pub fn barycentric_evaluation<F: Field, E: ExtensionField<F>>(
    evaluations: &[Fields<F, E>],
    evaluation_point: &Fields<F, E>,
) -> Fields<F, E> {
    let m_x = (0..evaluations.len()).fold(E::one(), |mut acc, val| {
        acc *= evaluation_point.to_extension_field() - E::from_canonical_usize(val);
        acc
    });

    let mut res = E::zero();

    for i in 0..evaluations.len() {
        let numerator = evaluations[i].to_extension_field();

        let di = (0..evaluations.len())
            .into_iter()
            .filter(|val| *val != i)
            .fold(E::one(), |mut acc, val| {
                acc *= F::from_canonical_usize(i) - F::from_canonical_usize(val);
                acc
            });

        let denominator = di * (evaluation_point.to_extension_field() - E::from_canonical_usize(i));

        res += numerator * denominator.inverse()
    }

    Fields::Extension(m_x * res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use p3_field::{AbstractExtensionField, extension::BinomialExtensionField};
    use p3_mersenne_31::Mersenne31;

    #[test]
    fn test_barycentric_evaluation() {
        // Polynomial in question: 3x + 2
        let poly: Vec<Fields<Mersenne31, BinomialExtensionField<Mersenne31, 3>>> = [2, 3]
            .into_iter()
            .map(|val| Fields::Base(Mersenne31::new(val)))
            .collect();
        let res = barycentric_evaluation(&poly, &Fields::Base(Mersenne31::new(5)));
        assert_eq!(
            res,
            Fields::Extension(AbstractExtensionField::from_base(Mersenne31::new(7)))
        );

        // Polynomial in question: 5x^2 + 3x + 2
        let poly: Vec<Fields<Mersenne31, BinomialExtensionField<Mersenne31, 3>>> =
            [2, 10, 28, 56, 94]
                .into_iter()
                .map(|val| Fields::Base(Mersenne31::new(val)))
                .collect();
        let res = barycentric_evaluation(&poly, &Fields::Base(Mersenne31::new(5)));
        assert_eq!(
            res,
            Fields::Extension(AbstractExtensionField::from_base(Mersenne31::new(142)))
        );
    }
}
