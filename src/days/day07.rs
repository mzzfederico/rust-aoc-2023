use crate::{Solution, SolutionPair};
use core::fmt;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////
type Cards = String; // es. 32T3K
type Strength = String; // digit matching the strength of the hand
type CardValue = String; // card value turned into hexadecimal digit
type HexHand = String; // string to use as index of hands
type Bid = u32;

enum Score {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Score::FiveOfAKind => write!(f, "6"),
            Score::FourOfAKind => write!(f, "5"),
            Score::FullHouse => write!(f, "4"),
            Score::ThreeOfAKind => write!(f, "3"),
            Score::TwoPair => write!(f, "2"),
            Score::OnePair => write!(f, "1"),
            Score::HighCard => write!(f, "0"),
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: String,
    hex: HexHand,
    hex2: HexHand,
    bid: Bid,
}

fn score_hand(cards: &Cards, jokers: bool) -> Strength {
    let mut piles: [usize; 13] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    for card in cards.chars() {
        let value = usize::from_str_radix(card_to_value(card).as_str(), 16).unwrap();
        piles[value] += 1;
    }

    let piles = piles.to_vec();

    let piles_total = piles.iter().filter(|x| **x != 0).count();
    let max_pile = piles.iter().max().unwrap().clone();
    let jokers_total = if jokers { piles[9] } else { 0 };

    let tuple = (piles_total, max_pile, jokers_total);

    let score = match tuple {
        (1, _, _) => Score::FiveOfAKind,
        (2, 4, 0) => Score::FourOfAKind,
        (2, 4, _) => Score::FiveOfAKind,
        (2, 3, 0) => Score::FullHouse,
        (2, 3, _) => Score::FiveOfAKind,
        (3, 3, 0) => Score::ThreeOfAKind,
        (3, 3, _) => Score::FourOfAKind,
        (3, 2, 0) => Score::TwoPair,
        (3, 2, 1) => Score::FullHouse,
        (3, 2, 2) => Score::FourOfAKind,
        (4, _, 0) => Score::OnePair,
        (4, _, _) => Score::ThreeOfAKind,
        (5, _, 0) => Score::HighCard,
        (5, _, 1) => Score::OnePair,
        _ => Score::OnePair,
    };

    format!("{score}")
}

fn card_to_value(card: char) -> CardValue {
    let kinds: [char; 13] = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];
    /* let kinds_jokers: [char; 13] = [
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ]; */

    let value = kinds
        .iter()
        .rev() // reversed so that stronger values gets lower ranks
        .position(|k| *k == card)
        .expect("Unexpected card value found!");

    format!("{value:X}") // turns value into hex
}

fn card_to_value_jokers(card: char) -> CardValue {
    let kinds_jokers: [char; 13] = [
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ];

    let value = kinds_jokers
        .iter()
        .rev() // reversed so that stronger values gets lower ranks
        .position(|k| *k == card)
        .expect("Unexpected card value found!");

    format!("{value:X}") // turns value into hex
}

fn cards_into_hex(cards: &String) -> HexHand {
    let mut hex_hand = "".to_string();

    hex_hand.push_str(score_hand(cards, false).as_str());

    cards
        .chars()
        .for_each(|c| hex_hand.push_str(card_to_value(c).as_str()));

    hex_hand
}

fn cards_into_hex_joker(cards: &String) -> HexHand {
    let mut hex_hand = "".to_string();

    hex_hand.push_str(score_hand(cards, true).as_str());

    cards
        .chars()
        .for_each(|c| hex_hand.push_str(&card_to_value_jokers(c).as_str()));

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
                hex: cards_into_hex(&cards),
                hex2: cards_into_hex_joker(&cards),
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
