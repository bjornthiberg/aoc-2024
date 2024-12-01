use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    let file = fs::File::open("inputs/day1.txt").expect("Could not open file");
    let reader = BufReader::new(file);

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line: String = line.unwrap();
        let mut nums = line.split_whitespace().map(|n| n.parse::<i32>().unwrap());
        left.push(nums.next().unwrap());
        right.push(nums.next().unwrap());
    }

    left.sort();
    right.sort();

    let mut dist = 0;
    for i in 0..left.len() {
        dist += (left[i] - right[i]).abs();
    }

    println!("Total distance: {}", dist);
}
