use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    let file: fs::File = fs::File::open("inputs/day1.txt").expect("Could not open file");
    let reader: BufReader<fs::File> = BufReader::new(file);

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line: String = line.unwrap();
        let mut nums = line
            .split_whitespace()
            .map(|n: &str| n.parse::<i32>().unwrap());
        left.push(nums.next().unwrap());
        right.push(nums.next().unwrap());
    }

    let mut similarity = 0;
    for i in 0..left.len() {
        let l = left[i];
        let count = right.iter().filter(|&n| *n == l).count() as i32;
        similarity += l * count;
    }

    println!("Similartiy score: {}", similarity);
}
