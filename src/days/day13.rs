use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};

use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

fn calculate_reflection_points(map: Vec<String>) -> u64 {
    let mut mirror_ver = 0;
    for i in 1..map.len() {
        println!("{} {}", i, map.len());

        let (top, bottom) = map.split_at(i);
        let mut top = top.iter().rev().collect_vec();
        let mut bottom = bottom.iter().collect_vec();
        let max = top.len().min(bottom.len());
        top.truncate(max);
        bottom.truncate(max);

        if top
            .iter()
            .zip(bottom.iter())
            .map(|(a, b)| (a.as_str() == b.as_str()))
            .all(|r| r == true)
        {
            mirror_ver = i;
        }
    }

    let mut mirror_hor = 0;
    for i in 1..map[0].len() {
        let reflecting = map
            .par_iter()
            .map(|r| {
                let (left, right) = r.split_at(i);
                let max = left.len().min(right.len());

                let mut left = left.chars().rev().collect_vec();
                let mut right = right.chars().collect_vec();
                left.truncate(max);
                right.truncate(max);

                left.iter()
                    .zip(right.iter())
                    .map(|(a, b)| a == b)
                    .all(|r| r == true)
            })
            .all(|r| r == true);
        if reflecting {
            mirror_hor = i;
        }
    }

    println!("{} {}", mirror_ver, mirror_hor);

    return mirror_ver as u64 * 100 + mirror_hor as u64;
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/days/day13.txt").expect("Cannot find the file");

    let sol1: u64 = input
        .split("\n\n")
        .par_bridge()
        .map(|set| {
            calculate_reflection_points(set.split("\n").map(|r| r.to_string()).collect_vec())
        })
        .sum();

    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}
