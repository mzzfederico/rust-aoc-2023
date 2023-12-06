use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
struct Pos {
    x: u32,
    y: u32,
}

impl Pos {
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone)]
struct Number {
    value: u32,
    pos: Vec<Pos>,
}

#[derive(Debug, Clone)]
struct Symbol {
    character: char,
    pos: Pos,
    nums_close: Vec<Number>,
}

impl Symbol {
    fn new(character: char, pos: Pos) -> Self {
        Self {
            character,
            pos,
            nums_close: Vec::new(),
        }
    }
}

pub fn solve() -> SolutionPair {
    let input: String = read_to_string("input/days/day03.txt")
        .expect("Could not read input")
        .replace('\r', "");

    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().chars().count();

    let mut lines = input.lines();
    let mut nums: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    let mut char_buf: Vec<char> = Vec::new();
    let mut curr_num: Number = Number {
        value: 0,
        pos: Vec::new(),
    };

    for i in 0..rows {
        let mut row = lines.next().unwrap().chars();
        for j in 0..cols {
            if let Some(c) = row.next() {
                if c.is_numeric() {
                    char_buf.push(c);
                    curr_num.pos.push(Pos::new(i as u32, j as u32));
                } else {
                    if !char_buf.is_empty() {
                        let value = char_buf.iter().collect::<String>().parse::<u32>().unwrap();
                        curr_num.value = value;
                        nums.push(curr_num);
                        char_buf.clear();
                        curr_num = Number {
                            value: 0,
                            pos: Vec::new(),
                        };
                    }
                    if c != '.' {
                        symbols.push(Symbol::new(c, Pos::new(i as u32, j as u32)));
                    }
                }
            }
        }
    }

    let mut parts: Vec<Number> = Vec::new();

    for s in symbols.iter_mut() {
        s.nums_close = nums
            .iter()
            .filter(|n| {
                n.pos
                    .iter()
                    .any(|p| p.x.abs_diff(s.pos.x) <= 1 && p.y.abs_diff(s.pos.y) <= 1)
            }).cloned()
            .collect::<Vec<Number>>();

        for n in s.nums_close.iter() {
            parts.push(n.clone());
        }
    }

    let sol1: u32 = parts.iter().map(|n| n.value).sum();

    let gears = symbols
        .iter()
        .filter(|s| s.character == '*' && s.nums_close.len() == 2)
        .cloned()
        .collect::<Vec<Symbol>>();

    let sol2: u64 = gears
        .iter()
        .map(|g| g.nums_close.iter().map(|n| n.value as u64).product::<u64>())
        .sum();

    (Solution::from(sol1), Solution::from(sol2))
}
