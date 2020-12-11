use criterion::{criterion_group, criterion_main, Criterion};
use day_11::main::part_one; //, part_two};
use std::fs;

fn criterion_benchmark(c: &mut Criterion) {
    let contents = fs::read_to_string("input").expect("unable to read file");
    let mut seats: [[char; 92]; 90] = [['.'; 92]; 90];
    let mut line = 0;
    let mut col = 0;
    for x in contents.lines() {
        for y in x.chars() {
            seats[line][col] = y;
            col += 1;
        }
        col = 0;
        line += 1;
    }
    c.bench_function("part 1", |b| b.iter(|| part_one(&mut seats)));
    //c.bench_function("part 2", |b| b.iter(|| part_two(&content)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
