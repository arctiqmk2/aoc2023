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

    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

    for line in lines[2..].iter() {
        for cap in re.captures_iter(line) {
            let node: String = cap[1].parse().unwrap();
            let left: String = cap[2].parse().unwrap();
            let right: String = cap[3].parse().unwrap();
            nodes.push(node.clone());
            lefts.insert(node.clone(), left.clone());
            rights.insert(node.clone(), right.clone());
            //println!("node: {} {} {}", node, left, right);
        }
    }
    //println!("nodes: {:?}", nodes);
    //println!("lefts: {:?}", lefts);
    //println!("rights: {:?}", rights);
    let mut current_node = "AAA";
    let mut step = 0;
    let mut found = 0;

    while found == 0 {
        for instruction in instructions.iter() {
            match instruction {
                'L' => {
                    step += 1;
                    current_node = lefts.get(current_node).unwrap();
                    if current_node == "ZZZ" {
                        println!("Found ZZZ in {} steps", step);
                        found += 1;
                        break;
                    }
                },
                'R' => {
                    step += 1;
                    current_node = rights.get(current_node).unwrap();
                    if current_node == "ZZZ" {
                        println!("Found ZZZ in {} steps", step);
                        found += 1;
                        break;
                    }
                },
                _ => {
                    println!("Unknown instruction: {}", instruction);
                    break;
                }
            }
        }
    }
    for instruction in instructions.iter() {
        match instruction {
            'L' => {
                step += 1;
                current_node = lefts.get(current_node).unwrap();
                if current_node == "ZZZ" {
                    println!("Found ZZZ in {} steps", step);
                    found += 1;
                    break;
                }
            },
            'R' => {
                step += 1;
                current_node = rights.get(current_node).unwrap();
                if current_node == "ZZZ" {
                    println!("Found ZZZ in {} steps", step);
                    found += 1;
                    break;
                }
            },
            _ => {
                println!("Unknown instruction: {}", instruction);
                break;
            }
        }
    }
}





