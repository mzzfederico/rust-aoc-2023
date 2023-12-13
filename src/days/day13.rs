use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

fn flip_str(s: &str) -> String {
    s.chars().rev().collect::<String>()
}

fn get_diffs(a: &str, b: &str) -> i32 {
    let a = a.chars().collect_vec();
    let b = b.chars().collect_vec();
    let max = a.len().min(b.len());

    let mut diffs: i32 = 0;
    for i in 0..max {
        if a[i] != b[i] {
            diffs += 1;
        }
    }
    return diffs;
}

fn calculate_diffs(map: Vec<&str>, diff_sum: u8) -> u64 {
    let mut horizontal_mirror = 0;
    for i in 1..map.len() {
        //println!("slice of rows: {}", i);
        let (top, bottom) = map.split_at(i);
        let top = top.iter().rev().collect_vec();
        let bottom = bottom.iter().collect_vec();

        if top
            .iter()
            .zip(bottom.iter())
            .map(|(top, bottom)| get_diffs(top, bottom))
            .sum::<i32>()
            == diff_sum as i32
        {
            horizontal_mirror = i;
        }
    }

    let mut vertical_mirror = 0;
    for i in 1..map[0].len() {
        if map
            .iter()
            .map(|r| r.split_at(i))
            .map(|(left, right)| get_diffs(flip_str(left).as_str(), right))
            .sum::<i32>()
            == diff_sum as i32
        {
            vertical_mirror = i as u64;
        }
    }

    return horizontal_mirror as u64 * 100 + vertical_mirror as u64;
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/days/day13.txt").expect("Cannot find the file");

    let early_solutions = input.split("\n\n").par_bridge();

    let sol1: u64 = early_solutions
        .clone()
        .map(|set| calculate_diffs(set.split("\n").collect_vec(), 0))
        .sum();

    let sol2: u64 = early_solutions
        .clone()
        .map(|set| calculate_diffs(set.split("\n").collect_vec(), 1))
        .sum();

    (Solution::from(sol1), Solution::from(sol2))
}
