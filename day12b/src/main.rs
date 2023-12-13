#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use std::collections::HashMap;
use std::env;
use std::fs;
use std::cmp::{min, max};
//use std::collections::{HashSet, VecDeque};


fn score_sets (cached_sets: &mut HashMap<(usize, usize, usize), i64>, springs: &Vec<u8>, numbers: &Vec<i64>, position: usize, number_position: usize, current_length: usize) -> i64 {
    
    match cached_sets.get(&(position, number_position, current_length)) {
        Some(value) => {
            //println!("cached: {:?}, {:?}, {:?}, {:?}", position, number_position, current_length, value);
            return *value;
        },
        None => {
            //println!("not cached: {:?}, {:?}, {:?}", position, number_position, current_length);
        }
    }
    //println!("expand: {:?}, {:?}, {:?}, {:?}", springs, numbers, position, real_length);
    if position == springs.len() {
        if number_position == numbers.len() && current_length == 0 {
            return 1;
        } else if number_position == numbers.len()-1 && numbers[number_position] == current_length.try_into().unwrap() {
            return 1;
        } else {
            return 0;
        }
    }
    
    let mut score: i64 = 0;
    for c in [ b'.', b'#'] {
        if springs[position] == c || springs[position] == b'?' {
            if c == b'.' &&
               current_length == 0 {
                score += score_sets(cached_sets, &springs, &numbers, position + 1, number_position, 0);
            } else if c == b'.' &&
                      current_length > 0 &&
                      number_position < numbers.len() &&
                      numbers[number_position] == current_length.try_into().unwrap() {
                score += score_sets(cached_sets, &springs, &numbers, position + 1, number_position + 1, 0);
            } else if c == b'#' {
                score += score_sets(cached_sets, &springs, &numbers, position + 1, number_position, current_length + 1);
            }
        }
    }
    cached_sets.insert((position, number_position, current_length), score);
    return score;
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

    let mut total: i64 = 0;

    let number_of_rows : usize = lines.len();
    println!("number_of_rows: {:?}", number_of_rows);
    //println!("testing: {:?}", is_valid([b'#', b'.', b'#', b'.', b'#', b'#', b'#'].to_vec(), [1, 1, 3].to_vec()));
    lines.iter().for_each(|line| {
        println!("LINE: {:?}", line);
        let parts: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();
        let read_springs: Vec<u8> = parts[0].as_bytes().to_vec();
        let read_numbers: Vec<i64> = parts[1].split(',').filter_map(|s| s.trim().parse().ok()).collect();
        let mut springs: Vec<u8> = Vec::new();
        let mut numbers: Vec<i64> = Vec::new();
        for copies in 0..5 {
            if copies > 0 {
                springs.push(b'?');
            }
            springs.extend(read_springs.clone());
            numbers.extend(read_numbers.clone());
        }
        let mut operational_count: i64 = 0;
        let mut locked: Vec<u8> = Vec::new();
        let mut cached_sets: HashMap<(usize, usize, usize), i64> = HashMap::new();
        //println!("springs: {:?}, numbers: {:?}", springs, numbers);
        let localscore = score_sets(&mut cached_sets, &springs, &numbers, 0, 0, 0);
        //println!("localscore: {:?}", localscore);
        total += localscore;
    });
    println!("total: {:?}", total);
}