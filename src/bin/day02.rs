use std::io::Result;

const YEAR: &'static str = "2022";
const DAY: &'static str = "02";

#[derive(PartialEq, Debug, Clone, Copy)]
enum Shape {
    Rock,
    Scissor,
    Paper,
}
use Shape::*;

impl Shape {
    fn score(self) -> u64 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissor => 3,
        }
    }

    fn from_abc(shape: char) -> Shape {
        match shape {
            'A' => Rock,
            'B' => Paper,
            'C' => Scissor,
            _ => unreachable!("Shape")
        }
    }

    fn from_xyz(shape: char) -> Shape {
        match shape {
            'X' => Rock,
            'Y' => Paper,
            'Z' => Scissor,
            _ => unreachable!("Shape")
        }
    }

    fn from_res(oponent: &Shape, res: &RoundResult) -> Shape {
        match *res {
            Draw => oponent.clone(),
            Win => match oponent {
                Rock => Paper,
                Paper => Scissor,
                Scissor => Rock,
            },
            Lose => match oponent {
                Rock => Scissor,
                Paper => Rock,
                Scissor => Paper,
            },
        }
    }
}

#[derive(PartialEq, Debug)]
enum RoundResult {
    Win,
    Draw,
    Lose,
}

use RoundResult::*;

impl RoundResult {
    fn from_code(code: char) -> RoundResult {
        match code {
            'X' => Lose,
            'Y' => Draw,
            'Z' => Win,
            _ => panic!("Unknown code!"),
        }
    }

    fn from_shapes(oponent_shape: &Shape, user_shape: &Shape) -> RoundResult {
        match (*user_shape, *oponent_shape) {
            (u, o) if o == u => Draw,
            (Rock, Scissor) | (Scissor, Paper) | (Paper, Rock) => Win,
            _ => Lose,
        }
    }

    fn score(&self) -> u64 {
        match *self {
            Win => 6,
            Draw => 3,
            Lose => 0,
        }
    }
}

#[derive(Debug)]
struct Round {
    user: Shape,
    res: RoundResult,
}

impl Round {
    fn from_line_1(line: &str) -> Round {
        let error_msg = format!("Line ({}) invalid!", line);

        let opponent_code: char = line.chars().nth(0).expect(&error_msg);
        let user_code: char = line.chars().nth(2).expect(&error_msg);

        let opponent = Shape::from_abc(opponent_code);
        let user = Shape::from_xyz(user_code);
        let res = RoundResult::from_shapes(&opponent, &user);

        Round { user, res }
    }

    fn from_line_2(line: &str) -> Round {
        let error_msg = format!("Line ({}) invalid!", line);

        let opponent_code: char = line.chars().nth(0).expect(&error_msg);
        let result_code: char = line.chars().nth(2).expect(&error_msg);

        let opponent = Shape::from_abc(opponent_code);
        let res = RoundResult::from_code(result_code);
        let user = Shape::from_res(&opponent, &res);

        Round { user, res }
    }

    fn score(&self) -> u64 {
        self.res.score() + self.user.score()
    }
}

fn part1(data: &str) -> u64 {
    data.lines()
        .map(|line| Round::from_line_1(line))
        .map(|round| round.score())
        .sum()
}

fn part2(data: &str) -> u64 {
    data.lines()
        .map(|line| Round::from_line_2(line))
        .map(|round| round.score())
        .sum()
}

fn main() -> Result<()> {
    println!();
    println!("---( AOC{} - Day {} )-----------------------[Rust]----", YEAR, DAY);

    let test_data = include_str!("day02.test");
    println!("Test :: Part 1 ====>     {:?}", part1(&test_data));
    println!("Test :: Part 2 ====>     {:?}", part2(&test_data));
    println!("--------------------------------------------------------");
    

    let input_data = include_str!("day02.in");
    println!("Input:: Part 1 ====>     {:?}", part1(&input_data));
    println!("Input:: Part 2 ====>     {:?}", part2(&input_data));
    println!("--------------------------------------------------------");

    Ok(())
}
