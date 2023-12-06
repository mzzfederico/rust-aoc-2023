use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

fn match_text_digit(word: &str) -> Option<String> {
    // so we can match string to index
    let nums: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for (i, num) in nums.iter().enumerate() {
        if word.contains(num) {
            let value = (i + 1).to_string();
            return Some(value);
        }
    }

    None
}

fn get_first_digit(line: String, reverse: bool) -> String {
    let mut acc: String = "".to_string();

    for char in line.chars() {
        let s = char.to_string();
        if char.is_ascii_digit() {
            return s;
        } else {
            acc.push_str(s.as_str());
            // reverse to check when going backwards
            let matching_num = if reverse {
                let reverse_string = acc.chars().rev().collect::<String>();
                match_text_digit(reverse_string.as_str())
            } else {
                match_text_digit(&acc)
            };

            if let Some(found) = matching_num {
                return found;
            }
        }
    }

    "".to_string()
}

pub fn solve() -> SolutionPair {
    let example_input_1 = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
    println!("Example input 1: {:?}", example_input_1);

    let full_input = read_to_string("input/days/day01.txt").expect("Could not read day01.txt");

    // Part 1
    let mut _hidden_digits_part_one: Vec<u64> = vec![];

    for input in full_input.split('\n') {
        let mut full_digit: String = "".to_owned();
        for char in input.chars() {
            if char.is_numeric() {
                full_digit.push_str(&char.to_string());
                break;
            }
        }
        for char in input.chars().rev() {
            if char.is_numeric() {
                full_digit.push_str(&char.to_string());
                break;
            }
        }

        // print!("Input: {:?} | Full digit: {:?} | ", input, full_digit);

        let sum_int: u64 = full_digit.parse().unwrap_or_default();
        _hidden_digits_part_one.push(sum_int);
    }

    // Part 2
    let example_input_2 = vec![
        "two1nine",
        "eightwothree",
        "abcone2threexyz",
        "xtwone3four",
        "4nineeightseven2",
        "zoneight234",
        "7pqrstsixteen",
    ];
    println!("Example input 2: {:?}", example_input_2);

    let mut _hidden_digits_part_two: Vec<u64> = vec![];

    //for input in example_input_2 {
    for input in full_input.split('\n') {
        let mut left: String = get_first_digit(input.to_string(), false);
        let right: String = get_first_digit(input.chars().rev().collect(), true);
        left.push_str(right.as_str());

        _hidden_digits_part_two.push(left.parse().unwrap_or_default());
    }

    let sol1: u64 = _hidden_digits_part_one.iter().sum();
    let sol2: u64 = _hidden_digits_part_two.iter().sum();

    (Solution::from(sol1), Solution::from(sol2))
}
