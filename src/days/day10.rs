use std::collections::HashMap;

use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::ops::Index;

///////////////////////////////////////////////////////////////////////////////

type XY = (usize, usize);

#[derive(Debug, PartialEq, Clone, Copy)]
enum Dir {
    N,
    E,
    S,
    W,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Bend {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Pipe {
    Vertical,
    Horizontal,
    Turn(Bend),
    Ground,
    Start,
}

fn traverse(a: &XY, dir: &Dir) -> XY {
    (
        match dir {
            Dir::S => a.0 + 1,
            Dir::N => a.0 - 1,
            _ => a.0,
        },
        match dir {
            Dir::W => a.1 - 1,
            Dir::E => a.1 + 1,
            _ => a.1,
        },
    )
    /*  */
}

fn corner(s: &Dir, b: &Bend) -> Dir {
    match b {
        Bend::Left => match s {
            Dir::N => Dir::W,
            Dir::E => Dir::S,
            Dir::S => Dir::E,
            Dir::W => Dir::N,
        },
        Bend::Right => match s {
            Dir::N => Dir::E,
            Dir::E => Dir::N,
            Dir::S => Dir::W,
            Dir::W => Dir::S,
        },
    }
}

pub fn solve() -> SolutionPair {
    let mut start_pos: XY = (0, 0);

    let grid = read_to_string("input/days/day10.txt")
        .expect("Cannot find and split input file")
        .split('\n')
        .enumerate()
        .map(|(r, line)| {
            line.chars()
                .enumerate()
                .inspect(|(c, cell)| {
                    if *cell == 'S' {
                        start_pos = (r, *c);
                    }
                })
                .map(|t| match t.1 {
                    '|' => Pipe::Vertical,   // is a vertical pipe connecting north and south.
                    '-' => Pipe::Horizontal, // is a horizontal pipe connecting east and west.
                    'L' => Pipe::Turn(Bend::Left), // is a 90-degree bend connecting north and east.
                    'J' => Pipe::Turn(Bend::Right), // is a 90-degree bend connecting north and west.
                    '7' => Pipe::Turn(Bend::Left), // is a 90-degree bend connecting south and west.
                    'F' => Pipe::Turn(Bend::Right), // is a 90-degree bend connecting south and east.
                    '.' => Pipe::Ground,            // is ground; there is no pipe in this tile.
                    'S' => Pipe::Start,             // start
                    _ => Pipe::Ground,
                })
                .collect::<Vec<Pipe>>()
        })
        .collect::<Vec<Vec<Pipe>>>();

    let mut dir = Dir::N;
    let mut steps = 0;
    let mut pos = start_pos;
    let mut curr_pipe: &Pipe = &Pipe::Start;

    let mut border_points: HashMap<XY, u32> = HashMap::new();

    border_points.insert(start_pos, 0);

    while steps == 0 || curr_pipe != &Pipe::Start {
        pos = traverse(&pos, &dir);
        border_points.insert(pos, steps);
        curr_pipe = grid.index(pos.0).index(pos.1);
        dir = match curr_pipe {
            Pipe::Turn(x) => corner(&dir, x),
            _ => dir,
        };
        steps += 1;
    }

    // Your solution here...
    let sol1: u32 = steps / 2;

    let mut winding = 0;
    let mut sol2: i64 = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let cell = border_points.get(&(x, y)).unwrap_or(&0);
            let bottom = border_points.get(&(x, y + 1)).unwrap_or(&0);
            // apply the non-zero winding rule
            if cell - bottom == 1 {
                winding += 1;
            } else if bottom - cell == 1 {
                winding -= 1;
            }

            if cell != &0 {
                print!("{}", winding.to_string().as_str());
            } else if winding == 1 {
                print!("+");
                sol2 += 1;
            } else {
                print!(" ");
            }
        }
        println!();
    }

    (Solution::from(sol1), Solution::from(sol2))
}
