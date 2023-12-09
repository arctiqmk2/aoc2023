#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_imports)]

//use std::collections::HashMap;
use std::env;
use std::fs;


#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Card(char);

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    counts: Vec<i64>,
    values: Vec<i64>,
    handtype: HandType,
    bid: i64,
    rank: i64,
    score: i64,
    sort1: i64,
    sort2: i64,
}

fn main() {

    let mut hands: Vec<Hand> = Vec::new();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("Failed to read file");
    let lines: Vec<&str> = content.lines().collect();

    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let cards = parts[0].parse::<String>().unwrap_or("".to_string());
        let bid = parts[1].parse::<i64>().unwrap_or(0);
        let mut hand = Hand {
            cards: Vec::new(),
            counts: vec![0; 14],
            values: Vec::new(),
            handtype: HandType::HighCard,
            bid: 0,
            rank: 0,
            score: 0,
            sort1: 0,
            sort2: 0,
        };
        hand.bid = bid;
        for card in cards.chars() {
            hand.cards.push(Card(card));
            let card_index: usize = match card {
                'A' => 0,
                'K' => 1,
                'Q' => 2,
                'J' => 3,
                'T' => 4,
                '9' => 5,
                '8' => 6,
                '7' => 7,
                '6' => 8,
                '5' => 9,
                '4' => 10,
                '3' => 11,
                '2' => 12,
                _ => 13
            };
            let card_value: i64 = 13-card_index as i64;
            hand.counts[card_index] += 1;
            hand.values.push(card_value);
            hand.sort2 = hand.sort2 * 100 + card_value;
        }
            // implement handtype here.
            // five, four, full, three, two pair, one pair, high
            // 7, 6, 5, 4, 3, 2, 1
            if hand.counts.iter().find(|&&x| x == 5).is_some() {
                hand.handtype = HandType::FiveOfAKind;
                hand.sort1 = 7;
            }
            else if hand.counts.iter().find(|&&x| x == 4).is_some() {
                hand.handtype = HandType::FourOfAKind;
                hand.sort1 = 6;
            }
            else if hand.counts.iter().find(|&&x| x == 3).is_some() {
                if hand.counts.iter().find(|&&x| x == 2).is_some() {
                    hand.handtype = HandType::FullHouse;
                    hand.sort1 = 5;
                } else {
                    hand.handtype = HandType::ThreeOfAKind;
                    hand.sort1 = 4;
                }
            }
            else if hand.counts.iter().filter(|&&x| x == 2).count() == 2 {
                hand.handtype = HandType::TwoPair;
                hand.sort1 = 3;
            }
            else if hand.counts.iter().find(|&&x| x == 2).is_some() {
                hand.handtype = HandType::OnePair;
                hand.sort1 = 2;
            }
            else {
                hand.handtype = HandType::HighCard;
                hand.sort1 = 1;
            }
        
        hands.push(hand);
    }
    hands.sort_unstable_by(|b, a| b.sort1.cmp(&a.sort1).then(b.sort2.cmp(&a.sort2)));
    hands.iter_mut().enumerate().for_each(|(i, hand)| hand.rank = i as i64 + 1);
    hands.iter_mut().enumerate().for_each(|(i, hand)| hand.score = hand.rank * hand.bid);
    let mut total: i64 = 0;
    for hand in &hands {
        total += hand.score;
    }
    println!("Hands {:#?}", hands);
    println!("Total: {}", total);    
}





