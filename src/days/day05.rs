use crate::{Solution, SolutionPair};

use rayon::prelude::*;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////
type DestinationRangeStart = i64;
type SourceRangeStart = i64;
type Span = i64;
type SeedNum = i64;

/* type Range = (i64, i64);
type RangeTransform = (Vec<Range>, Vec<Range>); */

#[derive(Debug, Clone, Copy)]
struct Map {
    destination: DestinationRangeStart,
    source: SourceRangeStart,
    span: Span,
}

impl Map {
    fn transform(self, input_number: i64) -> Option<i64> {
        let max_source = self.source + (self.span - 1);
        if input_number >= self.source && input_number <= max_source {
            let offset = input_number - self.source;
            let result = self.destination + offset;
            Some(result)
        } else {
            None
        }
    }

    /* fn cut(self, r_start: i64, r_end: i64) -> RangeTransform {
        let (m_start, m_end): Range = (self.source, self.source + self.span);
        let delta = self.destination - self.source;

        if r_end < m_start || r_start > m_end {
            return (vec![(r_start, r_end)], vec![]);
        } else if r_start >= m_start && r_end <= m_end {
            return (vec![], vec![(r_start + delta, r_end + delta)]);
        } else if r_start < m_start && r_end > m_end {
            return (
                vec![(r_start, m_start - 1), (m_end + 1, r_end)],
                vec![(m_start + delta, m_end + delta)],
            );
        } else if r_start < m_start && r_end < m_end {
            return (
                vec![(r_start, m_start - 1)],
                vec![(m_start + delta, r_end + delta)],
            );
        } else if r_start < m_start && r_end < m_end {
            return (
                vec![(m_end + 1, r_end)],
                vec![(r_start + delta, m_end + delta)],
            );
        } else {
            return (vec![(r_start, r_end)], vec![]);
        }
    } */
}

type Layer = Vec<Map>;

#[derive(Debug, Clone)]
struct Almanac {
    seeds: Vec<SeedNum>,
    layers: Vec<Layer>,
}

impl Almanac {
    fn from_file(input_file: &str) -> Self {
        let lines = input_file.split('\n').enumerate();

        let mut seeds: Vec<SeedNum> = vec![];
        let mut layers: Vec<Vec<Map>> = vec![];

        let mut map_buffer: Vec<Map> = vec![];

        for (index, line) in lines {
            if index == 0 {
                let seeds_desc = line.split(": ").collect::<Vec<&str>>()[1];

                seeds = seeds_desc
                    .split_ascii_whitespace()
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>();
            } else {
                if line.contains("map") {
                    if map_buffer.len() > 0 {
                        layers.push(map_buffer.clone());
                        map_buffer.clear();
                    }
                } else {
                    let nums = line
                        .split_ascii_whitespace()
                        .map(|i| i.parse::<i64>().unwrap_or_default())
                        .collect::<Vec<i64>>();

                    if nums.len() > 0 {
                        let destination = nums[0];
                        let source = nums[1];
                        let span = nums[2];

                        map_buffer.push(Map {
                            destination,
                            source,
                            span,
                        });
                    }
                }
            }
        }
        layers.push(map_buffer.clone());
        map_buffer.clear();

        Self { seeds, layers }
    }
}

fn calculate_seed_location(almanac: &Almanac, seed: i64) -> i64 {
    let mut result: i64 = seed;
    for maps in almanac.layers.iter() {
        for map in maps {
            let found = map.transform(result);
            if found.is_some() {
                result = found.unwrap();
                break;
            }
        }
    }

    result
}

/* fn calculate_range_transformation(layer: &Layer, range: &Range) -> Vec<Range> {
    let mut source_ranges: Vec<Range> = vec![(range.0, range.1)];
    let mut result_ranges: Vec<Range> = vec![];
    for map in layer {
        let (unchanged, transformed) = map.cut(range.0, range.1);

        for range in unchanged {
            source_ranges.push(range);
        }

        for range in transformed {
            result_ranges.push(range);
        }
    }

    return vec![source_ranges, result_ranges].concat();
} */

pub fn solve() -> SolutionPair {
    let file_pos = "input/days/day05.txt";
    let file = read_to_string(file_pos).expect("Could not read input");
    let almanac = Almanac::from_file(file.as_str());

    let mut locations: Vec<i64> = vec![];

    let seeds = &almanac.seeds.clone();
    for seed in seeds {
        let result = calculate_seed_location(&almanac, *seed);
        locations.push(result);
    }

    let sol1: i64 = *locations.iter().min().unwrap();

    let tuple_seeds = seeds
        .chunks(2)
        .map(|pair| (pair[0], pair[1]))
        .collect::<Vec<(i64, i64)>>();

    let operations: Vec<Vec<i64>> = tuple_seeds
        .par_iter()
        .map(|range| {
            let range_se = (range.0, range.0 + range.1);
            let seeds: Vec<i64> = (range_se.0..range_se.1).collect();
            let locations = seeds
                .par_iter()
                .map(|s| calculate_seed_location(&almanac, *s))
                .collect::<Vec<i64>>();

            locations
        })
        .collect::<Vec<_>>();

    let sol2: i64 = *operations.iter().flatten().min().unwrap();

    (Solution::from(sol1), Solution::from(sol2))
}
