use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use std::path::Path;
//use regex::Regex;


// Struct to represent a number and its properties
#[derive(Debug)]
struct NumberInfo {
    x_start: usize,
    y_start: usize,
    x_end: usize,
    y_end: usize,
    number: i32,
    has_adjacent_symbol: bool,
}

// Function to process a line and find numbers
fn process_line(
    line: &str,
    _grid: &Vec<Vec<char>>,
    y: usize,
) -> Vec<NumberInfo> {
    let mut numbers_info = Vec::new();
    let mut current_number = String::new();
    let mut x_start = 0;

    for (x, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            current_number.push(c);
            if x == line.len() - 1 {
                // Reached the end of the line, add the number
                let number = current_number.parse::<i32>().unwrap();
                numbers_info.push(NumberInfo {
                    x_start,
                    y_start: y,
                    x_end: x,
                    y_end: y,
                    number,
                    has_adjacent_symbol: false,
                });
                current_number.clear();
            }
        } else {
            // Found a non-digit character
            if !current_number.is_empty() {
                // Add the previous number
                let number = current_number.parse::<i32>().unwrap();
                numbers_info.push(NumberInfo {
                    x_start,
                    y_start: y,
                    x_end: x - 1,
                    y_end: y,
                    number,
                    has_adjacent_symbol: false,
                });
                current_number.clear();
            }
            x_start = x + 1;
        }
    }

    numbers_info
}


fn main() -> io::Result<()> {
    // Retrieve the filename from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];

    // Open the specified file
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    // Read lines into a 2D grid
    let grid: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let mut total_sum = 0;
    let mut all_numbers_info: Vec<NumberInfo> = Vec::new();

    // First pass: Identify numbers and initialize properties
    for (y, line) in grid.iter().enumerate() {
        let line_numbers = process_line(&line.iter().collect::<String>(), &grid, y);
        all_numbers_info.extend(line_numbers);
    }

    // Second pass: Evaluate adjacency to symbols, including diagonals
    for info in &mut all_numbers_info {
        let x_start_adj = if info.x_start > 0 { info.x_start - 1 } else { 0 };
        let x_end_adj = if info.x_end < grid[info.y_start].len() - 1 {
            info.x_end + 1
        } else {
            grid[info.y_start].len() - 1
        };
        let y_start_adj = if info.y_start > 0 { info.y_start - 1 } else { 0 };
        let y_end_adj = if info.y_end < grid.len() - 1 {
            info.y_end + 1
        } else {
            grid.len() - 1
        };

        for y in y_start_adj..=y_end_adj {
            for x in x_start_adj..=x_end_adj {
                let symbol = grid[y][x];
                if symbol != '.' && !symbol.is_digit(10) {
                    info.has_adjacent_symbol = true;
                    break;
                }
            }
        }
    }
    // Calculate the sum of numbers with adjacent symbols
    for info in &all_numbers_info {
        if info.has_adjacent_symbol {
            total_sum += info.number;
        }
    }

    // Output the total sum
    println!("Total Sum: {}", total_sum);

    Ok(())
}