use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

#[derive(Eq, PartialEq, Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn match_str(color: &str) -> Self {
        //println!("{:?}", color);
        match color {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            _ => Color::Red,
        }
    }
}

#[derive(Debug)]
struct Pull {
    color: Color,
    value: u32,
}

impl Pull {
    fn new(color: Color, value: u32) -> Self {
        Self { color, value }
    }
}

#[derive(Debug)]
struct Game {
    pulls: Vec<Pull>,
}

impl Game {
    fn new(pulls: Vec<Pull>) -> Self {
        Self { pulls }
    }
    fn is_possible(&self, red: &u32, green: &u32, blue: &u32) -> bool {
        for pull in &self.pulls {
            if pull.color == Color::Red && &pull.value > red {
                return false;
            }
            if pull.color == Color::Green && &pull.value > green {
                return false;
            }
            if pull.color == Color::Blue && &pull.value > blue {
                return false;
            }
        }
        true
    }
    fn min_cubes(&self) -> (u32, u32, u32) {
        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;
        for pull in &self.pulls {
            if pull.color == Color::Red && pull.value > red {
                red = pull.value;
            }
            if pull.color == Color::Green && pull.value > green {
                green = pull.value;
            }
            if pull.color == Color::Blue && pull.value > blue {
                blue = pull.value;
            }
        }

        (red, green, blue)
    }
    fn power(&self) -> u32 {
        let min_cubes: (u32, u32, u32) = self.min_cubes();

        min_cubes.0 * min_cubes.1 * min_cubes.2
    }
}

pub fn solve() -> SolutionPair {
    // Your solution here...

    let input_games_txt = read_to_string("input/days/day02.txt").expect("Could not read day02.txt");

    let game_lines = input_games_txt.split('\n');
    let mut games: Vec<Game> = vec![];
    for line in game_lines {
        let slices = line.split(':').collect::<Vec<&str>>();
        let single_pulls = slices[1].strip_prefix(' ').unwrap().split("; ");

        let mut game = Game::new(vec![]);
        for pull_str in single_pulls {
            for set_str in pull_str.split(", ") {
                let pull_slices = set_str.split(' ').collect::<Vec<&str>>();
                let num_str: u32 = pull_slices[0].parse::<u32>().unwrap_or_default();
                let color_str: &str = pull_slices[1];
                game.pulls
                    .push(Pull::new(Color::match_str(color_str), num_str));
            }
        }
        games.push(game);
    }

    let mut sol1: usize = 0;
    let mut sol2: u32 = 0;
    let red: u32 = 12;
    let green: u32 = 13;
    let blue: u32 = 14;
    for (index, game) in games.into_iter().enumerate() {
        let id = index + 1;
        if game.is_possible(&red, &green, &blue) {
            sol1 += id
        }

        let pow = game.power();
        sol2 += pow;
    }

    (Solution::from(sol1), Solution::from(sol2))
}
