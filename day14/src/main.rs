fn main() {
    let input = include_str!("../input.txt");
    part1(input); // 113078
    part2(input); // 94255
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Round,
    Square,
    Empty,
}

fn parse(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Square,
                    'O' => Tile::Round,
                    _ => unreachable!()
                })
                .collect()
        })
        .collect()
}

fn slide_north(grid: &mut Vec<Vec<Tile>>) {
    for col in 0..grid[0].len() {
        let mut empty_or_round_row = 0;
        for row in 0..grid.len() {
            let curr = grid[row][col];
            match curr {
                Tile::Square => empty_or_round_row = row + 1,
                Tile::Round => {
                    let replace_with = std::mem::replace(&mut grid[empty_or_round_row][col], curr);
                    let _ = std::mem::replace(&mut grid[row][col], replace_with);
                    empty_or_round_row += 1;
                }
                Tile::Empty => (),
            }
        }
    }
}

fn weight(grid: &Vec<Vec<Tile>>) -> usize {
    grid.iter()
        .rev()
        .enumerate()
        .map(|(i, row)| {
            let round_rocks = row.iter().filter(|tile| **tile == Tile::Round).count();
            round_rocks * (i + 1)
        })
        .sum()
}

fn clockwise(grid: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let size = grid.len();
    let mut rotated = vec![vec![Tile::Empty; size]; size];
    for row in 0..size {
        for col in 0..size {
            rotated[col][size - 1 - row] = grid[row][col];
        }
    }
    rotated
}

fn cycle(mut grid: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    for _ in 0..4 {
        slide_north(&mut grid);
        let rotated = clockwise(&grid);
        grid = rotated;
    }
    grid
}

pub fn part1(input: &str) {
    let mut grid = parse(input);
    slide_north(&mut grid);
    let ouput = weight(&grid);
    println!("Output: {}", ouput);
}

pub fn part2(input: &str) {
    let mut grid = parse(input);
    let mut seen = vec![grid.clone()];

    loop {
        grid = cycle(grid);
        if let Some(idx) = seen.iter().position(|x| x == &grid) {
            let cycle_len = seen.len() - idx;
            let final_idx = idx + (1_000_000_000 - idx) % cycle_len;
            let output = weight(&seen[final_idx]);
            println!("Output: {}", output);
            break;
        }
        seen.push(grid.clone());
    }
}