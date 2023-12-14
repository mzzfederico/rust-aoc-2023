use itertools::Itertools;
use memoize::memoize;

use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rock {
    Round,
    Square,
    Empty,
}

#[memoize]
fn push_north(initial_map: String) -> String {
    let mut map = initial_map
        .split('\n')
        .map(|r| {
            r.chars()
                .map(|c| match c {
                    'O' => Rock::Round,
                    '#' => Rock::Square,
                    _ => Rock::Empty,
                })
                .collect_vec()
        })
        .collect_vec();

    let rocks = &map
        .iter()
        .enumerate()
        //.par_bridge()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                //.par_bridge()
                .filter(|(_, rock)| match rock {
                    Rock::Round => true,
                    _ => false,
                })
                .map(move |(x, _)| (x.clone(), y.clone()))
        })
        .flatten()
        .collect_vec();

    for (x, y) in rocks.iter() {
        let mut column: Vec<Rock> = vec![];
        let boundary: usize = *y;
        for i in 0..boundary {
            let rock = &map[i][*x];
            column.push(*rock);
        }
        column.reverse();

        let spaces = column
            .iter()
            .take_while(|r| **r != Rock::Square)
            .filter(|r| **r == Rock::Empty)
            .count();
        if spaces > 0 {
            map[*y][*x] = Rock::Empty;
            map[*y - spaces][*x] = Rock::Round;
        }
    }

    let new = map
        .iter()
        .map(|row| {
            row.iter()
                .map(|rock| match rock {
                    Rock::Round => 'O',
                    Rock::Square => '#',
                    _ => '.',
                })
                .collect::<String>()
        })
        .join("\n");

    new
}

fn calculate_pressure(map: String) -> u64 {
    let height: usize = map.split('\n').count();

    map.split('\n')
        .enumerate()
        .map(|(y, r)| {
            let weight = y;
            r.chars()
                .map(|c| match c {
                    'O' => height as u64 - weight as u64,
                    '#' => 0,
                    _ => 0,
                })
                .sum::<u64>()
        })
        .sum::<u64>()
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/days/day14.txt").unwrap();

    // Your solution here...
    let sol1: u64 = calculate_pressure(push_north(input.clone()));

    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}
