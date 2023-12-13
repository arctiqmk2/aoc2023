#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_imports)]

//use std::collections::HashMap;
use std::env;
use std::fs;
use std::cmp::{min, max};
use std::collections::{HashSet, VecDeque};

#[derive(Clone, Copy, Debug)]
enum Dir {
    North,
    East,
    West,
    South,
}
impl Dir {
    fn invert(&self) -> Dir {
        match self {
            Dir::East => Dir::West,
            Dir::West => Dir::East,
            Dir::North => Dir::South,
            Dir::South => Dir::North,
        }
    }
    fn all() -> Vec<Dir> {
        vec![Dir::North, Dir::East, Dir::West, Dir::South]
    }
}
fn opens(b: u8, d: Dir) -> bool {
    match (b, d) {
        (b'S', _) => true,
        (b'|', Dir::South | Dir::North) => true,
        (b'-', Dir::East | Dir::West) => true,
        (b'F', Dir::East | Dir::South) => true,
        (b'L', Dir::North | Dir::East) => true,
        (b'J', Dir::West | Dir::North) => true,
        (b'7', Dir::West | Dir::South) => true,
        _ => false,
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn is_valid(&self, yln: usize, xln: usize) -> bool {
        let p = self;
        p.0 >= 0 && p.1 >= 0 && p.0 < xln as i32 && p.1 < yln as i32
    }
    fn north(&self) -> Pos {
        let &Pos(x, y) = self;
        Pos(x, y - 1)
    }
    fn south(&self) -> Pos {
        let &Pos(x, y) = self;
        Pos(x, y + 1)
    }
    fn east(&self) -> Pos {
        let &Pos(x, y) = self;
        Pos(x + 1, y)
    }
    fn west(&self) -> Pos {
        let &Pos(x, y) = self;
        Pos(x - 1, y)
    }
}

struct Grid(Vec<Vec<u8>>);
impl Grid {
    fn yln(&self) -> usize {
        self.0.len()
    }
    fn xln(&self) -> usize {
        if self.yln() > 0 {
            return self.0[0].len();
        }
        0
    }
    fn at(&self, p: Pos) -> u8 {
        self.0[p.1 as usize][p.0 as usize]
    }
    fn empty(&self, p: Pos) -> bool {
        self.at(p) == b'.'
    }
    fn get(&self, p: Pos, d: Dir) -> Option<Pos> {
        let next = match d {
            Dir::East => p.east(),
            Dir::West => p.west(),
            Dir::North => p.north(),
            Dir::South => p.south(),
        };
        if next.is_valid(self.yln(), self.xln()) {
            return Some(next);
        }
        None
    }
    fn neigh(&self, p: Pos) -> Vec<Pos> {
        Dir::all()
            .iter()
            .filter(|&&d| opens(self.at(p), d))
            .filter_map(|&d| {
                self.get(p, d)
                    .and_then(|n| match opens(self.at(n), d.invert()) {
                        true => Some(n),
                        false => None,
                    })
            })
            .collect()
    }
}

fn calculate_range(x1: i32, x2: i32) -> Vec<i32> {
    let start = std::cmp::min(x1, x2); // +1 to exclude the starting point
    let end = std::cmp::max(x1, x2); // end is exclusive in Rust ranges
    (start..end).collect()
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Dist(Pos, usize);

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("Failed to read file");
    let grid: Vec<Vec<u8>> = content
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect();

    let columns: usize = grid.len();
    let rows: usize = grid[0].len();
    let g = Grid(grid);

    let mut galaxy_positions: Vec<Pos> = Vec::new();
    let mut galaxy_count = 0;

    let mut expand_x: Vec<i32> = Vec::with_capacity(columns);
    for _ in 0..columns {
        expand_x.push(1);
    }
    let mut expand_y: Vec<i32> = Vec::with_capacity(rows);
    for _ in 0..rows {
        expand_y.push(1);
    }

    // search for empty rows and update expand_y to multiplier 
    for y in 0..g.yln() {
        let mut galaxies = 0;
        for x in 0..g.xln() {
            let curr = Pos(x as i32, y as i32);
            if g.at(curr) == b'#' {
                galaxies += 1;
                galaxy_positions.push(curr);
                galaxy_count += 1;
            }
        }
        if galaxies == 0 {
            expand_y[y] = 1000000;
            println!("emtpy row: {}", y);
        }
    }
     // search for empty columns and update expand_x to multiplier 
    for x in 0..g.xln() {
        let mut galaxies = 0;
        for y in 0..g.yln() {
            let curr = Pos(x as i32, y as i32);
            if g.at(curr) == b'#' {
                galaxies += 1;
            }
        }
        if galaxies == 0 {
            expand_x[x] = 1000000;
            println!("emtpy column: {}", x);
        }
    }
    println!("galaxy count: {}", galaxy_count);
    println!("galaxy_positions: {:?}", galaxy_positions);

    let mut galaxy_pairs = Vec::new();
    let mut pair_count = 0;
    for i in 0..galaxy_count {
        for j in i+1..galaxy_count {
            galaxy_pairs.push([&galaxy_positions[i], &galaxy_positions[j]]);
            pair_count += 1;
        }
    }
    println!("pair count: {}", pair_count);

    let mut total_distance = 0;
    let mut total_calculated_distance: i64 = 0;

    galaxy_pairs.iter().for_each(|[near_galaxy, distant_galaxy]| {
        let x_dist = (near_galaxy.0 - distant_galaxy.0).abs();
        let calculated_x_dist = calculate_range(near_galaxy.0, distant_galaxy.0).iter().map(|x| expand_x[*x as usize] as i64).sum::<i64>();
        let y_dist = (near_galaxy.1 - distant_galaxy.1).abs();
        let calculated_y_dist = calculate_range(near_galaxy.1, distant_galaxy.1).iter().map(|y| expand_y[*y as usize] as i64).sum::<i64>();
        let distance = x_dist + y_dist;
        let calculated_distance = calculated_x_dist + calculated_y_dist;
        total_distance += distance;
        total_calculated_distance += calculated_distance;
        println!("{:?} {:?} distance {} expansion distance {}",near_galaxy, distant_galaxy, distance, calculated_distance);
    });
    
    println!("total distance: {}", total_distance);
    println!("total expansion distance: {}", total_calculated_distance);

    // let mut start = Pos(0, 0);

    // let mut queue = VecDeque::new();
    // let mut visited = HashSet::new();
    // for y in 0..g.yln() {
    //     for x in 0..g.xln() {
    //         let curr = Pos(x as i32, y as i32);
    //         if g.at(curr) == b'S' {
    //             visited.insert(curr);
    //             queue.push_back(Dist(curr, 0))
    //         }
    //     }
    // }
    // while let Some(Dist(p, d)) = queue.pop_front() {
    //     if !g.empty(p) && g.neigh(p).iter().all(|n| visited.contains(n)) {
    //         println!("{}", d + 1);
    //         break;
    //     }
    //     for n in g.neigh(p) {
    //         if !visited.contains(&n) {
    //             visited.insert(n);
    //             queue.push_back(Dist(n, d + 1))
    //         }
    //     }
    // }
}
