use ark_ff::fields::Field;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use whir::{
    crypto::fields::Field64 as F,
    poly_utils::{coeffs::CoefficientList, multilinear::MultilinearPoint},
    sumcheck::SumcheckSingle,
    whir::statement::{Statement, Weights},
};

// fn bench_sumcheck_compress(c: &mut Criterion) {
//     let mut group = c.benchmark_group("sumcheck_compress");

//     for &num_vars in &[1, 2, 4, 8, 12, 14] {
//         let num_points = 1 << num_vars;

//         // Shared setup for polynomial and evaluation
//         let coeffs = (0..num_points)
//             .map(|i| F::from(i as u64 + 1))
//             .collect::<Vec<_>>();
//         let poly = CoefficientList::new(coeffs);

//         let eval_point = MultilinearPoint(vec![F::from(10); num_vars]);
//         let claimed_value = poly.evaluate(&eval_point);

//         let combination_randomness = F::from(42);
//         let folding_randomness = MultilinearPoint(vec![F::from(7); num_vars]);

//         group.bench_with_input(
//             BenchmarkId::from_parameter(num_vars),
//             &num_vars,
//             |b, &_n| {
//                 b.iter(|| {
//                     // Reset prover and statement on each iteration
//                     let mut statement = Statement::new(num_vars);
//                     let weights = Weights::evaluation(eval_point.clone());
//                     statement.add_constraint(weights, claimed_value);

//                     let mut prover = SumcheckSingle::new(poly.clone(), &statement, F::ONE);
//                     let poly_1 = prover.compute_sumcheck_polynomial();

//                     prover.compress(combination_randomness, &folding_randomness, &poly_1);
//                 });
//             },
//         );
//     }

//     group.finish();
// }

// criterion_group!(benches, bench_sumcheck_compress);
// criterion_main!(benches);

fn bench_compress(c: &mut Criterion) {
    let mut group = c.benchmark_group("compress");

    for &size in &[4u64, 64, 1024, 1 << 14, 1 << 20] {
        // Skip invalid cases
        if size < 2 || !size.is_power_of_two() {
            continue;
        }

        let num_vars = size.trailing_zeros() as usize;
        let base_coeffs: Vec<_> = (0..size).map(F::from).collect();
        let eval_point = MultilinearPoint(vec![F::from(1); num_vars]);
        let combination_randomness = F::from(42);
        let folding_randomness = MultilinearPoint(vec![F::from(4999)]);

        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &_s| {
            b.iter(|| {
                // Reset everything on each iteration
                let coeffs = CoefficientList::new(base_coeffs.clone());
                let value = coeffs.evaluate(&eval_point);

                let mut statement = Statement::new(num_vars);
                let weights = Weights::evaluation(eval_point.clone());
                statement.add_constraint(weights, value);

                let mut prover = SumcheckSingle::new(coeffs, &statement, F::ONE);
                let sumcheck_poly = prover.compute_sumcheck_polynomial();

                prover.compress(combination_randomness, &folding_randomness, &sumcheck_poly);
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_compress);
criterion_main!(benches);
