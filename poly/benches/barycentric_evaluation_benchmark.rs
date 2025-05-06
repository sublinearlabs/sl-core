use criterion::{Criterion, black_box, criterion_group, criterion_main};
use p3_field::extension::BinomialExtensionField;
use p3_mersenne_31::Mersenne31;
use poly::{Fields, utils::barycentric_evaluation};

type F = Mersenne31;
type E = BinomialExtensionField<Mersenne31, 3>;

fn barycentric_evaluation_benchmark(c: &mut Criterion) {
    //c.bench_function("barycentric_evaluation", |b| {
    //    b.iter(|| {
    //        let poly_evaluation = vec![
    //            Fields::Base(F::new(428910)),
    //            Fields::Base(F::new(17234)),
    //            Fields::Base(F::new(59102)),
    //            Fields::Base(F::new(3679)),
    //            Fields::Base(F::new(28465)),
    //            Fields::Base(F::new(51007)),
    //            Fields::Base(F::new(12398)),
    //            Fields::Base(F::new(64020)),
    //            Fields::Base(F::new(37561)),
    //            Fields::Base(F::new(8946)),
    //            Fields::Base(F::new(23175)),
    //            Fields::Base(F::new(49303)),
    //            Fields::Base(F::new(61428)),
    //            Fields::Base(F::new(5916)),
    //            Fields::Base(F::new(33782)),
    //            Fields::Base(F::new(19054)),
    //            Fields::Base(F::new(55629)),
    //            Fields::Base(F::new(41963)),
    //            Fields::Base(F::new(2847)),
    //            Fields::Base(F::new(60215)),
    //        ];
    //
    //        black_box(barycentric_evaluation::<F, E>(
    //            &poly_evaluation,
    //            &Fields::Base(F::new(10)),
    //        ))
    //    })
    //});
}

criterion_group!(benches, barycentric_evaluation_benchmark,);

criterion_main!(benches);
