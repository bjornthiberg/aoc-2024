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
        if safe(&nums) {
            num_safe += 1;
        } else {
            for i in 0..nums.len() {
                let nums_no_i = remove_element_by_index(&nums, i);
                if safe(&nums_no_i) {
                    num_safe += 1;
                    break;
                }
            }
        }
    }

    println!("{}", num_safe);
}

fn remove_element_by_index(vec: &[i32], index: usize) -> Vec<i32> {
    vec.iter()
        .enumerate()
        .filter(|(i, _)| *i != index)
        .map(|(_, x)| *x)
        .collect()
}

fn safe(v: &[i32]) -> bool {
    v.windows(2).all(|pair| {
        let diff = pair[1] - pair[0];
        diff.abs() >= 1 && diff.abs() <= 3 && diff.signum() == v[1].cmp(&v[0]) as i32
    })
}
