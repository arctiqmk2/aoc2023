use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use std::path::Path;
use regex::Regex;

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

    let mut total_sum = 0;
    let re = Regex::new(r"(\d+) (red|green|blue)").unwrap();

    // Process each line in the file
    for line in reader.lines() {
        let line = line?;
        if line.starts_with("Game") {
            let parts: Vec<&str> = line.split(':').collect();
            let game_number: i32 = parts[0].replace("Game ", "").trim().parse().unwrap();
            let color_segments = parts[1].split(';');

            let mut is_possible_game = true;

            for segment in color_segments {
                for cap in re.captures_iter(segment) {
                    let value: i32 = cap[1].parse().unwrap();
                    match &cap[2] {
                        "red" if value > 12 => is_possible_game = false,
                        "green" if value > 13 => is_possible_game = false,
                        "blue" if value > 14 => is_possible_game = false,
                        _ => {}
                    }
                }
            }

            if is_possible_game {
                total_sum += game_number;
            }
        }
    }

    println!("Total sum of possible game numbers: {}", total_sum);

    Ok(())
}