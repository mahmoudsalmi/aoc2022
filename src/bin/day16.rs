use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    io,
    str::FromStr,
    string::ParseError,
};

const YEAR: &'static str = "2022";
const DAY: &'static str = "16";

type Rates = HashMap<String, usize>;
type Cache = HashMap<String, usize>;

#[derive(Clone)]
struct Node {
    id: String,
    rate: usize,
    next: Vec<String>,
}

struct Graph(HashMap<String, Node>);

impl Graph {
    fn new() -> Graph {
        Graph(HashMap::new())
    }

    fn next(&self, el: &str) -> Vec<String> {
        self.0
            .get(el)
            .map(|node| node.next.iter().map(|n| n.clone()).collect::<Vec<String>>())
            .unwrap_or(vec![])
    }

    fn init_rates(&self) -> Rates {
        self.0
            .values()
            .map(|node| (node.id.clone(), node.rate.clone()))
            .filter(|(_, rate)| *rate > 0)
            .collect()
    }
}

impl FromIterator<Node> for Graph {
    fn from_iter<T: IntoIterator<Item = Node>>(iter: T) -> Self {
        iter.into_iter().fold(Graph::new(), |mut graph, node| {
            graph.0.insert(node.id.clone(), node);
            graph
        })
    }
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

fn build_key(curr_node: &str, time: &usize, opened: &HashSet<String>, elephant_num: &usize) -> String {
    let mut opened: Vec<String> = opened.iter().map(|n| n.clone()).collect();
    opened.sort();

    [
        elephant_num.to_string(),
        curr_node.clone().to_owned(),
        time.to_string(),
        opened.join(","),
    ]
    .join("-")
}

fn find_path(
    graph: &Graph,
    curr_node: &str,
    rates: &Rates,
    opened: HashSet<String>,
    time: usize,
    cache: &mut Cache,
    elephant_num: &usize,
) -> usize {
    let key = build_key(curr_node, &time, &opened, elephant_num);
    if cache.contains_key(&key) {
        return cache.get(&key).unwrap().clone();
    }

    if time == 0 {
        return if *elephant_num == 1 {
            0
        } else {
            find_path(
                graph,
                "AA",
                rates,
                opened.clone(),
                26,
                cache,
                &(elephant_num - 1),
            )
        };
    }

    let mut max_presure = graph
        .next(curr_node)
        .iter()
        .map(|next_node| {
            find_path(
                graph,
                &next_node,
                rates,
                opened.clone(),
                time - 1,
                cache,
                elephant_num,
            )
        })
        .max()
        .unwrap();

    if time >= 2 {
        max_presure = max_presure.max(
            graph
                .next(curr_node)
                .iter()
                .filter(|next_node| {
                    !opened.contains(next_node.clone()) && rates.contains_key(next_node.clone())
                })
                .map(|next_node| {
                    let pressure = rates.get(next_node).unwrap();

                    let mut opened: HashSet<String> = opened.clone();
                    opened.insert(next_node.clone());

                    (pressure * (time - 2))
                        + find_path(
                            graph,
                            &next_node,
                            rates,
                            opened,
                            time - 2,
                            cache,
                            elephant_num,
                        )
                })
                .max()
                .unwrap_or(0),
        );
    }

    cache.entry(key).or_insert(max_presure.clone());
    max_presure
}

fn part1(data: &str) -> usize {
    let graph: Graph = data
        .lines()
        .map(|line| line.parse::<Node>().unwrap())
        .collect();

    find_path(
        &graph,
        "AA",
        &graph.init_rates(),
        HashSet::new(),
        30,
        &mut Cache::new(),
        &1,
    )
}

fn part2(data: &str) -> usize {
    let graph: Graph = data
        .lines()
        .map(|line| line.parse::<Node>().unwrap())
        .collect();

    find_path(
        &graph,
        "AA",
        &graph.init_rates(),
        HashSet::new(),
        26,
        &mut Cache::new(),
        &2,
    )
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
