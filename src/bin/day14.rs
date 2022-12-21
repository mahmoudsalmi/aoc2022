use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    io,
    str::FromStr,
    string::ParseError,
};

const YEAR: &'static str = "2022";
const DAY: &'static str = "14";

type Lines = HashMap<usize, HashSet<Range>>;

#[derive(Clone)]
enum LineType {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn get_by_line_type(&self, line_type: LineType) -> usize {
        match line_type {
            LineType::Vertical => self.x,
            LineType::Horizontal => self.y,
        }
    }

    fn get_oposite_line_type(&self, line_type: LineType) -> usize {
        match line_type {
            LineType::Vertical => self.y,
            LineType::Horizontal => self.x,
        }
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split(",").collect();
        Ok(Point {
            x: parts.get(0).unwrap().parse().unwrap(),
            y: parts.get(1).unwrap().parse().unwrap(),
        })
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Range {
    min: usize,
    max: usize,
}

impl Range {
    fn new(a: usize, b: usize) -> Self {
        let min = a.min(b);
        let max = a.max(b);
        Self { min, max }
    }

    fn contains(&self, n: usize) -> bool {
        n >= self.min && n <= self.max
    }
}

impl Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.min, self.max)
    }
}

#[derive(Debug)]
struct Path {
    vertical_lines: Lines,
    horizontal_lines: Lines,
    sands: HashSet<Point>,
    deepest_line: usize,
    with_floor: bool,
}

impl Path {
    fn build_lines(edges: &Vec<Point>, line_type: LineType) -> (usize, Lines) {
        edges
            .windows(2)
            .map(|windows| (&windows[0], &windows[1]))
            .filter(|(a, b)| {
                a.get_by_line_type(line_type.clone()) == b.get_by_line_type(line_type.clone())
            })
            .map(|(a, b)| {
                (
                    a.get_by_line_type(line_type.clone()),
                    Range::new(
                        a.get_oposite_line_type(line_type.clone()),
                        b.get_oposite_line_type(line_type.clone()),
                    ),
                )
            })
            .fold(
                (0, HashMap::<usize, HashSet<Range>>::new()),
                |(max, mut map), (idx, range)| {
                    map.entry(idx)
                        .and_modify(|set| {
                            set.insert(range.clone());
                        })
                        .or_insert(HashSet::from([range]));
                    (idx.max(max), map)
                },
            )
    }

    fn new() -> Self {
        Self {
            vertical_lines: HashMap::new(),
            horizontal_lines: HashMap::new(),
            sands: HashSet::new(),
            deepest_line: 0,
            with_floor: false,
        }
    }

    fn extend(&mut self, other: &Self) {
        other.vertical_lines.iter().for_each(|(y, ranges)| {
            let res_ranges = self.vertical_lines.entry(*y).or_insert(HashSet::new());
            ranges.iter().for_each(|range| {
                res_ranges.insert(range.clone());
            });
        });
        other.horizontal_lines.iter().for_each(|(y, ranges)| {
            let res_ranges = self.horizontal_lines.entry(*y).or_insert(HashSet::new());
            ranges.iter().for_each(|range| {
                res_ranges.insert(range.clone());
            });
        });
        self.deepest_line = self.deepest_line.max(other.deepest_line);
    }

    fn point_is_empty(&self, point: &Point) -> bool {
        if self.sands.contains(point) {
            false
        } else {
            let vertical_value = point.get_by_line_type(LineType::Vertical);
            let horizontal_value = point.get_by_line_type(LineType::Horizontal);

            let vertical_contains = self.vertical_lines.contains_key(&vertical_value)
                && self
                    .vertical_lines
                    .get(&vertical_value)
                    .unwrap()
                    .iter()
                    .any(|range| range.contains(horizontal_value));

            let horizontal_contains = self.horizontal_lines.contains_key(&horizontal_value)
                && self
                    .horizontal_lines
                    .get(&horizontal_value)
                    .unwrap()
                    .iter()
                    .any(|range| range.contains(vertical_value));

            !vertical_contains && !horizontal_contains
        }
    }

    fn next_point(&self, point: &Point) -> Option<Point> {
        if self.sands.contains(&point) {
            None
        } else if self.with_floor && point.y >= self.deepest_line + 1 {
            None
        } else if !self.with_floor && point.y >= self.deepest_line {
            None
        } else {
            let Point { x, y } = point;

            [
                Point::new(*x, *y + 1),
                Point::new(x - 1, y + 1),
                Point::new(x + 1, y + 1),
            ]
            .iter()
            .map(|p| *p)
            .find(|next_point| self.point_is_empty(next_point))
        }
    }

    fn add_point_rest(&mut self, point: &Point) -> Option<Point> {
        let mut next_point = point.clone();

        loop {
            match self.next_point(&next_point) {
                Some(p) => {
                    next_point = p;
                }
                None => {
                    break;
                }
            }
        }

        if !self.with_floor && next_point.y >= self.deepest_line {
            None
        } else if self.sands.contains(&next_point) {
            None
        } else {
            self.sands.insert(next_point.clone());
            Some(next_point)
        }
    }
}

impl FromStr for Path {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let edges = s
            .trim()
            .split(" -> ")
            .map(|s| s.parse::<Point>().unwrap())
            .collect::<Vec<Point>>();

        let (_, vertical_lines) = Path::build_lines(&edges, LineType::Vertical);
        let (deepest_line, horizontal_lines) = Path::build_lines(&edges, LineType::Horizontal);

        Ok(Path {
            vertical_lines,
            horizontal_lines,
            deepest_line,
            sands: HashSet::new(),
            with_floor: false,
        })
    }
}

fn part1(data: &str) -> usize {
    let mut all_path = Path::new();

    data.lines()
        .map(|s| s.parse::<Path>().unwrap())
        .for_each(|path| all_path.extend(&path));

    let start_sand = Point::new(500, 0);
    while all_path.add_point_rest(&start_sand) != None {}

    all_path.sands.len()
}

fn part2(data: &str) -> usize {
    let mut all_path = Path::new();

    data.lines()
        .map(|s| s.parse::<Path>().unwrap())
        .for_each(|path| all_path.extend(&path));
    all_path.with_floor = true;

    let start_sand = Point::new(500, 0);
    while let Some(_) = all_path.add_point_rest(&start_sand) {}

    all_path.sands.len()
}

fn main() -> io::Result<()> {
    println!();
    println!(
        "---( AOC{} - Day {} )-----------------------[Rust]----",
        YEAR, DAY
    );

    let test_data = include_str!("day14.test");
    println!("Test :: Part 1 ====>     {}", part1(&test_data));
    println!("Test :: Part 2 ====>     {}", part2(&test_data));
    println!("--------------------------------------------------------");

    let input_data = include_str!("day14.in");
    println!("Input:: Part 1 ====>     {}", part1(&input_data));
    println!("Input:: Part 2 ====>     {}", part2(&input_data));
    println!("--------------------------------------------------------");

    Ok(())
}
