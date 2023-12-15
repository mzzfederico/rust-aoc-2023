use itertools::Itertools;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::{Solution, SolutionPair};
use std::{fs::read_to_string, collections::HashMap};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input_string =
        read_to_string("input/days/day15.txt").expect("Input file could not be opened");

    let operations = input_string
        .trim()
        .split(',')
        .collect::<Vec<&str>>();

    let hashes = operations
        .par_iter()
        .map(|s| {
            let mut count = 0;
            for c in s.chars() {
                let ascii = c as u32;
                count += ascii;
                count = count * 17;
                count = count % 256;
            }
            count
        });

    // Your solution here...
    let sol1: u32 = hashes.sum();

    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}
