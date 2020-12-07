use criterion::{criterion_group, criterion_main, Criterion};
use day_3::main::get_trees;
use std::fs;

fn criterion_benchmark(c: &mut Criterion) {
    let content = fs::read_to_string("input").expect("unable to read file");
    let lines: Vec<&str> = content.lines().map(|x| x.trim()).collect();
    c.bench_function("part 1", |b| b.iter(|| get_trees(&lines, 3, 1)));
    c.bench_function("part 2", |b| {
        b.iter(|| {
            let x = get_trees(&lines, 1, 1)
                * get_trees(&lines, 3, 1)
                * get_trees(&lines, 5, 1)
                * get_trees(&lines, 7, 1)
                * get_trees(&lines, 1, 2);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
