use criterion::{criterion_group, criterion_main, Criterion};
use rust_aoc::aoc2015;
use std::fs::File;
use std::io::Read;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut f = File::open("res/2015/day_1.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    c.bench_function("day 1 part 1", |b| b.iter(|| aoc2015::day_1::part_1(&input)));
    c.bench_function("day 1 part 2", |b| b.iter(|| aoc2015::day_1::part_2(&input)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
