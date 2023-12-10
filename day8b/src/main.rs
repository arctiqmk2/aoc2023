#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_imports)]

//use std::collections::HashMap;
use std::env;
use std::fs;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let mut nodes = Vec::new();
    let mut lefts = HashMap::new();
    let mut rights = HashMap::new();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("Failed to read file");
    let lines: Vec<&str> = content.lines().collect();

    let instructions: Vec<_> = lines[0].chars().collect();
    
    println!("instructions: {:?}", instructions);

    let re = Regex::new(r"(\d{2}[A-Z]|[A-Z]{3}) = \((\d{2}[A-Z]|[A-Z]{3}), (\d{2}[A-Z]|[A-Z]{3})\)").unwrap();

    for line in lines[2..].iter() {
        for cap in re.captures_iter(line) {
            let node: String = cap[1].parse().unwrap();
            let left: String = cap[2].parse().unwrap();
            let right: String = cap[3].parse().unwrap();
            nodes.push(node.clone());
            lefts.insert(node.clone(), left.clone());
            rights.insert(node.clone(), right.clone());
            println!("node: {} {} {}", node, left, right);
        }
    }
    //println!("nodes: {:?}", nodes);
    //println!("lefts: {:?}", lefts);
    //println!("rights: {:?}", rights);
    let mut current_nodes: Vec<String> = Vec::new();
    current_nodes = nodes.iter()
                         .filter(|&s| s.ends_with('A'))
                         .cloned()
                         .collect();
    println!("start - current_nodes: {:?}", current_nodes);
    let mut step = 0;
    let mut found = 0;

    while found == 0 {
        for instruction in instructions.iter() {
            match instruction {
                'L' => {
                    step += 1;
                    current_nodes = current_nodes.iter()
                                                 .map(|s| lefts.get(s).unwrap().clone())
                                                 .collect();
                },
                'R' => {
                    step += 1;
                    current_nodes = current_nodes.iter()
                                                 .map(|s| rights.get(s).unwrap().clone())
                                                 .collect();
                },
                _ => {
                    println!("Unknown instruction: {}", instruction);
                    break;
                }
            }
            // need to put check for all ending in Z here.
            let check_nodes = current_nodes.iter()
                                           .filter(|&s| !s.ends_with('Z'))
                                           .cloned()
                                           .collect::<Vec<String>>();
            if check_nodes.len() == 0 {
                found = 1;
                println!("Found all Zs in {} steps", step);
                break;

            }
        }
    }              
}





