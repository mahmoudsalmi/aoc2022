use std::{fmt::Debug, io, str::FromStr, string::ParseError, collections::VecDeque};

const YEAR: &'static str = "2022";
const DAY: &'static str = "12";

#[derive(Debug, Clone, Copy, PartialEq)]
struct Cell(usize, usize);

impl Cell {
    fn cell_arround(&self, height: usize, width: usize) -> Vec<Cell> {
        let mut res: Vec<Cell> = vec![];
        if self.0 > 0 { res.push(Cell(self.0 - 1, self.1)); }
        if self.1 > 0 { res.push(Cell(self.0, self.1 - 1)); }
        if self.0 + 1 < height { res.push(Cell(self.0 + 1, self.1));} 
        if self.1 + 1 < width { res.push(Cell(self.0, self.1 + 1));} 
        res 
    }
}

fn draw_grid(
    grid: &Vec<Vec<isize>>,
    height: usize,
    width: usize,
    title: &str,
) -> String {

    let mut res: String = String::new();

    let line_width = 4 * width + 2 + 3;
    let header_trait_width = line_width 
        - title.len() 
        - (3 * 2) /* height & width */ 
        - 4 /* space for title */
        - 8 /* start trait */; 

    res += &format!("┌");
    for _ in 0..8 {
        res += &format!( "─");
    }
    res += &format!( "  {}{: >3}x{: <3} ", title, width, height);
    for _ in 0..header_trait_width {
        res += &format!( "─");
    }
    res += &format!("┐\n");

    for r in 0..height {
        res += &format!( "│  ");
        for c in 0..width {
            res += &format!( "{: >4}", grid[r][c]);
        }
        res += &format!("   │\n");
    }

    res += &format!( "└");
    for _ in 0..line_width {
        res += &format!( "─");
    }
    res += &format!("┘\n");

    res
}

struct Grid {
    width: usize,
    height: usize,

    grid: Vec<Vec<isize>>,

    start_cell: Cell,
    end_cell: Cell,
}

impl Grid {
    fn get_value(&self, Cell(r, c): Cell) -> isize {
        self.grid[r][c].clone()
    }

    fn get_mut_value(&mut self, Cell(r, c): Cell) -> &mut isize {
        self.grid.get_mut(r).unwrap().get_mut(c).unwrap()
    }

    fn next_cells(&self, cell: Cell) -> Vec<Cell> {
        let max_value: isize = match self.get_value(cell) {
            -1 => 1,
            v => v + 1,
        };

        cell
            .cell_arround(self.height, self.width)
            .into_iter()
            .filter(|c| *c != self.start_cell && self.get_value(*c) <= max_value)
            .collect()
    }

    fn init_weigths_grid(&self) -> Grid {
        let width: usize = self.width; 
        let height: usize = self.height;
        Grid { 
            width, 
            height, 
            grid: vec![vec![0; width]; height], 
            start_cell: self.start_cell.clone(), 
            end_cell: self.end_cell.clone()
        }
    }
}

fn navigate(grid: &Grid, weights: &mut Grid, cell: Cell, curr_weight: usize) -> (Vec<Cell>, bool) {
    let next_cells = grid.next_cells(cell);
    let curr_weight: isize = curr_weight as isize;
    let curr_value: isize = grid.get_value(cell);
    
    if let Some(end_cell) = next_cells.iter().find(|c| **c == grid.end_cell) {
        if curr_value == 26 {
            let end_weight = weights.get_mut_value(*end_cell);
            *end_weight = curr_weight + 1;

            return (vec![*end_cell], true);
        }
    }

    let mut res: Vec<Cell> = vec![];
    for i in 0..next_cells.len() {
        let next_cell = next_cells[i];
        let next_weight = weights.get_mut_value(next_cell);
        if *next_weight == 0 || *next_weight > curr_weight + 1 {
            *next_weight = curr_weight + 1;
            res.push(next_cell);
        }
    }
    (res, false)
}

fn navigate_to_end(grid: &Grid, weights: &mut Grid, starts: Vec<Cell>) -> isize {

    let mut paths: VecDeque<Vec<Cell>> = starts.iter().map(|c| vec![*c]).collect();

    loop {
        let mut new_paths: VecDeque<Vec<Cell>> = VecDeque::new();
        let mut ended = false;

        while paths.len() > 0 {
            let mut old_path = paths.pop_front().unwrap();

            let top_cell = old_path.pop().unwrap();
            let (next_cells, end_reached) = navigate(&grid, weights, top_cell, old_path.len());

            if end_reached {
                let mut end_path = old_path.clone();
                end_path.push(top_cell.clone());
                end_path.push(grid.end_cell.clone());
                new_paths = VecDeque::from(vec![end_path]);
                ended = true;
                break;
            } 

            if next_cells.len() != 0 {
                for next_cell in next_cells {
                    let mut new_path = old_path.clone();
                    new_path.push(top_cell.clone());
                    new_path.push(next_cell);
                    new_paths.push_back(new_path);
                }
            }

        }
        
        paths = new_paths;

        if ended {
            // println!("{:?}", weights);
            break;
        }    
    }

    weights.get_value(grid.end_cell)
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", draw_grid(&self.grid, self.height, self.width, "Grid"))
    }
}

impl FromStr for Grid {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();

        let height = lines.len();
        let width = lines.get(0).expect("EMPTY GRID!").len();

        let mut grid = vec![vec![0; width]; height];
        let mut start_cell = Cell(0, 0);
        let mut end_cell = Cell(0, 0);

        lines.iter().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                let idx: isize = match c {
                    'S' => {
                        start_cell = Cell(y, x);
                        -1
                    }
                    'E' => {
                        end_cell = Cell(y, x);
                        -2
                    }
                    alpha => alpha as isize - 'a' as isize + 1,
                };
                let cell = grid.get_mut(y).unwrap().get_mut(x).unwrap();
                *cell = idx;
            });
        });

        Ok(Grid {
            width,
            height,
            grid,
            start_cell,
            end_cell,
        })
    }
}

fn part1(data: &str) -> isize {
    let grid: Grid = data.parse().unwrap();
    let weights: &mut Grid = &mut grid.init_weigths_grid();
    // println!("{:?}", grid);
    navigate_to_end(&grid, weights, vec![grid.start_cell.clone()])
}

fn part2(data: &str) -> isize {
    let grid: Grid = data.parse().unwrap();
    let weights: &mut Grid = &mut grid.init_weigths_grid();
    // println!("{:?}", grid);
    let mut starts = Vec::<Cell>::new();
    for r in 0..grid.height {
        for c in 0..grid.width {
            let cell = Cell(r, c);
            if grid.get_value(cell) == 1 {
                starts.push(cell);
            }
        }
    }
    navigate_to_end(&grid, weights, starts)
}

fn main() -> io::Result<()> {
    println!();
    println!(
        "---( AOC{} - Day {} )-----------------------[Rust]----",
        YEAR, DAY
    );

    let test_data = include_str!("day12.test");
    println!("Test :: Part 1 ====>     {}", part1(&test_data));
    println!("Test :: Part 2 ====>     {}", part2(&test_data));
    println!("--------------------------------------------------------");

    let input_data = include_str!("day12.in");
    println!("Input:: Part 1 ====>     {}", part1(&input_data));
    println!("Input:: Part 2 ====>     {}", part2(&input_data));
    println!("--------------------------------------------------------");

    Ok(())
}
