#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_imports)]

//use std::collections::HashMap;
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

    let mut reports: Vec<Vec<i64>> = Vec::new();
    
    for line in lines.iter() {
        let numbers: Vec<i64> = line.split_whitespace().collect::<Vec<&str>>().iter().map(|x| x.parse::<i64>().unwrap()).collect();
        reports.push(numbers);
    }
    println!("reports: {:?}", reports);
}





