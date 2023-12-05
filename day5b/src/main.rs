#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

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

    let seed_ranges = parse_seed_ranges(lines[0]);
    //println!("Seed ranges: {:?}", seed_ranges);
    let mut mappings = HashMap::new();
    process_mapping_blocks(&mut mappings, &lines[1..]);
    //println!("Mappings: {:?}", mappings);

    let lowest_location_seed = find_lowest_location_seed(&seed_ranges, &mappings);
    println!("Lowest location seed: {:?}", lowest_location_seed);
}

fn parse_seed_ranges(line: &str) -> Vec<(i64, i64)> {
    line.split_whitespace()
        .skip(1) // Skip the "seeds:" part
        .filter_map(|s| s.parse::<i64>().ok())
        .collect::<Vec<_>>()
        .chunks(2)
        .filter_map(|chunk| {
            if chunk.len() == 2 {
                Some((chunk[0], chunk[0] + chunk[1]))
            } else {
                None
            }
        })
        .collect()
}


fn process_mapping_blocks(mappings: &mut HashMap<String, Vec<(i64, i64, i64)>>, lines: &[&str]) {
    let mut current_map = String::new();

    for line in lines {
        if line.ends_with("map:") {
            current_map = line.split_whitespace().next().unwrap().to_owned();
            mappings.insert(current_map.clone(), vec![]);
        } else if !line.is_empty() {
            let parts: Vec<i64> = line.split_whitespace()
                .filter_map(|p| p.parse().ok())
                .collect();

            if parts.len() == 3 {
                mappings.get_mut(&current_map).unwrap().push((parts[0], parts[1], parts[2]));
                //println!("Adding mapping for {}: {:?}", current_map, parts);
            }
        }
    }
}

fn find_lowest_location_seed(seed_ranges: &[(i64, i64)], mappings: &HashMap<String, Vec<(i64, i64, i64)>>) -> Option<(i64, i64)> {
    let mut lowest_location_seed = None;
    let mut lowest_location_value = i64::MAX;

    for (start_seed, end_seed) in seed_ranges {
        for seed in *start_seed..*end_seed {
            let mut current_values = HashMap::new();
            //println!("Processing seed: {}", seed);
            current_values.insert("seed", seed);

            // Apply mappings in the specified order
            for mapping_type in &["seed-to-soil", "soil-to-fertilizer", "fertilizer-to-water", "water-to-light", "light-to-temperature", "temperature-to-humidity", "humidity-to-location"] {
                if let Some(mapping_rules) = mappings.get(*mapping_type) {
                    let map_from = mapping_type.split('-').next().unwrap();
                    let map_to = mapping_type.split('-').nth(2).unwrap().split('-').next().unwrap();

                    //println!("Applying mapping {} -> {}", map_from, map_to);

                    let current_value = *current_values.get(map_from).unwrap_or(&seed);
                    let mut new_value = current_value;

                    let mut mapping_applied = false;
                    for (destination_start, source_start, range_size) in mapping_rules {
                        if current_value >= *source_start && current_value < source_start + range_size {
                            let delta = destination_start - source_start;
                            new_value = current_value + delta;
                            mapping_applied = true;
                            //println!("Applied mapping: {} -> {} (delta: {})", current_value, new_value, delta);
                            break; // Move to the next mapping type after applying current one
                        }
                    }

                    // If mapping is out of range, new_value remains as current_value
                    current_values.insert(map_to, new_value);
                    //println!("Current values: {:?}", current_values);

                    if *mapping_type == "humidity-to-location" && new_value < lowest_location_value {
                        lowest_location_value = new_value;
                        lowest_location_seed = Some(seed);
                    }
                }
            }
        }
    }

    lowest_location_seed.map(|seed| (seed, lowest_location_value))
}

