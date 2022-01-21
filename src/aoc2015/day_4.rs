extern crate crossbeam;
extern crate md5;
extern crate num_cpus;

use std::fmt::Write;
use std::ops::Range;
use std::sync::atomic::{AtomicI64, Ordering};

fn search_range(input: &str, search_text: &str, range: Range<i64>) -> Option<i64> {
    let mut num_str = String::with_capacity(input.len() + 32);
    let mut hash = String::with_capacity(64);

    for n in range {
        // Ensure we're starting with empty strings when we write in the input
        num_str.clear();
        hash.clear();

        write!(&mut num_str, "{}{}", input, n).unwrap();
        let digest = md5::compute(&num_str);
        write!(&mut hash, "{:x}", digest).unwrap();

        if hash.starts_with(search_text) {
            return Some(n);
        }
    }

    None
}

fn search_for<'a>(input: &'a str, search_text: &'a str) -> i64 {
    let next_index = AtomicI64::new(0i64);
    let result = AtomicI64::new(-1i64);

    crossbeam::scope(|scope| {
        for _ in 0..num_cpus::get() {
            scope.spawn(|_| {
                loop {
                    let start_idx = next_index.fetch_add(10000, Ordering::AcqRel);

                    if start_idx < 0 {
                        return;
                    }

                    if let Some(res) =
                        search_range(input, search_text, start_idx..start_idx + 10000)
                    {
                        next_index.store(i64::MIN, Ordering::Release);

                        // Store the result, but only if we haven't stored
                        // before. Either way return.
                        match result.compare_exchange(-1, res, Ordering::AcqRel, Ordering::Acquire)
                        {
                            _ => return,
                        }
                    }
                }
            });
        }
    })
    .unwrap();

    result.load(Ordering::Acquire)
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
