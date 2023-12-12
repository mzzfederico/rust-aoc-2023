use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::{Solution, SolutionPair};
use memoize::memoize;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Spring {
    Broken,
    Unkn,
    Empty,
}

#[memoize]
fn calculate_combinations(springs: Vec<Spring>, nums: Vec<u64>) -> u64 {
    if nums.is_empty() {
        if springs.is_empty() {
            return 1 as u64;
        } else {
            if springs
                .iter()
                .all(|c| *c == Spring::Empty || *c == Spring::Unkn)
            {
                return 1 as u64;
            } else {
                return 0 as u64;
            }
        }
    } else if springs.is_empty() {
        return 0 as u64;
    }

    let cursor = springs[0];

    match cursor {
        Spring::Empty => calculate_combinations(
            springs
                .iter()
                .skip_while(|c| **c == Spring::Empty)
                .map(|c| *c)
                .collect_vec(),
            nums,
        ),
        _ => {
            let mut acc = 0;
            if cursor == Spring::Unkn {
                acc += calculate_combinations(springs[1..].to_vec(), nums.clone());
            }
            let blanks = nums[0] as usize;
            if springs.len() >= blanks
                && springs[..blanks]
                    .iter()
                    .all(|c| *c == Spring::Broken || *c == Spring::Unkn)
            {
                let springs = springs[blanks..].to_vec();
                let nums = nums[1..].to_vec();
                if !springs.is_empty() {
                    if springs[0] == Spring::Empty || springs[0] == Spring::Unkn {
                        acc += calculate_combinations(springs[1..].to_vec(), nums);
                    }
                } else {
                    acc += calculate_combinations(springs, nums);
                }
            }
            acc
        }
    }
}

fn parse_input(input: &str, second_step: bool) -> (Vec<Spring>, Vec<u64>) {
    let (mut springs, mut nums) = input
        .split_ascii_whitespace()
        .map(|x| x.to_string())
        .collect_tuple()
        .unwrap();

    if second_step {
        springs = [
            springs.clone(),
            springs.clone(),
            springs.clone(),
            springs.clone(),
            springs,
        ]
        .join("?");

        nums = [nums.clone(), nums.clone(), nums.clone(), nums.clone(), nums].join(",");
    }

    let nums = nums
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .collect_vec();

    let springs = springs
        .chars()
        .map(|c| match c {
            '.' => Spring::Empty,
            '?' => Spring::Unkn,
            '#' => Spring::Broken,
            _ => panic!("Invalid spring: {}", c),
        })
        .collect_vec();

    (springs, nums)
}

pub fn solve() -> SolutionPair {
    let sol1: u64 = read_to_string("input/days/day12.txt")
        .expect("Error reading input file!")
        .split('\n')
        .par_bridge()
        .map(|r| parse_input(r, false))
        .map(|(springs, nums)| calculate_combinations(springs, nums))
        .sum::<u64>();

    let sol2: u64 = read_to_string("input/days/day12.txt")
        .expect("Error reading input file!")
        .split('\n')
        .par_bridge()
        .map(|r| parse_input(r, true))
        .map(|(springs, nums)| calculate_combinations(springs, nums))
        .sum::<u64>();

    (Solution::from(sol1), Solution::from(sol2))
}
