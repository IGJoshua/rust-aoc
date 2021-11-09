pub fn part_1(input: &str) -> i64 {
    input.lines()
        .flat_map(|line| line.chars())
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        }).sum()
}

pub fn part_2(input: &str) -> usize {
    let mut floor = 0;
    let mut idx = 0;
    'outer: for line in input.lines() {
        for c in line.chars() {
            idx += 1;

            floor += match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            };

            if floor < 0 {
                break 'outer;
            }
        }
    }

    idx
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_part_1() {
        let mut f = File::open("res/2015/day_1.txt").unwrap();
        let mut input = String::new();
        f.read_to_string(&mut input).unwrap();

        let result = part_1(&input);
        assert_eq!(result, 232, "the result was correct floor");
    }

    #[test]
    fn test_part_2() {
        let mut f = File::open("res/2015/day_1.txt").unwrap();
        let mut input = String::new();
        f.read_to_string(&mut input).unwrap();

        let index = part_2(&input);
        assert_eq!(index, 1783, "the result was correct instruction");
    }
}
