extern crate itertools;
use crate::{Solution, SolutionPair};
use itertools::Itertools;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

fn reduce_to_diffs(nums: &Vec<i64>) -> Vec<i64> {
    nums.iter()
        .tuple_windows()
        .collect_vec()
        .iter()
        .map(|(a, b)| *b - *a)
        .collect()
}

fn parse_into_zeroes(start_sequence: Vec<i64>) -> Vec<Vec<i64>> {
    let mut sequences: Vec<Vec<i64>> = vec![];

    sequences.push(start_sequence);

    while sequences
        .iter()
        .last()
        .expect("parse_into_zeroes: empty input of vectors")
        .iter()
        .to_owned()
        .sum::<i64>()
        != 0
    {
        sequences.push(reduce_to_diffs(sequences.last().unwrap()))
    }

    sequences
}

pub fn solve() -> SolutionPair {
    let zero: i64 = 0;

    // Your solution here...
    let sol1: i64 = read_to_string("input/days/day09.txt")
        .expect("Error fetching input file")
        .split("\n")
        .map(|r| {
            parse_into_zeroes(
                r.split_ascii_whitespace()
                    .map(|n| n.parse::<i64>().expect("Error trying to parse a number"))
                    .collect::<Vec<i64>>(),
            )
            .iter()
            .fold(0, |acc, curr| acc + curr.last().unwrap_or(&zero))
        })
        .collect::<Vec<i64>>()
        .iter()
        .sum();

    let sol2: i64 = read_to_string("input/days/day09.txt")
        .expect("Error fetching input file")
        .split("\n")
        .map(|r| {
            parse_into_zeroes(
                r.split_ascii_whitespace()
                    .map(|n| n.parse::<i64>().expect("Error trying to parse a number"))
                    .rev()
                    .collect::<Vec<i64>>(),
            )
            .iter()
            .fold(0, |acc, curr| acc + curr.last().unwrap_or(&zero))
        })
        .collect::<Vec<i64>>()
        .iter()
        .sum();

    (Solution::from(sol1), Solution::from(sol2))
}
