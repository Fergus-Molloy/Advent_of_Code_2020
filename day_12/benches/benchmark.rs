use criterion::{criterion_group, criterion_main, Criterion};
use day_12::main::{part_one, part_two};
use std::fs;

fn criterion_benchmark(c: &mut Criterion) {
    let contents = fs::read_to_string("input").expect("unable to read file");
    let lines: Vec<String> = contents.lines().map(|x| x.to_string()).collect();
    c.bench_function("part 1", |b| b.iter(|| part_one(&lines)));
    c.bench_function("part 2", |b| b.iter(|| part_two(&lines)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
