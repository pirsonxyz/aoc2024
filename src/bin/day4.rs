use anyhow::{Ok, Result};
use aoc2024::read_file_and_return_lines;

fn main() -> Result<()> {
    let lines = read_file_and_return_lines("inputs/day4.example")?;
    for line in lines {
        let line = line?;
        println!("{}", line.trim());
    }
    Ok(())
}
