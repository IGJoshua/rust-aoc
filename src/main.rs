use rust_aoc::{aoc2015, utils};
use std::fmt::Display;

fn display_day<F1, F2, Output1, Output2>(year: u32, day: u32, part_1: F1, part_2: F2)
where
    F1: Fn(&str) -> Output1,
    F2: Fn(&str) -> Output2,
    Output1: Display,
    Output2: Display,
{
    let input_file = format!("res/{}/day_{}.txt", year, day);
    let input = utils::read_input(&input_file).unwrap();

    println!("Day {}:", day);
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn aoc2015() {
    println!("2015:");
    display_day(2015, 1, aoc2015::day_1::part_1, aoc2015::day_1::part_2);
    display_day(2015, 2, aoc2015::day_2::part_1, aoc2015::day_2::part_2);
    display_day(2015, 3, aoc2015::day_3::part_1, aoc2015::day_3::part_2);
    display_day(2015, 4, aoc2015::day_4::part_1, aoc2015::day_4::part_2);
}

fn main() {
    println!("Advent of Code\n");
    aoc2015();
}
