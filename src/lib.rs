use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

pub fn read_file_and_return_lines(file_name: &str) -> std::io::Result<Lines<BufReader<File>>> {
    let file = File::open(file_name)?;
    Ok(BufReader::new(file).lines())
}
pub fn count_ocurences<T>(vec: &Vec<T>, num: T) -> usize
where
    T: std::cmp::PartialEq,
{
    vec.iter().filter(|&n| *n == num).count()
}
