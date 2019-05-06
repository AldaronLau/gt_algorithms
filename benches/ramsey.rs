#[macro_use]
extern crate criterion;
use criterion::Criterion;

use gt_algorithms::*;

fn r26(c: &mut Criterion) {
    c.bench_function("R(2,6)", |b| b.iter(|| {
        let _ = ramsey(2, 6);
    }));
}

criterion_group!(benches, r26);
criterion_main!(benches);
