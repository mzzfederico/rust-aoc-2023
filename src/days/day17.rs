use pathfinding::{directed::dijkstra::dijkstra, matrix::Matrix};

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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct QueueItem {
    node: Node,
    direction: Direction,
    dir_count: u32,
}

impl QueueItem {
    fn new(node: Node, direction: Direction, dir_count: u32) -> Self {
        Self {
            node,
            direction,
            dir_count,
        }
    }

    fn find_adjacent_items(self, grid: &Matrix<u32>, min: u32, max: u32) -> Vec<(Self, u32)> {
        let mut items: Vec<(Self, u32)> = vec![];

        if self.dir_count < min {
            let dir_count = self.dir_count + 1;

            if self.node.0 == 0 && self.direction == Direction::Up
                || self.node.0 == grid.rows - 1 && self.direction == Direction::Down
                || self.node.1 == 0 && self.direction == Direction::Left
                || self.node.1 == grid.columns - 1 && self.direction == Direction::Right
            {
                return items;
            } else {
                let next_node = match self.direction {
                    Direction::Up => (self.node.0 - 1, self.node.1),
                    Direction::Down => (self.node.0 + 1, self.node.1),
                    Direction::Left => (self.node.0, self.node.1 - 1),
                    Direction::Right => (self.node.0, self.node.1 + 1),
                };

                items.push((
                    QueueItem::new(next_node, self.direction, dir_count),
                    grid[next_node],
                ));

                return items;
            }
        }

        let directions = vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];

        for direction in directions {
            if self.direction == Direction::Left && direction == Direction::Right {
                continue;
            }
            if self.direction == Direction::Up && direction == Direction::Down {
                continue;
            }
            if self.direction == Direction::Left && direction == Direction::Right {
                continue;
            }
            if self.direction == Direction::Right && direction == Direction::Left {
                continue;
            }

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

            let mut dir_count = self.dir_count;
            if self.direction == direction {
                dir_count += 1;
            } else {
                dir_count = 1;
            }

            if dir_count > max {
                continue;
            }

            items.push((
                QueueItem::new(next_node, direction, dir_count),
                grid[next_node],
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

    /* let mut unvisited: Vec<QueueItem> = vec![];
    let mut visited: Vec<QueueItem> = vec![];

    unvisited.push(QueueItem::new((0, 0), (0, 0), 0, Direction::Left, 0));

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
    } */

    let min_dist = dijkstra(
        &QueueItem::new((0, 0), Direction::Down, 1),
        |p| p.find_adjacent_items(&grid, 1, 3),
        |p| p.node == (grid.rows - 1, grid.columns - 1),
    )
    .unwrap();

    let path = min_dist.0.iter().map(|i| i.node).collect::<Vec<Node>>();

    for r in 0..grid.rows {
        for c in 0..grid.columns {
            if path.contains(&(r, c)) {
                print!("O");
            } else {
                print!("+");
            }
        }
        println!();
    }

    // ????????
    let sol1 = min_dist.1 + 1;

    let min_dist_2 = dijkstra(
        &QueueItem::new((0, 0), Direction::Down, 5),
        |p| p.find_adjacent_items(&grid, 4, 10),
        |p| p.node == (grid.rows - 1, grid.columns - 1),
    )
    .unwrap();

    let path = min_dist_2.0.iter().map(|i| i.node).collect::<Vec<Node>>();

    for r in 0..grid.rows {
        for c in 0..grid.columns {
            if path.contains(&(r, c)) {
                print!("O");
            } else {
                print!("+");
            }
        }
        println!();
    }

    let sol2 = min_dist_2.1;

    (Solution::from(sol1), Solution::from(sol2))
}
