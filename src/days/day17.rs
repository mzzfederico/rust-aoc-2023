use pathfinding::matrix::Matrix;

use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Node = (usize, usize);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct QueueItem {
    node: Node,
    previous_node: Node,
    direction: Direction,
    dir_count: u32,
    heat_lost: u32,
}

impl QueueItem {
    fn new(
        node: Node,
        previous_node: Node,
        heat_lost: u32,
        direction: Direction,
        dir_count: u32,
    ) -> Self {
        Self {
            node,
            previous_node,
            heat_lost,
            dir_count,
            direction,
        }
    }

    fn find_adjacent_items(self, grid: &Matrix<u32>) -> Vec<Self> {
        let mut items: Vec<Self> = vec![];

        let directions = vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];

        for direction in directions {
            if self.node.0 == 0 && direction == Direction::Up {
                continue;
            }

            if self.node.0 == grid.rows - 1 && direction == Direction::Down {
                continue;
            }

            if self.node.1 == 0 && direction == Direction::Left {
                continue;
            }

            if self.node.1 == grid.columns - 1 && direction == Direction::Right {
                continue;
            }

            let next_node = match direction {
                Direction::Up => (self.node.0 - 1, self.node.1),
                Direction::Down => (self.node.0 + 1, self.node.1),
                Direction::Left => (self.node.0, self.node.1 - 1),
                Direction::Right => (self.node.0, self.node.1 + 1),
            };

            let heat_cost = grid[next_node];

            let mut dir_count = self.dir_count;
            if self.direction == direction {
                dir_count += 1;
            } else {
                dir_count = 1;
            }

            if dir_count > 3 {
                continue;
            }

            items.push(QueueItem::new(
                next_node,
                self.node,
                self.heat_lost + heat_cost,
                direction,
                dir_count,
            ));
        }

        items
    }
}

pub fn solve() -> SolutionPair {
    let data = read_to_string("input/days/day17.txt").expect("Can't read input file");
    let grid = Matrix::from_rows(
        data.lines()
            .map(|r| r.chars().map(|c| c.to_digit(10).unwrap())),
    )
    .expect("Error producing matrix from rows");

    let mut unvisited: Vec<QueueItem> = vec![];
    let mut visited: Vec<QueueItem> = vec![];

    unvisited.push(QueueItem::new(
        (0, 0),
        (0, 0),
        grid[(0, 0)],
        Direction::Right,
        1,
    ));

    while !unvisited.is_empty() {
        let current = unvisited.remove(0);

        visited.push(current.clone());
        let adjacent_items = &current.find_adjacent_items(&grid);

        for item in adjacent_items.iter() {
            if visited.iter().any(|v| v.node == item.node) {
                continue;
            }

            if unvisited.iter().any(|v| v.node == item.node) {
                let existing = unvisited.iter_mut().find(|v| v.node == item.node).unwrap();

                if existing.heat_lost > item.heat_lost {
                    existing.heat_lost = item.heat_lost;
                    existing.direction = item.direction;
                    existing.dir_count = item.dir_count;
                    existing.previous_node = item.previous_node;
                }
            } else {
                unvisited.push(item.clone());
            }
        }

        unvisited.sort_by(|a, b| a.heat_lost.cmp(&b.heat_lost));
    }

    let end = visited
        .iter()
        .find(|x| x.node == (grid.rows - 1, grid.columns - 1))
        .unwrap();

    let mut path: Vec<QueueItem> = vec![];
    let mut cur = end;

    while cur.node != (0, 0) {
        path.push(cur.clone());
        let prev = visited
            .iter()
            .find(|v| v.node == cur.previous_node)
            .unwrap();
        cur = prev;
    }

    for r in 0..grid.rows {
        for c in 0..grid.columns {
            if path.iter().any(|p| p.node == (r, c)) {
                let v = path.iter().find(|p| p.node == (r, c)).unwrap();
                print!(
                    "{}",
                    match v.direction {
                        Direction::Up => "^",
                        Direction::Down => "v",
                        Direction::Left => "<",
                        Direction::Right => ">",
                    }
                );
            } else {
                print!("{}", grid[(r, c)]);
            }
        }
        println!();
    }

    path.iter().for_each(|f| println!("{:?}", f));

    let sol1 = end.heat_lost;
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}
