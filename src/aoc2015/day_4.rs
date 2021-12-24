pub fn part_1(input: &str) -> u128 {
    for n in 1.. {
        let digest = md5::compute(format!("{}{}", input, n));
        let hash = format!("{:x}", digest);

        if hash.starts_with("00000") {
            println!("{}", hash);
            return n;
        }
    }

    unreachable!();
}

pub fn part_2(input: &str) -> u128 {
    for n in 1.. {
        let digest = md5::compute(format!("{}{}", input, n));
        let hash = format!("{:x}", digest);

        if hash.starts_with("000000") {
            println!("{}", hash);
            return n;
        }
    }

    unreachable!();
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
