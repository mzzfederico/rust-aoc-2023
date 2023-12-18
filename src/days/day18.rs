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

fn area_from_orders(orders: Vec<(Dir, i64)>) -> i64 {
    let mut cursor = (0, 0);
    let mut perimeter = 0;
    let mut area = 0;

    for (d, m) in orders {
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

    return (area.abs() / 2) + (perimeter / 2) + 1;
}

fn color_to_orders(color: &str) -> (Dir, i64) {
    (
        match &color[5..6] {
            "3" => Dir::U,
            "0" => Dir::R,
            "1" => Dir::D,
            "2" => Dir::L,
            _ => Dir::U,
        },
        i64::from_str_radix(&color[0..5], 16).unwrap(),
    )
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

    let sol1 = area_from_orders(orders.clone().map(|(d, m, _)| (d, m)).collect_vec());
    let sol2 = area_from_orders(orders.map(|(_, _, c)| color_to_orders(c)).collect_vec());

    (Solution::from(sol1), Solution::from(sol2))
}
