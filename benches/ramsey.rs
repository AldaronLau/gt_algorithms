#[macro_use]
extern crate criterion;
use criterion::Criterion;

use gt_algorithms::*;

fn r33(c: &mut Criterion) {
    c.bench_function("R(2,3)", |b| b.iter(|| {
        let _ = ramsey(2, 3);
    }));
}

criterion_group!(benches, r33);
criterion_main!(benches);
