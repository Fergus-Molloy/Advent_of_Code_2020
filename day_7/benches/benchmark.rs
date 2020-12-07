use criterion::{criterion_group, criterion_main, Criterion};
use day_7::main::{parse, part_one, part_two};
use std::collections::HashMap;
use std::fs;

fn criterion_benchmark(c: &mut Criterion) {
    let contents = fs::read_to_string("input").expect("unable to read file");
    let lines: Vec<String> = contents.lines().map(|x| x.to_string()).collect();
    c.bench_function("part 1", |b| {
        let map: HashMap<String, Vec<(u32, String)>> = parse(&lines);
        b.iter(|| part_one(&map))
    });
    c.bench_function("part 2", |b| {
        let map: HashMap<String, Vec<(u32, String)>> = parse(&lines);
        b.iter(|| part_two(&map))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
