#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_imports)]

//use std::collections::HashMap;
use std::env;
use std::fs;
use std::cmp::{min, max};

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("Failed to read file");
    let lines: Vec<&str> = content.lines().collect();

    let mut reports: Vec<Vec<i64>> = Vec::new();
    let mut history_sum: i64 = 0;
    
    for line in lines.iter() {
        let numbers: Vec<i64> = line.split_whitespace().collect::<Vec<&str>>().iter().map(|x| x.parse::<i64>().unwrap()).collect();
        let mut stack: Vec<Vec<i64>> = Vec::new();
        let mut current_sequence = numbers.clone();
        let mut current_sequence_length = current_sequence.len();
        let mut current_sum: i64 = current_sequence.iter().sum::<i64>();
        let mut current_min: i64 = *current_sequence.iter().min().unwrap();
        let mut current_max: i64 = *current_sequence.iter().max().unwrap();
        let mut current_depth = 0;
        stack.push(numbers.clone());
        current_depth += 1;
        //println!("depth: {:?}, current_sum: {:?}, current_min: {:?}, current_max: {:?}", current_depth, current_sum, current_min, current_max);
        while !(current_sum == 0 && current_min == 0 && current_max == 0) && current_sequence_length > 1 {
            let mut current_diff = 0;
            let mut differences: Vec<i64> = Vec::new();
            for i in 0..current_sequence_length-1 {
                current_diff = current_sequence[i+1] - current_sequence[i];
                differences.push(current_diff);
            }
            current_sequence_length -= 1;
            stack.push(differences.clone());
            current_depth += 1;
            current_sequence = differences;
            current_sum = current_sequence.iter().sum::<i64>();
            current_min = *current_sequence.iter().min().unwrap();
            current_max = *current_sequence.iter().max().unwrap();
            //println!("depth: {:?}, current_sum: {:?}, current_min: {:?}, current_max: {:?}", current_depth, current_sum, current_min, current_max);
        }
        let mut current_addition: i64 = 0;
        while current_depth > 0 {
            current_depth -= 1;
            let delta = max(current_addition, stack[current_depth][0]) - min(current_addition, stack[current_depth][0]);
            if stack[current_depth][0] > current_addition {
                current_addition = delta;
            } else {
                current_addition = -delta;
            }
            stack[current_depth].splice(0..0,[current_addition]);
        }
        //println!("stack: {:?}", stack);
        reports.push(numbers);
        history_sum += current_addition;
    }
    //println!("reports: {:?}", reports);
    println!("history_sum: {:?}", history_sum);
}





