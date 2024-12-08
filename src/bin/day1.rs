use std::collections::HashMap;

use aoc2024::{count_occurrences, read_file_and_return_lines};

fn main() -> std::io::Result<()> {
    // Part1
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    let mut distance_total = 0;

    let lines = read_file_and_return_lines("inputs/day1.input")?;
    for line in lines {
        if let Ok(line) = line {
            let (distance1, distance2) = line.split_once("  ").unwrap();
            let distance1: i32 = distance1.trim().parse().unwrap();
            let distance2: i32 = distance2.trim().parse().unwrap();
            left.push(distance1);
            right.push(distance2);
        }
    }

    left.sort();
    right.sort();

    for (distance1, distance2) in left.iter().zip(right.iter()) {
        distance_total += (distance2 - distance1).abs();
    }
    println!("{}", distance_total);

    // Part2
    let mut occurrence_score: i32 = 0;
    let mut map_of_ocurrences: HashMap<i32, i32> = HashMap::new();
    for &n in &right {
        *map_of_ocurrences.entry(n).or_insert(0) += 1;
    }
    for n in left {
        occurrence_score += n * map_of_ocurrences.get(&n).unwrap_or(&0);
    }
    println!("{}", occurrence_score);
    Ok(())
}
