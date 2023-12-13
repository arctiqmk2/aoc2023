#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use std::collections::HashMap;
use std::env;
use std::fs;
use std::cmp::{min, max};
use std::collections::{HashSet, VecDeque};

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum SymmetryType {
    Row,
    Column,
}

fn is_symmetric(
    grid: &[Vec<u8>],
    index1: usize,
    index2: usize,
    symmetry_type: SymmetryType,
    cache: &mut HashMap<(SymmetryType, usize, usize), bool>,
) -> bool {
    let key = (symmetry_type, index1, index2);
    let rows = grid.len();
    let columns = grid[0].len();

    println!("key: {:?}", key);
    if let Some(&cached) = cache.get(&key) {
        return cached;
    }
    let result = match symmetry_type {
        SymmetryType::Row => {
            grid[index1] == grid[index2]
        },
        SymmetryType::Column => {
            (0..grid.len()).all(|row| grid[row][index1] == grid[row][index2])
        },
    };
    cache.insert(key, result);
    result
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);



fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let mut contents = fs::read_to_string(filename).expect("Failed to read file");
    
    let normalized_contents = contents.replace("\r\n", "\n"); // Normalize line endings
    let line_groups = normalized_contents.split("\n\n");

    let mut global_score: i64 = 0;

    //let mut cache = HashMap::new();
 

    for group in line_groups {
        println!("{}", group);

    
        let grid: Vec<Vec<u8>> = group
            .lines()
            .map(|l| l.as_bytes().to_vec())
            .collect();

        let rows: usize = grid.len();
        let columns: usize = grid[0].len();

        println!("columns: {}, rows: {}", columns, rows);


        for column in 0..columns - 1 {
            let mut symmetry = 0;
            for checkcolumn in 0..columns {
                let left = column as i32 - checkcolumn as i32;
                let right = column as i32 + 1 + checkcolumn as i32;
                if left >= 0 && left < right && (right as usize) < columns {
                    println!("left: {}, right: {}", left, right);
                    for row in 0..rows {
                        if grid[row][left as usize] != grid[row][right as usize] {
                            symmetry += 1;
                        }
                    }
                }
            }
            if symmetry ==  0 {
                global_score += columns as i64 + 1;
            }
        }


        for row in 0..rows - 1  {
            let mut symmetry = 0;
            for checkrow in 0..rows {
                let up = row as i32 - checkrow as i32;
                let down = row as i32 + 1 + checkrow as i32;
                if up >= 0 && up < down && (down as usize) < rows {
                    for column in 0..columns {
                        if grid[up as usize][column] != grid[down as usize][column] {
                            symmetry += 1;
                        }
                    }
                }
            }
            if symmetry ==  0 {
                global_score += 100 * (row as i64 + 1);
            }
        }

    }
    println!("global_score: {:?}", global_score);
}