use regex::Regex;
use std::fs;
use std::io::prelude::*;

fn main() {
    let mut file = fs::File::open("inputs/day3.txt").expect("Could not open file");
    let mut buffer = String::new();
    let _ = file.read_to_string(&mut buffer);

    let re_file = Regex::new(r"(mul\(\d+,\d+\))").unwrap();
    let re_word = Regex::new(r"(\d)+").unwrap();

    let mut result = 0;
    for m in re_file.find_iter(&buffer) {
        let digits: Vec<i32> = re_word
            .find_iter(&buffer[m.start()..m.end()])
            .filter_map(|digits| digits.as_str().parse().ok())
            .collect();

        result += digits[0] * digits[1];
    }

    println!("Result: {}", result);
}
