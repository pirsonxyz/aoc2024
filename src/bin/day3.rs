use anyhow::Result;
use aoc2024::{read_file_and_return_lines, valid_mul_instr};

fn main() -> Result<()> {
    let lines = read_file_and_return_lines("inputs/day3.input")?;
    let mut total = 0;
    for line in lines {
        let line = line?;
        //let sum = valid_mul_instr(&line);
        let sum = valid_mul_instr(&line.trim());
        total += sum;
    }
    println!("{}", total);
    Ok(())
}
