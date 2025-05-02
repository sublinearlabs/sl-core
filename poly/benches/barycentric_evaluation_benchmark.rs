use criterion::{black_box, criterion_group, criterion_main, Criterion};


fn barycentric_evaluation_benchmark(c: &mut Criterion) {
    let poly_evaluation = vec![
        42891, 
        17234, 
        59102, 
        3679, 
        28465, 
        51007, 
        12398, 
        64020, 
        37561, 
        8946, 
        23175, 
        49303, 
        61428, 
        5916, 
        33782, 
        19054, 
        55629, 
        41963, 
        2847, 
        60215
    ];
    
    let ba
}

criterion_group!(
    benches,
    barycentric_evaluation_benchmark,
);

criterion_main!(benches);
