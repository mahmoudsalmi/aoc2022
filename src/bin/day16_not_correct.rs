use std::{collections::HashMap, fmt::Debug, io, str::FromStr, string::ParseError};

const YEAR: &'static str = "2022";
const DAY: &'static str = "16";

type Graph = HashMap<String, Node>;
type Path = Vec<(String, usize, usize, usize)>;
type Rates = HashMap<String, usize>;

#[derive(Clone)]
struct Node {
    id: String,
    rate: usize,
    next: Vec<String>,
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({},{: >4}) -> [{}]",
            self.id,
            self.rate,
            self.next.join(", ")
        )
    }
}

impl FromStr for Node {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .replace("Valve ", "")
            .replace(" has flow rate=", "|")
            .replace("; tunnels lead to valves ", "|")
            .replace("; tunnel leads to valve ", "|");

        let mut tokens: Vec<&str> = s.split("|").collect();
        let next: Vec<String> = tokens
            .pop()
            .unwrap()
            .split(", ")
            .map(|t| t.to_string())
            .collect();

        let rate: usize = tokens.pop().unwrap().parse().unwrap();

        Ok(Self {
            id: tokens[0].into(),
            rate,
            next,
        })
    }
}

fn possible_max(remain_time: usize, rates: &Rates) -> usize {
    let mut res = 0;
    let mut values: Vec<usize> = rates.values().map(|v| v.clone()).collect();
    values.sort_by(|a, b| b.cmp(a));
    let mut remain_time = remain_time.clone();

    while remain_time > 2 && !values.is_empty() {
        res += remain_time * values.pop().unwrap();
        remain_time -= 2;
    }

    res
}

fn find_path(
    graph: &Graph,
    path: Path,
    rates: Rates,
    time: usize,
    global_pressure: usize,
    current_max: &mut isize,
) -> (Path, usize) {
    if time == 0 || rates.is_empty() {
        return (path.clone(), global_pressure);
    }

    let last_node = path.last().unwrap().clone().0;
    let next_nodes = graph
        .get(&last_node)
        .map(|node| node.next.clone())
        .unwrap_or(vec![]);

    let mut result: (Path, usize) = (path.clone(), global_pressure);
    for next_node in next_nodes {
        let mut path = path.clone();
        let time = time.clone() - 1;
        let new_global_pressure = global_pressure;

        path.push((next_node.clone(), 0, new_global_pressure, time));

        if (possible_max(time, &rates) + new_global_pressure) as isize >= *current_max - 1 {
            if new_global_pressure as isize > *current_max {
                *current_max = new_global_pressure as isize;
            }

            let next_pressure = find_path(
                graph,
                path.clone(),
                rates.clone(),
                time,
                new_global_pressure,
                current_max,
            );

            result = if result.1 > next_pressure.1 {
                result
            } else {
                next_pressure
            };
        }

        let mut rates = rates.clone();
        if rates.contains_key(&next_node) && time > 0 {
            let time = time - 1;
            let pressure = rates.remove(&next_node).unwrap();
            let new_global_pressure = global_pressure + (pressure * time);

            let (el, _, _, _) = path.pop().unwrap();
            path.push((el, pressure, new_global_pressure, time));

            if (possible_max(time, &rates) + new_global_pressure) as isize >= *current_max - 1 {
                if new_global_pressure as isize > *current_max {
                    *current_max = new_global_pressure as isize;
                }

                let next_pressure =
                    find_path(graph, path, rates, time, new_global_pressure, current_max);

                result = if result.1 > next_pressure.1 {
                    result
                } else {
                    next_pressure
                };
            }
        }
    }
    result
}

fn part1(data: &str) -> usize {
    let graph: Graph =
        data.lines()
            .map(|s| s.parse::<Node>().unwrap())
            .fold(Graph::new(), |mut graph, node| {
                graph.insert(node.id.clone(), node);
                graph
            });

    let rates: HashMap<String, usize> = graph
        .clone()
        .iter_mut()
        .filter(|(_, node)| node.rate > 0)
        .map(|(id, node)| (id.clone(), node.rate))
        .collect();

    println!("{}", possible_max(30, &rates));

    let mut current_max: isize = 0;

    let res = find_path(
        &graph,
        vec![("AA".to_owned(), 0, 0, 30)],
        rates,
        30,
        0,
        &mut current_max,
    );

    println!("{:?}", res);

    res.1
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

    let test_data = include_str!("day16.test");
    println!("Test :: Part 1 ====>     {}", part1(&test_data));
    println!("Test :: Part 2 ====>     {}", part2(&test_data));
    println!("--------------------------------------------------------");

    let input_data = include_str!("day16.in");
    println!("Input:: Part 1 ====>     {}", part1(&input_data));
    println!("Input:: Part 2 ====>     {}", part2(&input_data));
    println!("--------------------------------------------------------");

    Ok(())
}
