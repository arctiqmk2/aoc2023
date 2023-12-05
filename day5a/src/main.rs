use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("Failed to read file");
    let lines: Vec<&str> = content.lines().collect();

    let mut seedbank: Vec<HashMap<String, i64>> = Vec::new();
    
    initialize_seedbank(&mut seedbank, &lines);
    println!("-----");
    println!("{:?}", seedbank);
    println!("-----");

    process_data_blocks(&mut seedbank, &lines);


    println!("-----");
    println!("{:?}", seedbank);
    println!("-----");


    let slice_with_lowest_location = find_lowest_location_slice(&seedbank);
    println!("{:?}", slice_with_lowest_location);
}

fn initialize_seedbank(seedbank: &mut Vec<HashMap<String, i64>>, lines: &[&str]) {
    let seeds: Vec<i64> = lines[0].split_whitespace()
        .skip(1) // skip the "seeds:" part
        .filter_map(|s| s.parse().ok())
        .collect();

    for seed in seeds {
        let mut map = HashMap::new();
        map.insert("seed".to_string(), seed);
        map.insert("soil".to_string(), -1);
        map.insert("fertilizer".to_string(), -1);
        map.insert("water".to_string(), -1);
        map.insert("light".to_string(), -1);
        map.insert("temperature".to_string(), -1);
        map.insert("humidity".to_string(), -1);
        map.insert("location".to_string(), -1);
        seedbank.push(map);
    }
}

fn process_data_blocks(seedbank: &mut Vec<HashMap<String, i64>>, lines: &[&str]) {
    let mut current_block_lines: Vec<&str> = vec![];
    for line in lines.iter().skip(1) { // skipping the seeds line
        if line.ends_with("map:") {
            if !current_block_lines.is_empty() {
                println!("Processing block: {:?}", current_block_lines);
                update_seedbank(seedbank, &current_block_lines);
            }
            current_block_lines.clear();
            current_block_lines.push(line);
            continue;
        }

        if !line.is_empty() {
            current_block_lines.push(line);
        }
    }

    if !current_block_lines.is_empty() {
        println!("Processing block: {:?}", current_block_lines);
        update_seedbank(seedbank, &current_block_lines);
    }
}

fn update_seedbank(seedbank: &mut Vec<HashMap<String, i64>>, block_lines: &[&str]) {
    let block_header = block_lines[0];
    let (map_from, map_to) = extract_map_from_to(block_header);

    for &line in &block_lines[1..] {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            continue; // skip invalid lines
        }

        let destination_start: i64 = parts[0].parse().unwrap();
        let source_start: i64 = parts[1].parse().unwrap();
        let range_size: i64 = parts[2].parse().unwrap();
        let source_end = source_start + range_size - 1;
        let delta = destination_start - source_start;
        println!("destination_start: {}, source_start: {}, range_size: {}, source_end: {}, delta: {}", destination_start, source_start, range_size, source_end, delta);

        for map in seedbank.iter_mut() {
            let seed = map.get("seed").copied().unwrap_or_default();
            let value = map.get(map_from).copied().unwrap_or_default();
            println!("seed: {}, map_from: {}, value: {}", seed, map_from, value);
            if value >= source_start && value <= source_end {
                *map.entry(map_to.to_string()).or_insert(0) = value+delta;
                println!("IR: map_to: {}, value: {}", map_to, value+delta);
            } else if map.get(map_to).copied().unwrap_or_default() == -1 {
                *map.entry(map_to.to_string()).or_insert(0) = value;
                println!("OR: map_to: {}, value: {}", map_to, value);
            } else {
                println!("OR: skipping, already mapped.");
            }
        }
    }
}

fn extract_map_from_to(block_header: &str) -> (&str, &str) {
    let parts: Vec<&str> = block_header.split('-').collect();
    let map_from = parts[0].trim();
    let map_to_with_colon = parts[2].trim();
    let map_to = map_to_with_colon.trim_end_matches(" map:");
    println!("map_from: {}, map_to: {}", map_from, map_to);
    (map_from, map_to)
}

fn find_lowest_location_slice(seedbank: &[HashMap<String, i64>]) -> Option<&HashMap<String, i64>> {
    seedbank.iter().min_by_key(|map| map.get("location").copied().unwrap_or_default())
}
