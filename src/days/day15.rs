use rayon::iter::IndexedParallelIterator;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::{Solution, SolutionPair};
use itertools::Itertools;

use std::collections::HashMap;

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialOrd, PartialEq)]
enum OpType {
    Dash,
    Equal,
}

fn hash(s: &str) -> u32 {
    let mut count = 0;
    for c in s.chars() {
        let ascii = c as u32;
        count += ascii;
        count *= 17;
        count %= 256;
    }
    count
}

pub fn solve() -> SolutionPair {
    /* let input_string =
    read_to_string("input/days/day15.txt").expect("Input file could not be opened"); */

    let input_string = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string();

    let operations = input_string.trim().split(',').collect::<Vec<&str>>();

    let hashes = operations.par_iter().map(|l| hash(l));

    // Your solution here...
    let sol1: u32 = hashes.sum();

    let mut boxes: HashMap<u8, Vec<(String, u8)>> = HashMap::new();

    for op in operations.iter() {
        let label = op
            .chars()
            .take_while(|x| *x != '-' && *x != '=')
            .collect::<String>();

        let length = &label.len();

        let hash = hash(&label);

        let symbol = op.chars().nth(*length).unwrap();

        let op_type: OpType = match symbol {
            '-' => OpType::Dash,
            '=' => OpType::Equal,
            _ => OpType::Dash,
        };

        let op_lens_focal = op
            .chars()
            .skip(*length + 1)
            .collect::<String>()
            .parse::<u8>()
            .unwrap_or(0);

        match op_type {
            OpType::Dash => {
                let entry = &mut boxes.entry(hash as u8).or_default();
                let entry: Vec<(String, u8)> = entry
                    .iter()
                    .filter(|x| x.0 != label)
                    .map(|l| (l.0.clone(), l.1))
                    .collect();

                boxes.insert(hash as u8, entry.clone());
            }
            OpType::Equal => {
                let entry = boxes.entry(hash as u8).or_default();
                match entry.iter().find_position(|x| x.0 == label) {
                    Some((index, _)) => {
                        entry[index].1 = op_lens_focal;
                    }
                    None => {
                        entry.push((label.clone(), op_lens_focal));
                    }
                }
            }
        }
    }

    let mut sol2: u64 = 0;

    for i in 0..255 {
        let lens_box = boxes.get(&(i as u8));
        sol2 += match lens_box {
            Some(lens_box) => lens_box
                .par_iter()
                .enumerate()
                .map(|(slot, (_, focal_length))| (i + 1) * (slot + 1) as u64 * *focal_length as u64)
                .sum::<u64>(),
            None => 0,
        }
    }

    (Solution::from(sol1), Solution::from(sol2))
}
