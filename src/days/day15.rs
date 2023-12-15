use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::{Solution, SolutionPair};
use itertools::Itertools;

use std::collections::HashMap;
use std::fs::read_to_string;

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
    let input_string =
        read_to_string("input/days/day15.txt").expect("Input file could not be opened");

    /* let input_string = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string();
     */
    let operations = input_string.trim().split(',').collect::<Vec<&str>>();

    let hashes = operations.par_iter().map(|l| hash(l));

    // Your solution here...
    let sol1: u32 = hashes.sum();

    let mut boxes: HashMap<u64, Vec<(String, u64)>> = HashMap::new();

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
            .parse::<u64>()
            .unwrap_or(0);

        if op_lens_focal == 0 && op_type == OpType::Equal {
            println!("{op} - type: {op_type:?} - label: {label:?} symbol: {symbol:?}");
            panic!("Invalid equal operation!");
        }

        if op_lens_focal != 0 && op_type == OpType::Dash {
            println!("{op} - type: {op_type:?} - label: {label:?} symbol: {symbol:?}");
            panic!("Invalid dash operation!");
        }

        if hash > 255 {
            println!("{op} - type: {op_type:?} - label: {label:?} symbol: {symbol:?}");
            panic!("Hash is too damn hight!");
        }

        match op_type {
            OpType::Dash => {
                let entry = &mut boxes.entry(hash as u64).or_default();
                let entry: Vec<(String, u64)> = entry
                    .iter()
                    .filter(|x| x.0 != label)
                    .map(|l| (l.0.clone(), l.1))
                    .collect();

                boxes.insert(hash as u64, entry.clone());
            }
            OpType::Equal => {
                let entry = boxes.entry(hash as u64).or_default();
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

    for i in 0..boxes.capacity() {
        let lens_box = boxes.get(&(i as u64));
        if lens_box.is_some() {
            sol2 += lens_box
                .unwrap()
                .iter()
                .enumerate()
                .inspect(|(_, (label, _))| {
                    let local_hash = hash(&label) as u64;
                    let is_hash_correct = local_hash == i as u64;

                    if !is_hash_correct {
                        println!(
                            "Hashes do not match! - {label} - {local_hash} - {i}",
                            label = label,
                            local_hash = local_hash,
                            i = i
                        );
                    }
                })
                .map(|(slot, (_, focal_length))| {
                    (i as u64 + 1) * ((slot + 1) as u64) * (*focal_length as u64)
                })
                .sum::<u64>();
        };
    }

    (Solution::from(sol1), Solution::from(sol2))
}
