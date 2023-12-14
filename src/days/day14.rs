use pathfinding::{cycle_detection::brent, matrix::Matrix};

use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Rock {
    Round,
    Square,
    Empty,
}

impl std::fmt::Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rock::Round => write!(f, "O"),
            Rock::Square => write!(f, "#"),
            Rock::Empty => write!(f, "."),
        }
    }
}

fn cycle(grid: &mut Matrix<Rock>) {
    for _ in 0..4 {
        tilt(grid);
        grid.rotate_cw(1);
    }
}

fn tilt(grid: &mut Matrix<Rock>) {
    for x in 0..grid.columns {
        let mut free = 0;
        for y in 0..grid.rows {
            match grid[(y, x)] {
                Rock::Square => free = y + 1,
                Rock::Round => {
                    grid.swap((y, x), (free, x));
                    free += 1;
                }
                _ => {}
            }
        }
    }
}

fn parse_grid(input: String) -> Matrix<Rock> {
    Matrix::from_rows(input.lines().map(|r| {
        r.chars().map(|c| match c {
            '.' => Rock::Empty,
            '#' => Rock::Square,
            'O' => Rock::Round,
            _ => panic!("Invalid input"),
        })
    }))
    .unwrap()
}

fn calculate_pressure(grid: &Matrix<Rock>) -> u64 {
    grid.iter()
        .enumerate()
        .map(|(y, r)| {
            r.iter()
                .map(|r| match r {
                    Rock::Round => grid.columns as u64 - y as u64,
                    _ => 0,
                })
                .sum::<u64>()
        })
        .sum::<u64>()
}

fn part2(grid: &mut Matrix<Rock>) -> u64 {
    const LIMIT: usize = 1_000_000_000;
    let (len, mut last, idx) = brent(grid.clone(), |mut g| {
        cycle(&mut g);
        g
    });
    for _ in 0..(LIMIT - idx) % len {
        cycle(&mut last);
    }
    calculate_pressure(&last)
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/days/day14.txt").unwrap();
    let mut grid = parse_grid(input.clone());
    tilt(&mut grid);

    let sol1: u64 = calculate_pressure(&grid);

    /* let mut grid_2 = parse_grid(input.clone());
    let sol2: u64 = part_two(&grid_2); */
    let mut grid = parse_grid(input.clone());
    let sol2 = part2(&mut grid);

    (Solution::from(sol1), Solution::from(sol2))
}
