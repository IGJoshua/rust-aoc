extern crate md5;

use std::fmt::Write;

fn search_for(input: &str, search_text: &str) -> i64 {
    let mut num_str = String::with_capacity(input.len() + 32);
    let mut hash = String::with_capacity(64);

    for n in 1.. {
        // Ensure we're starting with empty strings when we write in the input
        num_str.clear();
        hash.clear();

        write!(&mut num_str, "{}{}", input, n).unwrap();
        let digest = md5::compute(&num_str);
        write!(&mut hash, "{:x}", digest).unwrap();

        if hash.starts_with(search_text) {
            return n;
        }
    }

    unreachable!();
}

pub fn part_1(input: &str) -> i64 {
    search_for(input, "00000")
}

pub fn part_2(input: &str) -> i64 {
    search_for(input, "000000")
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::read_input;

    #[test]
    fn part_1_examples() {
        assert_eq!(part_1("abcdef"), 609043);
        assert_eq!(part_1("pqrstuv"), 1048970);
    }

    #[test]
    fn test_part_1() {
        let input = read_input("res/2015/day_4.txt").unwrap().trim().to_string();

        let result = part_1(&input);
        assert_eq!(result, 254575, "the result was the correct index")
    }

    #[test]
    fn test_part_2() {
        let input = read_input("res/2015/day_4.txt").unwrap().trim().to_string();

        let result = part_2(&input);
        assert_eq!(result, 1038736, "the result was the correct index")
    }
}
