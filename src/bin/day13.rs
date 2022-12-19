use std::{cmp::Ordering, fmt::Debug, io, num::ParseIntError, str::FromStr};

const YEAR: &'static str = "2022";
const DAY: &'static str = "13";

#[derive(Clone, Eq, PartialEq)]
struct Node {
    is_number: bool,
    childs: Vec<Node>,
    value: Option<usize>,
}

impl Node {
    fn add_child(&mut self, child: Node) {
        self.childs.push(child)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.is_number, other.is_number) {
            (true, true) => self.value.cmp(&other.value),
            (false, false) => {
                let mut it = self.childs.iter();
                let mut other_it = other.childs.iter();

                let mut child = it.next();
                let mut other_child = other_it.next();

                'cmp: loop {
                    match (child, other_child) {
                        (None, None) => {
                            break 'cmp;
                        }
                        (None, Some(_)) => {
                            return Ordering::Less;
                        }
                        (Some(_), None) => {
                            return Ordering::Greater;
                        }
                        (Some(v), Some(ov)) => {
                            let cmp = v.cmp(ov);
                            if cmp != Ordering::Equal {
                                return cmp;
                            }
                            child = it.next();
                            other_child = other_it.next();
                        }
                    }
                }
                Ordering::Equal
            }
            (true, false) => Node {
                is_number: false,
                value: None,
                childs: vec![self.clone()],
            }
            .cmp(other),
            (false, true) => self.cmp(&Node {
                is_number: false,
                value: None,
                childs: vec![other.clone()],
            }),
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.is_number {
            true => {
                write!(f, "{}", self.value.unwrap())
            }
            false => {
                write!(f, "{:?}", self.childs)
            }
        }
    }
}

impl FromStr for Node {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parents: Vec<Node> = vec![];
        let mut it = s.chars().into_iter();

        let mut c = it.next();
        let mut old_c = ' ';
        let mut curr_node = Node {
            is_number: false,
            childs: vec![],
            value: None,
        };
        let mut digits = String::new();

        while c != None {
            match c.unwrap() {
                '[' => {
                    curr_node.is_number = false;
                    parents.push(curr_node);
                    curr_node = Node {
                        is_number: false,
                        childs: vec![],
                        value: None,
                    };
                }
                ']' => {
                    if digits.len() > 0 {
                        curr_node.is_number = true;
                        curr_node.value = Some(digits.clone().parse::<usize>().unwrap());
                        digits.clear();
                    } else {
                        curr_node.is_number = false;
                    }

                    let mut parent = parents.pop().expect("No Parent !!");
                    if old_c != '[' {
                        parent.add_child(curr_node);
                    }
                    curr_node = parent;
                }
                ',' => {
                    if digits.len() > 0 {
                        curr_node.is_number = true;
                        curr_node.value = Some(digits.clone().parse::<usize>().unwrap());
                        digits.clear();
                    } else {
                        curr_node.is_number = false;
                    }

                    let mut parent = parents.pop().expect("No Parent !!");
                    parent.add_child(curr_node);
                    parents.push(parent);
                    curr_node = Node {
                        is_number: false,
                        childs: vec![],
                        value: None,
                    };
                }
                d => {
                    digits.push(d);
                }
            }
            old_c = c.unwrap();
            c = it.next();
        }

        Ok(curr_node)
    }
}

fn part1(data: &str) -> usize {
    let line_nodes: Vec<Node> = data
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.parse::<Node>().unwrap())
        .collect();

    let sets: Vec<(&Node, &Node)> = line_nodes
        .chunks(2)
        .map(move |chunks| (chunks.get(0).unwrap(), chunks.get(1).unwrap()))
        .collect();

    sets.iter()
        .enumerate()
        .filter(|(_, (left, right))| left < right)
        .map(|(idx, _)| idx + 1)
        .sum()
}

fn part2(data: &str) -> usize {
    let mut line_nodes: Vec<Node> = data
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.parse::<Node>().unwrap())
        .collect();

    line_nodes.push("[[2]]".parse().unwrap());
    line_nodes.push("[[6]]".parse().unwrap());
    line_nodes.sort();

    line_nodes
        .iter()
        .enumerate()
        .filter(|(_, node)| {
            let str = format!("{:?}", node);
            str == "[[2]]" || str == "[[6]]"
        })
        .map(|(idx, _)| idx + 1)
        .fold(1, |a, b| a*b)

}

fn main() -> io::Result<()> {
    println!();
    println!(
        "---( AOC{} - Day {} )-----------------------[Rust]----",
        YEAR, DAY
    );

    let test_data = include_str!("day13.test");
    println!("Test :: Part 1 ====>     {}", part1(&test_data));
    println!("Test :: Part 2 ====>     {}", part2(&test_data));
    println!("--------------------------------------------------------");

    let input_data = include_str!("day13.in");
    println!("Input:: Part 1 ====>     {}", part1(&input_data));
    println!("Input:: Part 2 ====>     {}", part2(&input_data));
    println!("--------------------------------------------------------");

    Ok(())
}
