use std::{collections::HashSet, io::Result};

const YEAR: &'static str = "2022";
const DAY: &'static str = "08";

fn ranges(rows_len: usize, columns_len: usize, reverse: bool) -> (Vec<usize>, Vec<usize>) {
    if reverse {
        (
            (0..rows_len).collect::<Vec<usize>>(),
            (0..columns_len).collect::<Vec<usize>>(),
        )
    } else {
        (
            (0..rows_len).rev().collect::<Vec<usize>>(),
            (0..columns_len).rev().collect::<Vec<usize>>(),
        )
    }
}

fn part1(data: &str) -> usize {
    let digits: Vec<Vec<i8>> = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect()
        })
        .collect();

    let rows_len: usize = digits.len();
    let columns_len: usize = digits[1].len();

    let rows_colums: Vec<Vec<(usize, usize, i8)>> = [
        ranges(rows_len, columns_len, false),
        ranges(rows_len, columns_len, true),
    ]
    .iter()
    .map(|(rows, columns)| {
        rows.iter()
            .map(|r| {
                columns
                    .iter()
                    .map(|c| (r.to_owned(), c.to_owned(), digits[*r][*c]))
                    .collect::<Vec<(usize, usize, i8)>>()
            })
            .collect::<Vec<Vec<(usize, usize, i8)>>>()
    })
    .flatten()
    .collect();

    let columns_rows: Vec<Vec<(usize, usize, i8)>> = [
        ranges(rows_len, columns_len, false),
        ranges(rows_len, columns_len, true),
    ]
    .iter()
    .map(|(rows, columns)| {
        columns
            .iter()
            .map(|c| {
                rows.iter()
                    .map(|r| (r.to_owned(), c.to_owned(), digits[*r][*c]))
                    .collect::<Vec<(usize, usize, i8)>>()
            })
            .collect::<Vec<Vec<(usize, usize, i8)>>>()
    })
    .flatten()
    .collect();

    let global_set = [rows_colums, columns_rows]
        .iter()
        .flatten()
        .map(|segment| {
            let (_max, set): (i8, HashSet<(usize, usize)>) = segment.iter().fold(
                (-1_i8, HashSet::<(usize, usize)>::new()),
                move |mut res, &point| {
                    if point.2 > res.0 {
                        res.1.insert((point.0, point.1));
                        (point.2, res.1)
                    } else {
                        res
                    }
                },
            );

            set
        })
        .flatten()
        .collect::<HashSet<(usize, usize)>>();


    global_set.len()
}

fn part2(_data: &str) -> String {
    "NOT IMPLEMENTED!!!".to_owned()
}

fn main() -> Result<()> {
    println!();
    println!(
        "---( AOC{} - Day {} )-----------------------[Rust]----",
        YEAR, DAY
    );

    let test_data = include_str!("day08.test");
    println!("Test :: Part 1 ====>     {}", part1(&test_data));
    println!("Test :: Part 2 ====>     {}", part2(&test_data));
    println!("--------------------------------------------------------");

    let input_data = include_str!("day08.in");
    println!("Input:: Part 1 ====>     {}", part1(&input_data));
    println!("Input:: Part 2 ====>     {}", part2(&input_data));
    println!("--------------------------------------------------------");

    Ok(())
}
