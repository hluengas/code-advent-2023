use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // open input file
    let file = File::open("./input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    // map the lines, filtering out non-numeric characters, then keeping the first and last digit for the final sum
    let numbers: Vec<i32> = reader
        .lines()
        .map(|line| line.expect("line read failure"))
        .filter_map(|line| {
            let filtered_line: String = line.chars().filter(|c| c.is_numeric()).collect();
            let truncated_line = format!(
                "{}{}",
                filtered_line
                    .chars()
                    .nth(0)
                    .expect("parse failure")
                    .to_string(),
                filtered_line
                    .chars()
                    .last()
                    .expect("parse failure")
                    .to_string()
            );
            truncated_line.parse::<i32>().ok()
        })
        .collect();

    let sum: i32 = numbers.iter().sum();
    println!("Sum: {}", sum);
}
