#![allow(dead_code)]
#![allow(unused_variables)]
use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use std::path::Path;
//use std::collections::HashSet;
//use std::collections::HashMap;
use regex::Regex;
use std::str::FromStr;


// Struct to represent a number and its properties
struct CardInfo {
    card_number: i32,
    winning_numbers: Vec<i32>,
    drawn_numbers: Vec<i32>,
    value: i32,
    cards_won: i32,
    count: i32,
}

fn main() -> io::Result<()> {
    // Retrieve the filename from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];

    // Open the specified file
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut cards_info:Vec<CardInfo> = Vec::new();
    let mut total_value = 0;

    let re = Regex::new(r"Card\s+(\d+):\s*([0-9\s]+)\|\s*([0-9\s]+)").unwrap();

    for line in reader.lines() {
            let card = line.unwrap().to_string();
            let cap = re.captures(&card).unwrap();
            let card_number = cap[1].parse::<i32>().unwrap();
     
            let winning_numbers: Vec<i32> = cap[2].split_whitespace()
                .filter_map(|num| i32::from_str(num).ok())
                .collect();
     
            let drawn_numbers: Vec<i32> = cap[3].split_whitespace()
                .filter_map(|num| i32::from_str(num).ok())
                .collect();

            let mut score = 0;
            let mut cards_won = 0;

            for &drawn_number in &drawn_numbers {
                if winning_numbers.contains(&drawn_number) {
                    if score == 0 {
                        score = 1; // Set score to 1 on the first match
                        cards_won = 1;
                    } else {
                        score <<= 1; // Double the score on subsequent matches
                        cards_won += 1;
                    }
                }
            }

            //println!("Card Number: {}, Winning Numbers: {:?}, Drawn Numbers: {:?}, Score: {:?}", card_number, winning_numbers, drawn_numbers, score);

            cards_info.push(CardInfo {
                card_number: card_number,
                winning_numbers: winning_numbers,
                drawn_numbers: drawn_numbers,
                value: score,
                cards_won: cards_won,
                count: 1,
            });
            total_value += score;
        }   
    println!("Total Score: {}", total_value);
    
    let cards:usize = cards_info.len().try_into().unwrap();
    let mut number_of_cards = 0;

    let mut card_counts = vec![1; cards];
    for card_number in 0..cards {
        let card_number: usize = card_number;
        number_of_cards += 1;
        let cards_won:usize = cards_info[card_number].cards_won.try_into().unwrap();
        let current_copies = cards_info[card_number].count;
        //println!("Card Number: {}, Score: {}, Copies: {}", cards_info[card_number].card_number, cards_won, current_copies);
        if cards_won > 0 {
            let mut next_card: usize = (card_number + 1).try_into().unwrap();
            let mut last_card: usize  = (card_number + cards_won + 1).try_into().unwrap();
            if next_card >= cards {
                next_card = cards;
            }
            if last_card >= cards {
                last_card = cards;
            }
            if last_card > next_card {
                //println!("Winning {} copies of Next Card: {} to Last Card: {}", current_copies, next_card, last_card);
                for next_card_number in next_card..last_card {
                    let next_card_number: usize = next_card_number.try_into().unwrap();
                    number_of_cards += current_copies;
                    card_counts[next_card_number] += current_copies;
                    cards_info[next_card_number].count += current_copies;
                }
            }

        }
        
    }
    println!("Number of Cards: {}", number_of_cards);


    Ok(())
}