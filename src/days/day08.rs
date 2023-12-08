use rayon::iter::IntoParallelRefIterator;

use crate::{Solution, SolutionPair};
use rayon::iter::ParallelIterator;
use std::{collections::HashMap, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////
enum Direction {
    Left,
    Right,
}

// https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
pub fn lcm(nums: Vec<u64>) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(nums.split_at(1).1.into());
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

pub fn solve() -> SolutionPair {
    let input_text = read_to_string("input/days/day07.txt").expect("Cannot read file!");
    let lines: Vec<String> = input_text
        .split("\n")
        .into_iter()
        .map(|x| x.to_string())
        .collect();

    let instructions: Vec<Direction> = lines[0]
        .clone()
        .chars()
        .map(|c| {
            if c == 'L' {
                Direction::Left
            } else {
                Direction::Right
            }
        })
        .collect();

    let mut nodes: HashMap<String, (String, String)> = HashMap::new();

    lines.split_at(2).1.iter().for_each(|s| {
        let node = &s[0..3];
        let left = &s[7..10];
        let right = &s[12..15];

        nodes.insert(node.to_string(), (left.to_string(), right.to_string()));
    });

    let mut sol1: u64 = 0;
    let start: String = "AAA".to_string();
    let end: String = "ZZZ".to_string();
    let mut curr: String = start;

    loop {
        if curr == end {
            break;
        }
        for d in instructions.iter() {
            sol1 += 1;
            let curr_node = nodes.get(&curr).expect("Missing start!");
            curr = match d {
                Direction::Left => curr_node.0.clone(),
                Direction::Right => curr_node.1.clone(),
            };
            if curr == end {
                break;
            }
        }
    }

    let ghost_starts: Vec<String> = nodes
        .iter()
        .filter(|x| x.0.ends_with('A'))
        .map(|x| x.0.clone())
        .collect();

    let steps_vector = ghost_starts
        .par_iter()
        .map(|s| {
            let mut steps: u64 = 0;
            let mut curr: String = s.clone();

            loop {
                if curr.ends_with('Z') {
                    break;
                }
                for d in instructions.iter() {
                    steps += 1;
                    let curr_node = nodes.get(&curr).expect("Missing start!");
                    curr = match d {
                        Direction::Left => curr_node.0.clone(),
                        Direction::Right => curr_node.1.clone(),
                    };
                    if curr.ends_with('Z') {
                        break;
                    }
                }
            }

            steps
        })
        .collect::<Vec<u64>>();

    let sol2 = lcm(steps_vector);

    (Solution::from(sol1), Solution::from(sol2))
}
