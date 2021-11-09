use std::collections::HashSet;

fn dir_for_char(c: char) -> (i32, i32){
    match c {
        '>' => (1, 0),
        '<' => (-1, 0),
        '^' => (0, 1),
        'v' => (0, -1),
        _ => (0, 0),
    }
}

pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .flat_map(|line| line.chars())
        .map(dir_for_char)
        .scan((0, 0), |(x, y), (dirx, diry)| {
            *x += dirx;
            *y += diry;
            Some((*x, *y))
        })
        .collect::<HashSet<_>>()
        .len()
}

pub fn part_2(input: &str) -> usize {
    input
        .lines()
        .flat_map(|line| line.chars())
        .map(dir_for_char)
        .enumerate()
        .scan(((0, 0), (0, 0)), |((sx, sy), (rx, ry)), (idx, (dirx, diry))| {
            if idx % 2 == 0 {
                *sx += dirx;
                *sy += diry;
                Some((*sx, *sy))
            } else {
                *rx += dirx;
                *ry += diry;
                Some((*rx, *ry))
            }
        })
        .collect::<HashSet<_>>()
        .len()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::read_input;

    #[test]
    fn test_part_1() {
        let input = read_input("res/2015/day_3.txt").unwrap();

        let result = part_1(&input);
        assert_eq!(result, 2572, "the result was the correct number of houses")
    }

    #[test]
    fn test_part_2() {
        let input = read_input("res/2015/day_3.txt").unwrap();

        let result = part_2(&input);
        assert_eq!(result, 2631, "the result was the correct number of houses")
    }
}
