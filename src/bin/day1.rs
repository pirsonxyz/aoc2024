use aoc2024::{count_ocurences, read_file_and_return_lines};

fn main() -> std::io::Result<()> {
    // Part1
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    let mut distance_total = 0;

    let lines = read_file_and_return_lines("inputs/day1.txt")?;
    for line in lines {
        if let Ok(line) = line {
            let (distance1, distance2) = line.split_once("  ").unwrap();
            let distance1: i32 = distance1.trim().parse().unwrap();
            let distance2: i32 = distance2.trim().parse().unwrap();
            list1.push(distance1);
            list2.push(distance2);
        }
    }

    list1.sort();
    list2.sort();

    for (distance1, distance2) in list1.iter().zip(list2.iter()) {
        distance_total += (distance2 - distance1).abs();
    }
    println!("{}", distance_total);

    // Part2
    let mut ocurrence_score: i32 = 0;
    for n in list1 {
        let num_of_ocurrences = count_ocurences(&list2, n);
        if num_of_ocurrences == 0 {
            continue;
        }
        let ocurrence_s = n * num_of_ocurrences as i32;
        ocurrence_score += ocurrence_s;
    }
    println!("{}", ocurrence_score);
    Ok(())
}
