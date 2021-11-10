pub fn part_1(input: &str) -> i64 {
    input
        .lines()
        .flat_map(|line| line.chars())
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    input
        .lines()
        .flat_map(|line| line.chars())
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .enumerate()
        .try_fold(0i32, |floor, (idx, dir)| {
            if floor >= 0 {
                Ok(floor + dir)
            } else {
                Err(idx)
            }
        })
        .unwrap_err()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::read_input;

    #[test]
    fn test_part_1() {
        let input = read_input("res/2015/day_1.txt").unwrap();

        let result = part_1(&input);
        assert_eq!(result, 232, "the result was correct floor");
    }

    #[test]
    fn test_part_2() {
        let input = read_input("res/2015/day_1.txt").unwrap();

        let index = part_2(&input);
        assert_eq!(index, 1783, "the result was correct instruction");
    }
}
