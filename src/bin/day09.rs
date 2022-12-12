use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Debug, Display},
    io::Result,
};

const YEAR: &'static str = "2022";
const DAY: &'static str = "09";

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    y: isize,
    x: isize,
}

impl Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Position {
    fn init() -> Position {
        Position { y: 0, x: 0 }
    }

    fn next_position(&self, direction: &str) -> Position {
        match direction {
            "U" => Position {
                y: self.y + 1,
                x: self.x,
            },
            "D" => Position {
                y: self.y - 1,
                x: self.x,
            },
            "R" => Position {
                y: self.y,
                x: self.x + 1,
            },
            "L" => Position {
                y: self.y,
                x: self.x - 1,
            },
            _ => unreachable!("UNKNOWN DIRECTION!!"),
        }
    }

    fn distance(self, other: Position) -> Position {
        Position {
            y: self.y - other.y,
            x: self.x - other.x,
        }
    }
}

struct Rope {
    size: usize,
    knots: HashMap<usize, Position>,
    tail_positions: HashSet<Position>,
}

impl Rope {
    fn apply(&mut self, direction: &str, steps: isize) {
        (0..steps).for_each(|_step| {
            let head = self
                .knots
                .get(&0)
                .expect("NO HEAD!")
                .clone()
                .next_position(direction);
            self.knots.insert(0, head);

            (0..self.size)
                .into_iter()
                .collect::<Vec<usize>>()
                .windows(2)
                .for_each(|window| {
                    let head = self.knots.get(&window[0]).expect("NO HEAD!").clone();

                    let t = window[1];
                    let mut tail = self.knots.get(&t).expect("NO TAIL!!").clone();

                    let distance = head.distance(tail);

                    if distance.x.abs() > 1 && distance.y.abs() > 1 {
                        tail = Position {
                            x: head.x - (distance.x.signum()),
                            y: head.y - (distance.y.signum()),
                        };
                    } else {
                        if distance.x.abs() > 1 {
                            tail = Position {
                                x: head.x - (distance.x.signum()),
                                y: head.y,
                            };
                        }
                        if distance.y.abs() > 1 {
                            tail = Position {
                                x: head.x,
                                y: head.y - (distance.y.signum()),
                            };
                        }
                    }

                    self.knots.insert(t, tail);
                    if t == self.size - 1 {
                        self.tail_positions.insert(tail.clone());
                    }
                });

            // println!(
            //     "{}*{} -- {} -- {:?}",
            //     direction,
            //     steps,
            //     step,
            //     (0..self.size)
            //         .map(|i| self.knots.get(&i).unwrap())
            //         .collect::<Vec<_>>()
            // );
        });
        // println!();
    }
}

impl Rope {
    fn init(size: usize) -> Self {
        Self {
            size,
            knots: (0..size).map(|idx| (idx, Position::init())).collect(),
            tail_positions: HashSet::from([Position::init()]),
        }
    }
}

fn part1(data: &str) -> usize {
    let mut rope = Rope::init(2);

    data.lines()
        .map(|line| {
            let line_parts = line.split(" ").collect::<Vec<&str>>();
            (line_parts[0], line_parts[1].parse::<isize>().unwrap())
        })
        .for_each(|(direction, steps)| {
            rope.apply(direction, steps);
        });

    rope.tail_positions.len()
}

fn part2(data: &str) -> usize {
    let mut rope = Rope::init(10);

    data.lines()
        .map(|line| {
            let line_parts = line.split(" ").collect::<Vec<&str>>();
            (line_parts[0], line_parts[1].parse::<isize>().unwrap())
        })
        .for_each(|(direction, steps)| {
            rope.apply(direction, steps);
        });

    rope.tail_positions.len()
}

fn main() -> Result<()> {
    println!();
    println!(
        "---( AOC{} - Day {} )-----------------------[Rust]----",
        YEAR, DAY
    );

    let test_data = include_str!("day09.test");
    println!("Test :: Part 1 ====>     {}", part1(&test_data));
    println!("Test :: Part 2 ====>     {}", part2(&test_data));
    println!("--------------------------------------------------------");

    let test_data2 = include_str!("day09.test2");
    println!("Test2:: Part 1 ====>     {}", part1(&test_data2));
    println!("Test2:: Part 2 ====>     {}", part2(&test_data2));
    println!("--------------------------------------------------------");

    let input_data = include_str!("day09.in");
    println!("Input:: Part 1 ====>     {}", part1(&input_data));
    println!("Input:: Part 2 ====>     {}", part2(&input_data));
    println!("--------------------------------------------------------");

    Ok(())
}
