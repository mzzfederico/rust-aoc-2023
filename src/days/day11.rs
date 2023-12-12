use itertools::Itertools;

use crate::{Solution, SolutionPair};
use std::{collections::HashSet, fs::read_to_string};

type XY = (u64, u64);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Point {
    Galaxy,
    Space,
}

fn magnitude(a: &XY, b: &XY) -> u64 {
    let (x1, y1) = a;
    let (x2, y2) = b;

    let x = *x1 as i64 - *x2 as i64;
    let y = *y1 as i64 - *y2 as i64;

    (x.abs() + y.abs()) as u64
}

fn calculate_older_universe(input: String, ratio: u64) -> u64 {
    let origin = input
        .split("\n")
        .map(|r| {
            r.chars()
                .map(|c| match c {
                    '#' => Point::Galaxy,
                    '.' => Point::Space,
                    _ => Point::Space,
                })
                .collect::<Vec<Point>>()
        })
        .collect::<Vec<Vec<Point>>>();

    let mut remote_galaxies = HashSet::<XY>::new();

    let mut x = 0;
    let mut y = 0;
    let mut r = 0;
    let mut c = 0;

    while r < origin.iter().len() {
        let is_double = origin[r].iter().all(|c| *c == Point::Space);

        if is_double {
            y += ratio;
            r += 1;
            continue;
        }

        while c < origin[r].iter().len() {
            let column: Vec<Point> = origin.iter().map(|r| r[c]).collect();
            let is_double = column.iter().all(|c| *c == Point::Space);

            if is_double {
                x += ratio;
                c += 1;
                continue;
            }
            match origin[r][c] {
                Point::Galaxy => {
                    remote_galaxies.insert((x, y));
                }
                _ => (),
            }

            c += 1;
            x += 1;
        }

        r += 1;
        y += 1;
        x = 0;
        c = 0;
    }

    return remote_galaxies
        .into_iter()
        .combinations(2)
        .map(|v| magnitude(&v[0], &v[1]))
        .sum::<u64>();
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/days/day11.txt").expect("Missing input file for day 11!");

    let sol1: u64 = calculate_older_universe(input.clone(), 2);
    let sol2: u64 = calculate_older_universe(input.clone(), 1000000);

    (Solution::from(sol1), Solution::from(sol2))
}
