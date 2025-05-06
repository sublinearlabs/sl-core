use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use ff_ext::{ExtensionField as CenoExtensionField, FromUniformBytes, GoldilocksExt2};
use multilinear_extensions::mle::{
    DenseMultilinearExtension, MultilinearExtension as CenoMultilinearExtension,
};
use p3::goldilocks::Goldilocks as CenoGoldilocks;
use p3_field::{AbstractField, extension::BinomialExtensionField};
use p3_goldilocks::Goldilocks;
use poly::{Fields, MultilinearExtension, mle::MultilinearPoly};
use rand::thread_rng;

type F = Goldilocks;
type E = BinomialExtensionField<Goldilocks, 2>;

fn n_points(n: usize) -> Vec<GoldilocksExt2> {
    let mut rng = thread_rng();
    (0..n)
        .map(|_| CenoGoldilocks::random(&mut rng).into())
        .collect()
}

fn n_points_slcore(n: usize) -> Vec<Fields<F, E>> {
    let points: Vec<GoldilocksExt2> = n_points(n);
    points
        .into_iter()
        .map(|p| Fields::Base(F::from_canonical_u64(p.to_canonical_u64_vec()[0])))
        .collect()
}

fn random_dense_poly_with_eval(
    nv: usize,
    eval_count: usize,
) -> (
    DenseMultilinearExtension<GoldilocksExt2>,
    Vec<GoldilocksExt2>,
) {
    let mut rng = thread_rng();
    assert!(eval_count <= nv);
    (
        DenseMultilinearExtension::random(nv, &mut rng),
        n_points(eval_count),
    )
}

fn random_dense_poly_with_eval_slcore(
    nv: usize,
    eval_count: usize,
) -> (MultilinearPoly<F, E>, Vec<Fields<F, E>>) {
    (
        MultilinearPoly::new_from_vec(nv, n_points_slcore(1 << nv)),
        n_points_slcore(eval_count),
    )
}

fn bench_slcore_against_ceno(c: &mut Criterion) {
    // 20 vars, 1 eval
    c.bench_function("slcore_fix_variables_20_vars_1_eval", |b| {
        let (poly, to_eval) = random_dense_poly_with_eval_slcore(20, 1);
        b.iter(|| poly.partial_evaluate(to_eval.as_slice()))
    });

    c.bench_function("ceno_fix_variables_20_vars_1_eval", |b| {
        let (poly, to_eval) = random_dense_poly_with_eval(20, 1);
        b.iter(|| poly.fix_variables(to_eval.as_slice()))
    });

    c.bench_function(
        "ceno_fix_variables_in_place_20_vars_1_eval (in place)",
        |b| {
            b.iter_batched(
                || random_dense_poly_with_eval(20, 1),
                |(mut poly, to_eval)| poly.fix_variables_in_place(to_eval.as_slice()),
                BatchSize::SmallInput,
            );
        },
    );

    // 20 vars, 10 eval
    c.bench_function("slcore_fix_variables_20_vars_10_eval", |b| {
        let (poly, to_eval) = random_dense_poly_with_eval_slcore(20, 10);
        b.iter(|| poly.partial_evaluate(to_eval.as_slice()))
    });

    c.bench_function("ceno_fix_variables_20_vars_10_eval", |b| {
        let (poly, to_eval) = random_dense_poly_with_eval(20, 10);
        b.iter(|| poly.fix_variables(to_eval.as_slice()))
    });

    c.bench_function(
        "ceno_fix_variables_in_place_20_vars_10_eval (in place)",
        |b| {
            b.iter_batched(
                || random_dense_poly_with_eval(20, 10),
                |(mut poly, to_eval)| poly.fix_variables_in_place(to_eval.as_slice()),
                BatchSize::SmallInput,
            );
        },
    );

    // 23 vars, 1 eval
    c.bench_function("slcore_fix_variables_23_vars_1_eval", |b| {
        let (poly, to_eval) = random_dense_poly_with_eval_slcore(23, 1);
        b.iter(|| poly.partial_evaluate(to_eval.as_slice()))
    });

    c.bench_function("ceno_fix_variables_23_vars_1_eval", |b| {
        let (poly, to_eval) = random_dense_poly_with_eval(23, 1);
        b.iter(|| poly.fix_variables(to_eval.as_slice()))
    });

    c.bench_function(
        "ceno_fix_variables_in_place_23_vars_1_eval (in place)",
        |b| {
            b.iter_batched(
                || random_dense_poly_with_eval(23, 1),
                |(mut poly, to_eval)| poly.fix_variables_in_place(to_eval.as_slice()),
                BatchSize::SmallInput,
            );
        },
    );

    // 23 vars, 10 evals
    c.bench_function("slcore_fix_variables_23_vars_10_eval", |b| {
        let (poly, to_eval) = random_dense_poly_with_eval_slcore(23, 10);
        b.iter(|| poly.partial_evaluate(to_eval.as_slice()))
    });

    c.bench_function("ceno_fix_variables_23_vars_10_eval", |b| {
        let (poly, to_eval) = random_dense_poly_with_eval(23, 10);
        b.iter(|| poly.fix_variables(to_eval.as_slice()))
    });

    c.bench_function(
        "ceno_fix_variables_in_place_23_vars_10_eval (in place)",
        |b| {
            b.iter_batched(
                || random_dense_poly_with_eval(23, 10),
                |(mut poly, to_eval)| poly.fix_variables_in_place(to_eval.as_slice()),
                BatchSize::SmallInput,
            );
        },
    );

    // 25 vars, 1 eval
    c.bench_function("slcore_fix_variables_25_vars_1_eval", |b| {
        let (poly, to_eval) = random_dense_poly_with_eval_slcore(25, 1);
        b.iter(|| poly.partial_evaluate(to_eval.as_slice()))
    });

    c.bench_function("ceno_fix_variables_25_vars_1_eval", |b| {
        let (poly, to_eval) = random_dense_poly_with_eval(25, 1);
        b.iter(|| poly.fix_variables(to_eval.as_slice()))
    });

    c.bench_function(
        "ceno_fix_variables_in_place_25_vars_1_eval (in place)",
        |b| {
            b.iter_batched(
                || random_dense_poly_with_eval(25, 1),
                |(mut poly, to_eval)| poly.fix_variables_in_place(to_eval.as_slice()),
                BatchSize::SmallInput,
            );
        },
    );

    // 25 vars, 10 eval
    c.bench_function("slcore_fix_variables_25_vars_10_eval", |b| {
        let (poly, to_eval) = random_dense_poly_with_eval_slcore(25, 10);
        b.iter(|| poly.partial_evaluate(to_eval.as_slice()))
    });

    c.bench_function("ceno_fix_variables_25_vars_10_eval", |b| {
        let (poly, to_eval) = random_dense_poly_with_eval(25, 10);
        b.iter(|| poly.fix_variables(to_eval.as_slice()))
    });

    c.bench_function(
        "ceno_fix_variables_in_place_25_vars_10_eval (in place)",
        |b| {
            b.iter_batched(
                || random_dense_poly_with_eval(25, 10),
                |(mut poly, to_eval)| poly.fix_variables_in_place(to_eval.as_slice()),
                BatchSize::SmallInput,
            );
        },
    );
}

criterion_group!(benches, bench_slcore_against_ceno);
criterion_main!(benches);
