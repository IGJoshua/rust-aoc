use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_aoc::aoc2015;
use rust_aoc::utils::read_input;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = read_input("res/2015/day_1.txt").unwrap();

    c.bench_function("day 1 part 1", |b| {
        b.iter(|| aoc2015::day_1::part_1(black_box(&input)))
    });
    c.bench_function("day 1 part 2", |b| {
        b.iter(|| aoc2015::day_1::part_2(black_box(&input)))
    });

    let input = read_input("res/2015/day_2.txt").unwrap();

    c.bench_function("day 2 part 1", |b| {
        b.iter(|| aoc2015::day_2::part_1(black_box(&input)))
    });
    c.bench_function("day 2 part 2", |b| {
        b.iter(|| aoc2015::day_2::part_2(black_box(&input)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
