use pathfinding::matrix::Matrix;

use crate::{Solution, SolutionPair};
use std::{collections::HashMap, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

// row, column
type Pos = (i64, i64);

#[derive(Clone, Copy, PartialEq, Debug, Eq, Hash)]
enum Dir {
    U,
    L,
    D,
    R,
}

#[derive(Clone, Copy, PartialEq, Debug, Eq, Hash)]
struct Photon {
    pos: Pos,
    dir: Dir,
}

impl Photon {
    fn new(pos: Pos, dir: Dir) -> Self {
        Photon { pos, dir }
    }

    fn next(&self) -> Pos {
        match self.dir {
            Dir::U => (self.pos.0 - 1, self.pos.1),
            Dir::D => (self.pos.0 + 1, self.pos.1),
            Dir::L => (self.pos.0, self.pos.1 - 1),
            Dir::R => (self.pos.0, self.pos.1 + 1),
        }
    }

    fn move_on(&mut self, grid: &Matrix<&u8>) -> Vec<Photon> {
        let next = &self.next();

        if grid.within_bounds((next.0 as usize, next.1 as usize)) {
            let c = *grid[(next.0 as usize, next.1 as usize)];

            let split_ver = b'|';
            let split_hor = b'-';

            if (self.dir == Dir::L || self.dir == Dir::R) && c == split_ver {
                return vec![
                    Self::new(*next, Dir::U),
                    Self::new(*next, Dir::D),
                ];
            }

            if (self.dir == Dir::U || self.dir == Dir::D) && c == split_hor {
                return vec![
                    Self::new(*next, Dir::L),
                    Self::new(*next, Dir::R),
                ];
            }

            return vec![Self::new(*next, derive_turn(self.dir, c))];
        }

        vec![]
    }
}

fn derive_turn(initial_dir: Dir, turn_char: u8) -> Dir {
    let turn_left = b'\\';
    let turn_right = b'/';

    if turn_char == turn_left {
        match initial_dir {
            Dir::U => Dir::L,
            Dir::R => Dir::D,
            Dir::D => Dir::R,
            Dir::L => Dir::U,
        }
    } else if turn_char == turn_right {
        match initial_dir {
            Dir::U => Dir::R,
            Dir::R => Dir::U,
            Dir::D => Dir::L,
            Dir::L => Dir::D,
        }
    } else {
        initial_dir
    }
}

fn get_energized_cells(grid: &Matrix<&u8>, first_photon: Photon) -> u64 {
    let mut cells: HashMap<(i64, i64), Dir> = HashMap::new();
    let mut photons: Vec<Photon> = vec![first_photon];

    while !photons.is_empty() {
        // replace with new photons
        photons = photons
            .iter_mut()
            .flat_map(|p| p.move_on(grid))
            .filter(|p| {
                // filter out photons that are on the same position
                match cells.get(&p.pos) {
                    Some(dir) => dir != &p.dir,
                    None => true,
                }
            })
            .collect::<Vec<Photon>>();

        // save locations
        photons.iter().for_each(|p| {
            cells.insert(p.pos, p.dir);
        });
    }

    cells.len() as u64
}

pub fn solve() -> SolutionPair {
    // Your solution here...

    let data = read_to_string("input/days/day16.txt").expect("Can't read input file");
    let grid = Matrix::from_rows(data.lines().map(|r| r.as_bytes()))
        .expect("Error producing matrix from rows");

    let sol1: u64 = get_energized_cells(&grid, Photon::new((0, -1), Dir::R));

    let mut sol2: u64 = 0;

    for row in 0..grid.rows {
        sol2 = sol2.max(get_energized_cells(
            &grid,
            Photon::new((row as i64, -1), Dir::R),
        ));

        sol2 = sol2.max(get_energized_cells(
            &grid,
            Photon::new((row as i64, grid.columns as i64), Dir::L),
        ));
    }

    for column in 0..grid.columns {
        sol2 = sol2.max(get_energized_cells(
            &grid,
            Photon::new((-1, column as i64), Dir::D),
        ));

        sol2 = sol2.max(get_energized_cells(
            &grid,
            Photon::new((grid.rows as i64, column as i64), Dir::U),
        ));
    }

    (Solution::from(sol1), Solution::from(sol2))
}
