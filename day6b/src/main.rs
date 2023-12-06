#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_imports)]

//use std::collections::HashMap;
use std::env;
use std::fs;
use num_bigint::BigInt;
use num_traits::ToPrimitive;
use bigdecimal::BigDecimal;
use num_traits::{Zero, One};

#[derive(Debug)]
struct RaceData {
    time: i64,
    distance: i64,
    wins: i64,
}

fn count_winning_scenarios(time: i64, distance: i64) -> i64 {
    let time_big = BigDecimal::from(time);
    let distance_big = BigDecimal::from(distance);
    let four = BigDecimal::from(4);

    let discriminant = &time_big * &time_big - &four * &distance_big;

    if discriminant < Zero::zero() {
        return 0; // No real roots, thus no winning scenarios
    }

    let sqrt_discriminant = discriminant.sqrt().unwrap_or_else(|| BigDecimal::from(0));
    let two = BigDecimal::from(2);

    // Calculate roots
    let root1 = ((&time_big - &sqrt_discriminant) / &two).to_i64().unwrap_or(0);
    let root2 = ((&time_big + &sqrt_discriminant) / &two).to_i64().unwrap_or(0);
    let result: i64 = root2 - root1;
    result
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
    let time_line: String = lines[0].chars().filter(|c| !c.is_whitespace()).collect();
    let distance_line: String = lines[1].chars().filter(|c| !c.is_whitespace()).collect();

    let times: Vec<i64> = time_line.split(":")
                                   .skip(1) // Skip the "Time:" label
                                   .map(|s| s.parse().unwrap())
                                   .collect();

    let distances: Vec<i64> = distance_line.split(":")
                                           .skip(1) // Skip the "Distance:" label
                                           .map(|s| s.parse().unwrap())
                                           .collect();
                                                                               
    let mut races: Vec<RaceData> = times.into_iter().zip(distances.into_iter())
        .map(|(time, distance) | RaceData { time, distance, wins: count_winning_scenarios(time, distance) })
        .collect();
                                                         
    //let wins_product: i32 = races.iter().map(|race| race.wins).fold(1, |acc, x| acc * x);

    println!("Races {:#?}", races);
    //println!("Races Won Product: {:#?}", wins_product);
    
}





