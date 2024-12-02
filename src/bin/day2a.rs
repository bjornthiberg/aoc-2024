use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    let file = fs::File::open("inputs/day2.txt").expect("Could not open file");
    let reader = BufReader::new(file);

    let mut num_safe = 0;

    for line in reader.lines() {
        let line: String = line.unwrap();
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|n: &str| n.parse::<i32>().unwrap())
            .collect();
        if safe(nums) {
            num_safe += 1;
        }
    }

    println!("{}", num_safe);
}

fn safe(v: Vec<i32>) -> bool {
    let increasing = v[0] < v[1];
    for i in 1..(v.len()) {
        if ((v[i] > v[i - 1]) & !increasing)
            || ((v[i] < v[i - 1]) & increasing)
            || v[i] == v[i - 1]
            || (v[i] - v[i - 1]).abs() > 3
        {
            return false;
        }
    }
    true
}
