use std::io::Result;

const YEAR: &'static str = "2022";
const DAY: &'static str = "09";

#[derive(Debug)]
struct Cpu {
    x_register: isize,
    cycles: Vec<isize>,
}

impl Cpu {
    fn init() -> Cpu {
        Cpu {
            x_register: 1,
            cycles: vec![1],
        }
    }

    fn apply(&mut self, instruction: &str) {
        match instruction {
            "noop" => {
                self.cycles.push(self.x_register);
            }
            add if add.starts_with("addx") => {
                let num = add
                    .split(" ")
                    .collect::<Vec<&str>>()
                    .get(1)
                    .unwrap()
                    .parse::<isize>()
                    .unwrap();

                self.cycles.push(self.x_register);
                self.x_register += num;
                self.cycles.push(self.x_register);
            }
            _ => unreachable!("UNKNOWN INSTRUICTION!!"),
        }
    }

    fn signal_strenghts(&self, cycle: usize) -> isize {
        cycle as isize * self.cycles[cycle - 1]
    }

    fn pixel_state(&self, r: isize, c: isize) -> char {
        let cycle_value: isize = *self.cycles.get((r * 40 + c) as usize).unwrap() - 1;

        if c >= cycle_value && c <= cycle_value + 2  {
            '#'
        } else {
            '.'
        }
    }
}

fn part1(data: &str) -> isize {
    let mut cpu = Cpu::init();
    data.lines().for_each(|line| cpu.apply(line));

    [20, 60, 100, 140, 180, 220]
        .iter()
        .map(|cycle| cpu.signal_strenghts(*cycle))
        .sum()
}

fn part2(data: &str) {
    let mut cpu = Cpu::init();
    data.lines().for_each(|line| cpu.apply(line));

    println!();
    (0..6).for_each(|r| {
        (0..40).for_each(|c| {
            print!("{}", cpu.pixel_state(r, c));
        });
        println!();
    });
    println!();
}

fn main() -> Result<()> {
    println!();
    println!(
        "---( AOC{} - Day {} )-----------------------[Rust]----",
        YEAR, DAY
    );

    let test_data = include_str!("day10.test");
    println!("Test :: Part 1 ====>     {}", part1(&test_data));
    println!("Test :: Part 2 ====>     ");
    part2(&test_data);
    println!("--------------------------------------------------------");

    let input_data = include_str!("day10.in");
    println!("Input:: Part 1 ====>     {}", part1(&input_data));
    println!("Input:: Part 2 ====>     ");
    part2(&input_data);
    println!("--------------------------------------------------------");

    Ok(())
}
