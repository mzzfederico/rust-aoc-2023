use crate::{Solution, SolutionPair};
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

fn multiplier(index: u32) -> u64 {
    let base: u64 = 2;
    if index == 0 {
        return 0;
    }
    base.pow(index - 1)
}

fn calculate_cards_won(index: u64, score: u32, max_card_id: u64) -> Vec<u64> {
    let mut cards_won: Vec<u64> = vec![];
    let mut i = 0;
    while i < (score) {
        let value = index + i as u64 + 1;
        if value > max_card_id {
            break;
        }
        cards_won.push(value);
        i += 1;
    }

    cards_won
}

fn parse_sidecard_numbers(side: String) -> HashSet<u64> {
    let mut set: HashSet<u64> = HashSet::new();
    let numbers = side.split_ascii_whitespace().map(|x| x.parse::<u64>());

    for num in numbers.flatten() {
        set.insert(num);
    }
    set
}

fn traverse_sum(hashmap: &HashMap<u64, Vec<u64>>, id: &u64) -> u64 {
    let cards = &hashmap.get(id);
    if cards.is_some() {
        let total = cards.unwrap().len() as u64;
        let cards_totals: u64 = cards
            .unwrap()
            .iter()
            .map(|c| traverse_sum(hashmap, c))
            .sum::<u64>();

        total + cards_totals
    } else {
        0
    }
}

pub fn solve() -> SolutionPair {
    //let example_input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    // Your solution here...
    let complete_input = read_to_string("input/days/day04.txt").unwrap();

    let mut total_points: u64 = 0;

    let cards = complete_input.split('\n');

    let mut replicating_cards_tree: HashMap<u64, Vec<u64>> = HashMap::new();
    let max = cards.clone().count() as u64;

    for (i, card) in cards.enumerate() {
        let card_values_string = card.split(": ").collect::<Vec<&str>>()[1];
        let sides = card_values_string.split(" | ").collect::<Vec<&str>>();

        let scratch_nums = parse_sidecard_numbers(sides[0].to_string());
        let winning_nums = parse_sidecard_numbers(sides[1].to_string());

        let matches = scratch_nums.intersection(&winning_nums);

        let result = matches.count() as u32;

        let id = i as u64;
        let cards_won: Vec<u64> = calculate_cards_won(id, result, max);
        replicating_cards_tree.insert(id, cards_won);

        //println!("{:?}{:?}{:?}", scratch_nums, winning_nums, &result);
        total_points += multiplier(result);
    }

    let sol1: u64 = total_points;

    let total_real_cards = replicating_cards_tree.len() as u64;
    let mut total_cards: u64 = 0;
    for c in 0..total_real_cards {
        let fakes_sum = traverse_sum(&replicating_cards_tree, &c);
        total_cards += fakes_sum + 1;
    }

    let sol2 = total_cards;

    (Solution::from(sol1), Solution::from(sol2))
}
