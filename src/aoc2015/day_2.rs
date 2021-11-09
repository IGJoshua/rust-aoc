use std::cmp;

fn area(sides: (u32, u32, u32)) -> (u32, u32, u32) {
    let (length, width, height) = sides;
    (length * width, width * height, height * length)
}

fn paper_amount(sides: (u32, u32, u32)) -> u32 {
    let (side_a, side_b, side_c) = sides;
    let smallest = cmp::min(cmp::min(side_a, side_b), side_c);

    2 * (side_a + side_b + side_c) + smallest
}

fn parse_box(line: &str) -> (u32, u32, u32) {
    let mut iter = line.split_terminator('x').map(|s| s.parse().unwrap());
    (
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    )
}

pub fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(parse_box)
        .map(area)
        .map(paper_amount)
        .sum()
}

fn smallest_perimiter(sides: (u32, u32, u32)) -> u32 {
    cmp::min(
        cmp::min(2 * (sides.0 + sides.1), 2 * (sides.1 + sides.2)),
        2 * (sides.2 + sides.0),
    )
}

fn ribbon(sides: (u32, u32, u32)) -> u32 {
    let perimiter = smallest_perimiter(sides);
    let bow = sides.0 * sides.1 * sides.2;

    perimiter + bow
}

pub fn part_2(input: &str) -> u32 {
    input.lines().map(parse_box).map(ribbon).sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::read_input;

    #[test]
    fn test_part_1() {
        let input = read_input("res/2015/day_2.txt").unwrap();

        let result = part_1(&input);
        assert_eq!(result, 1598415, "the result was the correct square footage")
    }

    #[test]
    fn test_part_2() {
        let input = read_input("res/2015/day_2.txt").unwrap();

        let result = part_2(&input);
        assert_eq!(
            result, 3812909,
            "the result was the correct number of feet of ribbon"
        );
    }
}
