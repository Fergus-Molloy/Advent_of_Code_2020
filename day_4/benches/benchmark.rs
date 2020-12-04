use day_4::main::{part_one, part_two};
use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;

fn criterion_benchmark(c: &mut Criterion) {
    let content = fs::read_to_string("input").expect("unable to read file");
    c.bench_function("part 1", |b| b.iter(|| part_one(&content)));
    c.bench_function("part 2", |b| b.iter(|| part_two(&content)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
