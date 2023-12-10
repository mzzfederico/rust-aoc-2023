use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

enum Pipe {
    Vertical,
    Horizontal,
    Turn((Dir, Dir)),
    Ground,
}

enum Dir {
    N,
    E,
    S,
    W,
}

struct Cursor {
    pos: (u64, u64),
    direction: Dir,
}

pub fn solve() -> SolutionPair {
    let mut start_pos: Cursor = Cursor {
        pos: (0, 0),
        direction: Dir::N,
    };

    let grid = read_to_string("/input/days/day10.txt")
        .expect("Cannot find and split input file")
        .split("\n")
        .enumerate()
        .map(|(r, line)| {
            line.split_ascii_whitespace()
                .enumerate()
                .inspect(|(c, cell)| {
                    if cell.to_owned() == "S" {
                        start_pos.pos = (r.clone() as u64, c.clone() as u64);
                    }
                })
                .map(|t| match t.1 {
                    "|" => Pipe::Vertical,   // is a vertical pipe connecting north and south.
                    "-" => Pipe::Horizontal, // is a horizontal pipe connecting east and west.
                    "L" => Pipe::Turn((Dir::N, Dir::E)), // is a 90-degree bend connecting north and east.
                    "J" => Pipe::Turn((Dir::N, Dir::W)), // is a 90-degree bend connecting north and west.
                    "7" => Pipe::Turn((Dir::S, Dir::W)), // is a 90-degree bend connecting south and west.
                    "F" => Pipe::Turn((Dir::S, Dir::E)), // is a 90-degree bend connecting south and east.
                    "." => Pipe::Ground, // is ground; there is no pipe in this tile.
                    _ => Pipe::Ground,
                })
                .collect::<Vec<Pipe>>()
        })
        .collect::<Vec<Vec<Pipe>>>();

    // Your solution here...
    let sol1: u64 = 0;
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}
