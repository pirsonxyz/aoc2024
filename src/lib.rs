use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

use regex::Regex;

pub fn read_file_and_return_lines(file_name: &str) -> std::io::Result<Lines<BufReader<File>>> {
    let file = File::open(file_name)?;
    Ok(BufReader::new(file).lines())
}
pub fn count_occurrences<T>(vec: &Vec<T>, num: T) -> usize
where
    T: std::cmp::PartialEq,
{
    vec.iter().filter(|&n| *n == num).count()
}
pub fn is_valid_report(numbers: &[i32]) -> bool {
    if numbers.len() < 2 {
        return false;
    }
    let first_diff = numbers[1] - numbers[0];
    if first_diff == 0 {
        return false;
    }
    let is_increasing = first_diff > 0;
    numbers.windows(2).all(|w| {
        let diff = w[1] - w[0];
        if is_increasing {
            diff >= 1 && diff <= 3
        } else {
            diff <= -1 && diff >= -3
        }
    })
}
pub fn is_valid_with_dampener(numbers: &[i32]) -> bool {
    if is_valid_report(numbers) {
        return true;
    }
    for i in 0..numbers.len() {
        let mut nums_vec = numbers.to_vec();
        nums_vec.remove(i);
        if is_valid_report(&nums_vec) {
            return true;
        }
    }
    false
}
pub fn valid_mul_instr(instr: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(instr)
        .map(|_match| &_match[1].parse::<i32>().unwrap() * &_match[2].parse::<i32>().unwrap())
        .sum()
}
