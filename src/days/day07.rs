use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////
type Cards = String; // es. 32T3K
type Strength = String; // digit matching the strength of the hand
type CardValue = String; // card value turned into hexadecimal digit
type HexHand = String; // string to use as index of hands
type Bid = u32;

#[derive(Debug)]
struct Hand {
    cards: String,
    hex: HexHand,
    hex2: HexHand,
    bid: Bid,
}

fn score_hand(cards: &Cards, count_the_jokers: bool) -> Strength {
    let mut piles: [usize; 13] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut jokers_total = 0;

    for card in cards.chars() {
        if card == 'J' && count_the_jokers == true {
            jokers_total += 1;
        }
        let value =
            usize::from_str_radix(card_to_value(card, count_the_jokers).as_str(), 16).unwrap();
        piles[value] += 1;
    }

    let two: usize = 2;
    let piles = piles.to_vec();
    let max = piles.iter().max().unwrap();

    if *max == 5 {
        "6".to_string()
    } else if *max == 4 {
        match jokers_total {
            4 => "6".to_string(), // JJJJX => 6 five of a kind
            1 => "6".to_string(), // 1111J => 6 five of a kind
            _ => "5".to_string(),
        }
    } else if *max == 3 && piles.contains(&two) {
        match jokers_total {
            2 => "6".to_string(), // 111JJ => 6 five of a kind
            // 1 => can't happen!
            _ => "4".to_string(), // 111XX => 4 full house
        }
    } else if *max == 3 {
        match jokers_total {
            // 2 => can't happen!
            1 => "5".to_string(), // 111JX => 5 four of kind
            _ => "3".to_string(), // 111XY => 3 threr of a kind
        }
    } else if piles.iter().filter(|x| *x == &two).count() == two {
        match jokers_total {
            2 => "5".to_string(), // 11JJX => 5 four of kind
            1 => "5".to_string(), // 11JXX => 4 full house
            _ => "2".to_string(), // XXYYZ => 2 couples
        }
    } else if piles.contains(&two) {
        match jokers_total {
            1 => "4".to_string(), // 11JXY => 4 three of a kind
            _ => "1".to_string(), // 11XYZ => 1 pair
        }
    } else {
        match jokers_total {
            1 => "1".to_string(), // 1JXYZ => 1 pair
            _ => "0".to_string(), // 12345 => all different, no jokers
        }
    }
}

fn card_to_value(card: char, jokers_mode: bool) -> CardValue {
    let kinds: [char; 13] = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];
    let kinds_jokers: [char; 13] = [
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ];

    if jokers_mode {
        let value = kinds
            .iter()
            .rev() // reversed so that stronger values gets lower ranks
            .position(|k| *k == card)
            .expect("Unexpected card value found!");

        format!("{value:X}") // turns value into hex
    } else {
        let value = kinds_jokers
            .iter()
            .rev() // reversed so that stronger values gets lower ranks
            .position(|k| *k == card)
            .expect("Unexpected card value found!");

        format!("{value:X}") // turns value into hex
    }
}

fn cards_into_hex(cards: &String, jokers_mode: bool) -> HexHand {
    let mut hex_hand = "".to_string();

    hex_hand.push_str(score_hand(cards, jokers_mode).as_str());

    cards
        .chars()
        .for_each(|c| hex_hand.push_str(card_to_value(c, jokers_mode).as_str()));

    hex_hand
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/days/day07.txt").expect("Cannot read input 07");

    let mut hands: Vec<Hand> = input
        .split("\n")
        .map(|hand| {
            let sides: Vec<&str> = hand.split_ascii_whitespace().collect();
            let cards: String = sides[0].to_string();
            let bid = sides[1].parse::<Bid>().expect("Error parsing bid");

            Hand {
                cards: cards.clone(),
                hex: cards_into_hex(&cards, false),
                hex2: cards_into_hex(&cards, true),
                bid,
            }
        })
        .collect();

    hands.sort_by_key(|h| h.hex.clone());

    let mut sol1: u32 = 0;

    for (index, hand) in hands.iter().enumerate() {
        let rank = (index + 1) as u32;

        sol1 += hand.bid * rank;
    }

    hands.sort_by_key(|h| h.hex2.clone());

    let mut sol2: u32 = 0;

    for (index, hand) in hands.iter().enumerate() {
        let rank = (index + 1) as u32;

        sol2 += hand.bid * rank;
    }

    (Solution::from(sol1), Solution::from(sol2))
}
