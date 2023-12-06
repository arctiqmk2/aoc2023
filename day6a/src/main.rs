#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

//use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug)]
struct RaceData {
    time: i32,
    distance: i32,
    wins: i32,
}

fn count_winning_scenarios(time: i32, distance: i32) -> i32 {
    let mut count = 0;

    // Calculate the continuous optimal charging time (as a floating-point number)
    let optimal_charge_time = time as f64 / 2.0;

    // Check for winning scenarios around the optimal charging time
    let mut charge_time = optimal_charge_time.floor() as i32;
    while charge_time <= optimal_charge_time.ceil() as i32 {
        if charge_time * (time - charge_time) >= distance {
            count += 1;
        }
        charge_time += 1;
    }

    // Expand the search around the optimal charging time
    let mut lower = (optimal_charge_time.floor() - 1.0) as i32;
    let mut upper = (optimal_charge_time.ceil() + 1.0) as i32;

    while lower >= 0 {
        if lower * (time - lower) > distance {
            count += 1;
        } else {
            break;
        }
        lower -= 1;
    }

    while upper <= time {
        if upper * (time - upper) > distance {
            count += 1;
        } else {
            break;
        }
        upper += 1;
    }

    count
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("Failed to read file");
    let lines: Vec<&str> = content.lines().collect();
    let time_line = lines[0];
    let distance_line = lines[1];

    let times: Vec<i32> = time_line.split_whitespace()
                                   .skip(1) // Skip the "Time:" label
                                   .map(|s| s.parse().unwrap())
                                   .collect();

    let distances: Vec<i32> = distance_line.split_whitespace()
                                           .skip(1) // Skip the "Distance:" label
                                           .map(|s| s.parse().unwrap())
                                           .collect();
                                    
    let mut races: Vec<RaceData> = times.into_iter().zip(distances.into_iter())
        .map(|(time, distance) | RaceData { time, distance, wins: count_winning_scenarios(time, distance) })
        .collect();
                                                         
    let wins_product: i32 = races.iter().map(|race| race.wins).fold(1, |acc, x| acc * x);

    println!("Races Won Product: {:#?}", wins_product);
    
}





