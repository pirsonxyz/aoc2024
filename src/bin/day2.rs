use anyhow::Result;
use aoc2024::{is_valid_report, is_valid_with_dampener, read_file_and_return_lines};

fn main() -> Result<()> {
    let mut count_of_safe = 0;
    let reports = read_file_and_return_lines("inputs/day2.input")?;
    for report in reports {
        if let Ok(report) = report {
            let numbers: Vec<i32> = report.split(' ').map(|s| s.parse().unwrap()).collect();
            /* part 1
            if is_valid_report(&numbers) {
                count_of_safe += 1
            }*/
            if is_valid_with_dampener(&numbers) {
                count_of_safe += 1;
            }
        }
    }
    println!("{}", count_of_safe);
    Ok(())
}
