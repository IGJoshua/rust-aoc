use std::fs::File;
use std::io::{BufRead, BufReader, Result};

pub fn part_1() -> Result<i32> {
    let f = File::open("res/2015/day_1.txt")?;

    let mut floor = 0;
    for line in BufReader::new(f).lines() {
        if let Ok(line) = line {
            for c in line.chars() {
                floor += match c {
                    '(' => 1,
                    ')' => -1,
                    _ => 0,
                }
            }
        } else {
            panic!("The reader failed");
        }
    }
    Ok(floor)
}

pub fn part_2() -> Result<i32> {
    let f = File::open("res/2015/day_1.txt")?;

    let mut floor = 0;
    let mut idx = 0;
    'outer: for line in BufReader::new(f).lines() {
        if let Ok(line) = line {
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
    }

    Ok(idx)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        if let Ok(result) = part_1() {
            assert_eq!(result, 232, "the result was correct floor");
        } else {
            assert!(false, "a file error occurred");
        }
    }

    #[test]
    fn test_part_2() {
        if let Ok(index) = part_2() {
            assert_eq!(index, 1783, "the result was correct instruction");
        } else {
            assert!(false, "a file error occurred");
        }
    }
}
