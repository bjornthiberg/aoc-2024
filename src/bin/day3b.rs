use regex::Regex;
use std::fs;
use std::io::prelude::*;

fn main() {
    let mut file = fs::File::open("inputs/day3.txt").expect("Could not open file");
    let mut buffer = String::new();
    let _ = file.read_to_string(&mut buffer);

    let re_dont = Regex::new(r"(don't\(\))").unwrap();
    let re_do = Regex::new(r"(do\(\))").unwrap();

    let mut result = 0;
    let mut matching = true;
    let mut start = 0;
    let mut slice: &str;
    let mut haystack;
    loop {
        haystack = &buffer[start..];
        if matching {
            if let Some(m) = re_dont.find(haystack) {
                slice = &buffer[start..(start + m.end())];
                result += get_result(slice);
                start += m.end();
                matching = false;
            } else {
                slice = &buffer[start..];
                result += get_result(slice);
                break;
            }
        } else if let Some(m) = re_do.find(haystack) {
            start += m.end();
            matching = true;
        } else {
            break;
        }
    }

    println!("Result: {}", { result });
}

fn get_result(slice: &str) -> i32 {
    let re_file = Regex::new(r"(mul\(\d+,\d+\))").unwrap();
    let re_word = Regex::new(r"(\d)+").unwrap();

    let mut result = 0;
    for m in re_file.find_iter(slice) {
        let digits: Vec<i32> = re_word
            .find_iter(m.into())
            .filter_map(|digits| digits.as_str().parse().ok())
            .collect();

        result += digits[0] * digits[1];
    }

    result
}
