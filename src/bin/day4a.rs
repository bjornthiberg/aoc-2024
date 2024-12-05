use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    let file = fs::File::open("inputs/day4.txt").expect("Could not open file");
    let reader = BufReader::new(file);

    let mut result = 0;
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let row: Vec<char> = line.unwrap().chars().collect();
        grid.push(row);
    }

    let size = grid.len();

    result += get_xmas_number(&grid[0]);
    result += get_xmas_number(&get_diagonal(&grid, 0, 0, 1, 0));
    result += get_xmas_number(&get_diagonal(&grid, 0, 0, 1, 1));
    result += get_xmas_number(&get_diagonal(&grid, 0, grid.len() - 1, 1, -1));

    for i in 1..size {
        // horizontal
        result += get_xmas_number(&grid[i]);
        // vertical
        result += get_xmas_number(&get_diagonal(&grid, 0, i, 1, 0));
        // diagonal, left to right, growing downwards
        result += get_xmas_number(&get_diagonal(&grid, i, 0, 1, 1));
        // diagonal, right to left, growing downwards
        result += get_xmas_number(&get_diagonal(&grid, i, grid.len() - 1, 1, -1));
        // diagonal, left to right, growing sidewards
        result += get_xmas_number(&get_diagonal(&grid, 0, i, 1, 1));
        // diagonal, right to left, growing sidewards
        result += get_xmas_number(&get_diagonal(&grid, 0, grid.len() - 1 - i, 1, -1));
    }

    println!("Result: {}", result);
}

fn get_xmas_number(v: &[char]) -> usize {
    v.windows(4)
        .filter(|window| window == &['X', 'M', 'A', 'S'] || window == &['S', 'A', 'M', 'X'])
        .count()
}

fn get_diagonal(
    grid: &[Vec<char>],
    start_row: usize,
    start_col: usize,
    row_inc: i32,
    col_inc: i32,
) -> Vec<char> {
    let mut result = Vec::new();
    let mut row = start_row as i32;
    let mut col = start_col as i32;

    while row >= 0 && row < grid.len() as i32 && col >= 0 && col < grid.len() as i32 {
        result.push(grid[row as usize][col as usize]);
        row += row_inc;
        col += col_inc;
    }

    result
}
