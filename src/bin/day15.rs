use std::{collections::HashSet, fmt::Debug, io, str::FromStr, string::ParseError};

const YEAR: &'static str = "2022";
const DAY: &'static str = "01";

#[derive(Clone, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn distance(&self, point: &Point) -> isize {
        (self.x - point.x).abs() + (self.y - point.y).abs()
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Clone)]
struct Sensor {
    position: Point,
    first_beacon: Point,
    min_distance: isize,
}

impl Sensor {
    fn distance(&self, point: &Point) -> isize {
        self.position.distance(point)
    }

    fn is_in_no_beacon_zone(&self, point: &Point) -> bool {
        self.min_distance >= self.distance(point)
    }
}

impl Debug for Sensor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{:?} -> {:?} | {}]",
            self.position, self.first_beacon, self.min_distance
        )
    }
}

impl FromStr for Sensor {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line = s
            .replace("Sensor at x=", "")
            .replace(" y=", "")
            .replace(": closest beacon is at x=", ",");

        let numbers = line
            .split(",")
            .map(|n| n.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();

        let position = Point {
            x: numbers[0],
            y: numbers[1],
        };

        let first_beacon = Point {
            x: numbers[2],
            y: numbers[3],
        };

        let min_distance = first_beacon.distance(&position);

        Ok(Self {
            position,
            first_beacon,
            min_distance,
        })
    }
}

#[derive(Debug)]
struct Puzzle {
    sensors: Vec<Sensor>,
    left_top: Point,
    right_buttom: Point,
    _positions: HashSet<Point>,
    _beacons: HashSet<Point>,
}

impl Puzzle {
    fn from(sensors: Vec<Sensor>) -> Self {
        let mut sensors = sensors;
        let mut puzzle = Self::init(sensors.pop().unwrap());

        while let Some(sensor) = sensors.pop() {
            puzzle.add_sensor(sensor);
        }

        puzzle
    }

    fn init(sensor: Sensor) -> Self {
        Puzzle {
            sensors: vec![sensor.clone()],
            left_top: Point {
                x: sensor.position.x - sensor.min_distance,
                y: sensor.position.y - sensor.min_distance,
            },
            right_buttom: Point {
                x: sensor.position.x + sensor.min_distance,
                y: sensor.position.y + sensor.min_distance,
            },
            _positions: HashSet::from([sensor.position]),
            _beacons: HashSet::from([sensor.first_beacon]),
        }
    }

    fn add_sensor(&mut self, sensor: Sensor) {
        self.left_top = Point {
            x: self.left_top.x.min(sensor.position.x - sensor.min_distance),
            y: self.left_top.y.min(sensor.position.y - sensor.min_distance),
        };

        self.right_buttom = Point {
            x: self
                .right_buttom
                .x
                .max(sensor.position.x + sensor.min_distance),
            y: self
                .right_buttom
                .y
                .max(sensor.position.y + sensor.min_distance),
        };

        self.sensors.push(sensor.clone());
        self._positions.insert(sensor.position);
        self._beacons.insert(sensor.first_beacon);
    }

    fn is_in_no_beacon_zone(&self, point: &Point) -> bool {
        for sensor in &self.sensors {
            if sensor.is_in_no_beacon_zone(&point) {
                return true;
            }
        }

        false
    }
}

fn part1(data: &str, y: isize) -> usize {
    let sensors = data
        .lines()
        .map(|line| line.parse::<Sensor>().unwrap())
        .collect::<Vec<Sensor>>();
    let puzzle = Puzzle::from(sensors);

    let points = (puzzle.left_top.x..=puzzle.right_buttom.x)
        .map(|x| Point { x, y })
        .filter(|point| !puzzle._beacons.contains(point) && puzzle.is_in_no_beacon_zone(point))
        .collect::<Vec<Point>>();

    points.len()
}

fn part2(data: &str, max: isize) -> isize {
    let sensors = data
        .lines()
        .map(|line| line.parse::<Sensor>().unwrap())
        .collect::<Vec<Sensor>>();
    let puzzle = Puzzle::from(sensors);

    // println!("{:#?}", puzzle);

    for x in 0..=max {
        for y in 0..=max {
            let point = Point {x,y};
            if !puzzle.is_in_no_beacon_zone(&point) {
                println!("{:?}", point);
                return point.x * 4000000 + point.y;
            }
        }
    }

    0
}

fn main() -> io::Result<()> {
    println!();
    println!(
        "---( AOC{} - Day {} )-----------------------[Rust]----",
        YEAR, DAY
    );

    let test_data = include_str!("day15.test");
    println!("Test :: Part 1 ====>     {}", part1(&test_data, 10));
    println!("Test :: Part 2 ====>     {}", part2(&test_data, 20));
    println!("--------------------------------------------------------");

    let input_data = include_str!("day15.in");
    println!("Input:: Part 1 ====>     {}", part1(&input_data, 2000000));
    println!("Input:: Part 2 ====>     {}", part2(&input_data, 4000000));
    println!("--------------------------------------------------------");

    Ok(())
}
