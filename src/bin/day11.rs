use std::{collections::HashMap, io, num::ParseIntError, str::FromStr, string::ParseError};

const YEAR: &'static str = "2022";
const DAY: &'static str = "11";

#[derive(Debug)]
enum OperationMemberType {
    Number(usize),
    Old,
}
use OperationMemberType::*;

impl FromStr for OperationMemberType {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "old" => Old,
            num => Number(num.parse().unwrap()),
        })
    }
}

#[derive(Debug)]
enum OperationType {
    Add,
    Multiply,
    Divisible,
}
use OperationType::*;

impl FromStr for OperationType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => Add,
            "*" => Multiply,
            _ => unreachable!("Unknown OperationType : {}", s),
        })
    }
}

#[derive(Debug)]
struct Operation {
    op: OperationType,
    left: OperationMemberType,
    right: OperationMemberType,
}

impl Operation {
    fn divisible(s: &str) -> Self {
        Self {
            op: Divisible,
            left: Old,
            right: Number(s.parse().unwrap()),
        }
    }

    fn exec(&self, item: usize) -> usize {
        let right = match self.right {
            Old => item,
            Number(n) => n,
        };
        let left = match self.left {
            Old => item,
            Number(n) => n,
        };

        match self.op {
            Add => left + right,
            Multiply => left * right,
            _ => unreachable!("Not executable op : {:?}!", self.op),
        }
    }

    fn check(&self, item: usize) -> bool {
        let right = match self.right {
            Old => item,
            Number(n) => n,
        };
        let left = match self.left {
            Old => item,
            Number(n) => n,
        };
        match self.op {
            Divisible => left % right == 0,
            _ => unreachable!("Not checkable op : {:?}!", self.op),
        }
    }
}

impl FromStr for Operation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elements = s.split(" ").collect::<Vec<&str>>();
        Ok(Operation {
            op: elements[1].parse().unwrap(),
            left: elements[0].parse().unwrap(),
            right: elements[2].parse().unwrap(),
        })
    }
}

struct Monkey {
    _id: usize,
    items: Vec<usize>,
    operation: Operation,
    test: Operation,
    true_dest: usize,
    false_dest: usize,
    items_inspected_count: usize,
}
impl Monkey {
    fn play(&mut self, divider: usize) -> HashMap<usize, Vec<usize>> {
        let mut res = HashMap::<usize, Vec<usize>>::new();
        self.items_inspected_count += self.items.len();
        self.items.iter().for_each(|item| {
            let op_res = self.operation.exec(*item);
            let op_res: usize = op_res / divider;

            let check_res = self.test.check(op_res);
            let dest_key = if check_res {
                self.true_dest
            } else {
                self.false_dest
            };
            res.entry(dest_key).or_insert(vec![]).push(op_res);
        });
        res
    }
}

impl FromStr for Monkey {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<&str>>();

        let id = lines[0]
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        let items = lines[1]
            .split(":")
            .map(|part| part.trim())
            .skip(1)
            .take(1)
            .flat_map(|part| part.split(","))
            .map(|str| str.trim().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let operation = lines[2]
            .split("=")
            .map(|part| part.trim())
            .skip(1)
            .take(1)
            .collect::<Vec<&str>>()
            .get(0)
            .unwrap()
            .parse()
            .unwrap();
        let test = Operation::divisible(
            lines[3]
                .chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .as_str(),
        );

        let true_dest = lines[4]
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        let false_dest = lines[5]
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        Ok(Monkey {
            _id: id,
            items,
            operation,
            test,
            true_dest,
            false_dest,
            items_inspected_count: 0,
        })
    }
}

fn part1(data: &str) -> usize {
    let mut monkeys = data
        .split("\n\n")
        .map(|monkey_str| monkey_str.parse::<Monkey>().unwrap())
        .collect::<Vec<Monkey>>();

    (0..20).for_each(|_| {
        (0..monkeys.len()).for_each(|i| {
            let monkey = monkeys.get_mut(i).unwrap();
            let res = monkey.play(3);
            monkey.items.clear();
            res.iter().for_each(|(key, value)| {
                let monkey_dest = monkeys.get_mut(*key).unwrap();
                value.iter().for_each(|item| {
                    monkey_dest.items.push(*item);
                });
            });
        });
    });

    let mut counts = monkeys
        .iter()
        .map(|monkey| monkey.items_inspected_count.clone())
        .collect::<Vec<usize>>();

    counts.sort_by(|a, b| b.cmp(a));
    
    counts.iter().take(2).into_iter().fold(1, |a, b| a * b)
}

fn part2(_data: &str) -> &str {
    "NOT IMPLEMENTED!"
}

fn main() -> io::Result<()> {
    println!();
    println!(
        "---( AOC{} - Day {} )-----------------------[Rust]----",
        YEAR, DAY
    );

    let test_data = include_str!("day11.test");
    println!("Test :: Part 1 ====>     {}", part1(&test_data));
    println!("Test :: Part 2 ====>     {}", part2(&test_data));
    println!("--------------------------------------------------------");

    let input_data = include_str!("day11.in");
    println!("Input:: Part 1 ====>     {}", part1(&input_data));
    println!("Input:: Part 2 ====>     {}", part2(&input_data));
    println!("--------------------------------------------------------");

    Ok(())
}
