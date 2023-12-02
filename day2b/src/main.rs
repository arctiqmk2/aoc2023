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

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("Game") {
            let parts: Vec<&str> = line.split(':').collect();
            let color_segments = parts[1].split(';');

            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;

            for segment in color_segments {
                for cap in re.captures_iter(segment) {
                    let value: i32 = cap[1].parse().unwrap();
                    match &cap[2] {
                        "red" => max_red = max_red.max(value),
                        "green" => max_green = max_green.max(value),
                        "blue" => max_blue = max_blue.max(value),
                        _ => {}
                    }
                }
            }

            let product = max_red * max_green * max_blue;
            total_sum += product;
        }
    }

    println!("Total sum of product of max values: {}", total_sum);
    
    Ok(())
}