#[macro_use]
extern crate criterion;
use criterion::Criterion;

use gt_algorithms::*;

fn r26(c: &mut Criterion) {
    c.bench_function("R(3,3)", |b| {
        b.iter(|| {
            let _ = ramsey(3, 3);
        })
    });
}

criterion_group!(benches, r26);
criterion_main!(benches);
