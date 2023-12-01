use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use std::path::Path;

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

    // Process each line in the file
    for line in reader.lines() {
        if let Ok(ip) = line {
            let digits: Vec<char> = ip.chars().filter(|c| c.is_digit(10)).collect();
            if !digits.is_empty() {
                let first_digit = digits.first().unwrap_or(&'0');
                let last_digit = digits.last().unwrap_or(first_digit);
                println!("{}{}", first_digit, last_digit);
            } else {
                println!("No digits found in this line.");
            }
        }
    }
    Ok(())
}
