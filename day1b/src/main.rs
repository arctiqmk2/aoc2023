use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use std::path::Path;
use std::collections::HashMap;


fn word_to_digit_map() -> HashMap<String, char> {
    [
        ("zero", '0'), ("one", '1'), ("two", '2'), ("three", '3'), 
        ("four", '4'), ("five", '5'), ("six", '6'), ("seven", '7'), 
        ("eight", '8'), ("nine", '9'),
    ].iter().map(|&(word, digit)| (word.to_string(), digit)).collect()
}

fn find_first_digit(line: &str, map: &HashMap<String, char>) -> Option<char> {
    for i in 0..line.len() {
        for (word, &digit) in map {
            if line[i..].starts_with(word) {
                return Some(digit);
            }
        }
        if line[i..].chars().next().unwrap().is_digit(10) {
            return Some(line[i..].chars().next().unwrap());
        }
    }
    None
}

fn find_last_digit(line: &str, map: &HashMap<String, char>) -> Option<char> {
    for i in (0..line.len()).rev() {
        for (word, &digit) in map {
            if line[..=i].ends_with(word) {
                return Some(digit);
            }
        }
        if line[..=i].chars().rev().next().unwrap().is_digit(10) {
            return Some(line[..=i].chars().rev().next().unwrap());
        }
    }
    None
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

    let digit_map = word_to_digit_map();
    let mut sum = 0;

    // Process each line in the file
    for line in reader.lines() {
        if let Ok(ip) = line {
            let first_digit = find_first_digit(&ip, &digit_map);
            let last_digit = find_last_digit(&ip, &digit_map);

            match (first_digit, last_digit) {
                (Some(first), Some(last)) => {
                    let number = format!("{}{}", first, last).parse::<i32>().unwrap_or(0);
                    println!("{} {}", ip, number);
                    sum += number;
                },
                _ => println!("No valid digits found in this line."),
            }
        }
    }

    println!("Sum of all numbers: {}", sum);
    
    Ok(())
}
