use anyhow::Result;
use aoc2024::{valid_do_dont, valid_mul_instr};
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/day3.input").unwrap();
    //let mut total = 0;
    let total = valid_do_dont(&input);
    println!("{}", total);
    Ok(())
}
