use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input_file = read_to_string("input/days/day06.txt").expect("Cannot read the input file");

    let data = input_file
        .split("\n")
        .into_iter()
        .map(|x| {
            return x
                .to_string()
                .split(":")
                .nth(1)
                .expect("Issue splitting up input values")
                .split_ascii_whitespace()
                .into_iter()
                .map(|x| x.parse::<u64>().expect("Issue parsing nums"))
                .collect::<Vec<u64>>();
        })
        .collect::<Vec<Vec<u64>>>();

    let times: &Vec<u64> = &data[0];
    let distances: &Vec<u64> = &data[1];

    let mut sol1: u64 = 1;
    for (total_time, min_distance) in times.iter().zip(distances.iter()) {
        let mut winning_strategies = 0;
        for button_pressing_ms in 1..(total_time - 1) {
            let remaining_time_ms = total_time - button_pressing_ms;
            let total_movement = remaining_time_ms * button_pressing_ms;

            if total_movement > *min_distance {
                winning_strategies += 1;
            }
        }
        sol1 *= winning_strategies;
    }

    let mut sol2: u64 = 0;
    let long_time: u64 = times
        .iter()
        .map(|x| x.to_string())
        .collect::<String>()
        .parse::<u64>()
        .expect("Error during parsing long_time");
    let long_distance: u64 = distances
        .iter()
        .map(|x| x.to_string())
        .collect::<String>()
        .parse::<u64>()
        .expect("Error during parsing long_distance");

    println!("{}\n{}", long_time, long_distance);

    for button_pressing_ms in 1..long_time {
        let remaining_time_ms = long_time - button_pressing_ms;
        let total_movement = remaining_time_ms * button_pressing_ms;

        if total_movement > long_distance {
            sol2 += 1;
        }
    }

    (Solution::from(sol1), Solution::from(sol2))
}
