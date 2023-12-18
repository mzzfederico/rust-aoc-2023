use crate::{Solution, SolutionPair};
use itertools::Itertools;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

enum Dir {
    U,
    R,
    D,
    L,
}

pub fn solve() -> SolutionPair {
    let input_string = read_to_string("input/days/day18.txt").expect("Cannot read input file");

    let orders = input_string.lines().map(|l| {
        let slices = l.split_ascii_whitespace().collect_vec();
        let d = &slices[0];
        let m = &slices[1];
        let c = &slices[2][2..8];

        (
            match *d {
                "U" => Dir::U,
                "R" => Dir::R,
                "D" => Dir::D,
                "L" => Dir::L,
                _ => Dir::U,
            },
            m.parse::<i64>().unwrap_or(0),
            c,
        )
    });

    let mut cursor = (0, 0);
    let mut perimeter = 0;
    let mut area = 0;

    for (d, m, _) in orders.clone() {
        let order = match d {
            Dir::U => (cursor.0, cursor.1 - m),
            Dir::D => (cursor.0, cursor.1 + m),
            Dir::L => (cursor.0 - m, cursor.1),
            Dir::R => (cursor.0 + m, cursor.1),
        };
        area += (cursor.0 * order.1) - (cursor.1 * order.0);
        perimeter += m;
        cursor = order;
    }

    let sol1 = (area.abs() / 2) + (perimeter / 2) + 1;

    let mut cursor = (0, 0);
    let mut perimeter = 0;
    let mut area: i64 = 0;

    for (_, _, c) in orders {
        let d = match &c[5..6] {
            "3" => Dir::U,
            "0" => Dir::R,
            "1" => Dir::D,
            "2" => Dir::L,
            _ => Dir::U,
        };

        let m = i64::from_str_radix(&c[0..5], 16).unwrap();

        let order = match d {
            Dir::U => (cursor.0, cursor.1 - m),
            Dir::D => (cursor.0, cursor.1 + m),
            Dir::L => (cursor.0 - m, cursor.1),
            Dir::R => (cursor.0 + m, cursor.1),
        };
        area += (cursor.0 * order.1) - (cursor.1 * order.0);
        perimeter += m;
        cursor = order;
    }

    let sol2 = (area.abs() / 2) + (perimeter / 2) + 1;

    (Solution::from(sol1), Solution::from(sol2))
}
