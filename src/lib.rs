use std::{fs::File, io::BufReader};

pub fn read_file_and_return_buf_reader(file_name: &str) -> std::io::Result<BufReader<File>> {
    let file = File::open(file_name)?;
    Ok(BufReader::new(file))
}
pub fn count_ocurences(vec: &Vec<i32>, num: i32) -> usize {
    vec.iter().filter(|&n| *n == num).count()
}
