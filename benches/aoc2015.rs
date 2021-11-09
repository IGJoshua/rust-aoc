use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_aoc::aoc2015;
use rust_aoc::utils::read_input;

pub fn individual_benchmarks(c: &mut Criterion) {
    let input = read_input("res/2015/day_1.txt").unwrap();
    c.bench_function("2015 day 1 part 1", |b| {
        b.iter(|| aoc2015::day_1::part_1(black_box(&input)))
    });
    c.bench_function("2015 day 1 part 2", |b| {
        b.iter(|| aoc2015::day_1::part_2(black_box(&input)))
    });

    let input = read_input("res/2015/day_2.txt").unwrap();
    c.bench_function("2015 day 2 part 1", |b| {
        b.iter(|| aoc2015::day_2::part_1(black_box(&input)))
    });
    c.bench_function("2015 day 2 part 2", |b| {
        b.iter(|| aoc2015::day_2::part_2(black_box(&input)))
    });

    let input = read_input("res/2015/day_3.txt").unwrap();
    c.bench_function("2015 day 3 part 1", |b| {
        b.iter(|| aoc2015::day_3::part_1(black_box(&input)))
    });
    c.bench_function("2015 day 3 part 2", |b| {
        b.iter(|| aoc2015::day_3::part_2(black_box(&input)))
    });

    let input = read_input("res/2015/day_4.txt").unwrap().trim().to_string();
    c.bench_function("2015 day 4 part 1", |b| {
        b.iter(|| aoc2015::day_4::part_1(black_box(&input)))
    });
    c.bench_function("2015 day 4 part 2", |b| {
        b.iter(|| aoc2015::day_4::part_2(black_box(&input)))
    });
}

pub fn complete_benchmark(c: &mut Criterion) {
    let mut inputs = vec![
        read_input("res/2015/day_1.txt").unwrap(),
        read_input("res/2015/day_2.txt").unwrap(),
        read_input("res/2015/day_3.txt").unwrap(),
        read_input("res/2015/day_4.txt").unwrap(),
    ];

    // Trim the day 4 input
    inputs[3] = inputs[3].to_string().trim().to_string();
    let inputs = inputs;

    c.bench_function("2015 all together", |b| {
        b.iter(|| {
            aoc2015::day_1::part_1(black_box(&inputs[0]));
            aoc2015::day_1::part_2(black_box(&inputs[0]));
            aoc2015::day_2::part_1(black_box(&inputs[1]));
            aoc2015::day_2::part_2(black_box(&inputs[1]));
            aoc2015::day_3::part_1(black_box(&inputs[2]));
            aoc2015::day_3::part_2(black_box(&inputs[2]));
            aoc2015::day_4::part_1(black_box(&inputs[3]));
            aoc2015::day_4::part_2(black_box(&inputs[3]));
        })
    });
}

criterion_group!(benches, individual_benchmarks, complete_benchmark);
criterion_main!(benches);
