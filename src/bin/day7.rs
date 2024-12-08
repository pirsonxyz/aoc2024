use anyhow::Result;
use aoc2024::read_file_and_return_lines;

fn main() -> Result<()> {
    let lines = read_file_and_return_lines("inputs/day7.example")?;
    for line in lines {
        let line = line?;
        let (test_value, numbers) = line.split_once(':').unwrap();
        for num in numbers.split(' ') {
            println!("{}", num);
        }
    }
    Ok(())
}
