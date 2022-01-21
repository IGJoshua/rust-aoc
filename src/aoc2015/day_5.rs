const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const NAUGHTY_PAIRS: [(char, char); 4] = [('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')];

trait Nice1 {
    fn is_nice(&self) -> bool;
}

impl<T> Nice1 for T
where
    T: AsRef<str>,
{
    fn is_nice(&self) -> bool {
        let mut has_naughty_pair = false;

        self.as_ref()
            .chars()
            .filter(|c| VOWELS.iter().any(|v| c == v))
            .count()
            >= 3
            && self
                .as_ref()
                .chars()
                .fold((None, 0), |state, current_char| {
                    let (ref last_char, mut num_pairs) = state;
                    if let Some(l) = last_char {
                        if *l == current_char {
                            num_pairs += 1;
                        }
                        if NAUGHTY_PAIRS.contains(&(*l, current_char)) {
                            has_naughty_pair = true;
                        }
                    }

                    (Some(current_char), num_pairs)
                })
                .1
                > 0
            && !has_naughty_pair
    }
}

pub fn part_1(input: &str) -> u128 {
    input.lines().filter(Nice1::is_nice).count() as u128
}

trait Nice2 {
    fn is_nice(&self) -> bool;
}

fn has_two_pairs(s: &str) -> bool {
    for start in 0..(s.len() - 2) {
        let pair = &s[start..start + 2];
        for next in (start + 2)..(s.len() - 1) {
            if pair == &s[next..next + 2] {
                return true;
            }
        }
    }

    false
}

impl<T> Nice2 for T
where
    T: AsRef<str>,
{
    fn is_nice(&self) -> bool {
        has_two_pairs(self.as_ref())
            && self
                .as_ref()
                .chars()
                .fold((None, None, false), |state, current_char| {
                    let (prev_prev, prev, valid) = state;
                    if valid {
                        return state;
                    }

                    if let Some(prev_prev) = prev_prev {
                        if prev_prev == current_char {
                            return (None, None, true);
                        }

                        (prev, Some(current_char), false)
                    } else if let Some(_) = prev {
                        (prev, Some(current_char), false)
                    } else {
                        (None, Some(current_char), false)
                    }
                })
                .2
    }
}

pub fn part_2(input: &str) -> u128 {
    input.lines().filter(Nice2::is_nice).count() as u128
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::read_input;

    #[test]
    fn test_examples_part_1() {
        let mut input = "ugknbfddgicrmopn";
        assert!(Nice1::is_nice(&input));
        input = "aaa";
        assert!(Nice1::is_nice(&input));
        input = "jchzalrnumimnmhp";
        assert!(!Nice1::is_nice(&input));
        input = "haegwjzuvuyypxyu";
        assert!(!Nice1::is_nice(&input));
        input = "dvszwmarrgswjxmb";
        assert!(!Nice1::is_nice(&input));
    }

    #[test]
    fn test_examples_part_2() {
        let mut input = "qjhvhtzxzqqjkmpb";
        assert!(Nice2::is_nice(&input));
        input = "xxyxx";
        assert!(Nice2::is_nice(&input));
        input = "uurcxstgmygtbstg";
        assert!(!Nice2::is_nice(&input));
        input = "ieodomkazucvgmuy";
        assert!(!Nice2::is_nice(&input));
    }

    #[test]
    fn test_part_1() {
        let input = read_input("res/2015/day_5.txt").unwrap().trim().to_string();

        let result = part_1(&input);
        assert_eq!(result, 238);
    }

    #[test]
    fn test_part_2() {
        let input = read_input("res/2015/day_5.txt").unwrap().trim().to_string();

        let result = part_2(&input);
        assert_eq!(result, 69);
    }
}
