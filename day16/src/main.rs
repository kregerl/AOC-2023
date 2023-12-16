use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");
    part1(input); // 7074
    part2(input); // 7530
}

fn part1(input: &str) {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start_point = (0, 0, 'r');

    let output = traverse(start_point, &mut grid);
    println!("Output: {}", output);
}

fn part2(input: &str) {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut max_count = 0;
    for y in 0..grid.len() {
        let count_visited = traverse((0, y, 'r'), &mut grid);
        if count_visited > max_count {
            max_count = count_visited;
        }
        let beam = (grid[0].len() - 1, y, 'l');
        let count_visited = traverse(beam, &mut grid);
        if count_visited > max_count {
            max_count = count_visited;
        }
    }
    for x in 0..grid[0].len() {
        let count_visited = traverse((x, 0, 'd'), &mut grid);
        if count_visited > max_count {
            max_count = count_visited;
        }
        let count_visited = traverse((x, grid.len() - 1, 'u'), &mut grid);
        if count_visited > max_count {
            max_count = count_visited;
        }
    }
    println!("Output: {}", max_count);
}

fn traverse(initial_beam: (usize, usize, char), grid: &mut Vec<Vec<char>>) -> i32 {
        let mut visited: HashSet<(usize, usize, char)> = HashSet::new();
        let mut queue: Vec<(usize, usize, char)> = Vec::new();
        queue.push(initial_beam);
        let mut count: HashMap<(usize, usize), usize> = HashMap::new();
    
        let last_x = grid[0].len() - 1;
        let last_y = grid.len() - 1;
        while !queue.is_empty() {
            let (x, y, beam_direction) = queue.remove(0);
            if !visited.insert((x, y, beam_direction)) {
                continue;
            }
            *count.entry((x, y)).or_insert(0) += 1;
    
            let current_cell = grid[y][x];
    
            match (beam_direction, current_cell) {
                ('u', '/') => if x < last_x { queue.push((x + 1, y, 'r')) },
                ('u', '\\') => if x > 0 { queue.push((x - 1, y, 'l')) },
                ('u', '|') => if y > 0 { queue.push((x, y - 1, 'u')) },
                ('u', '-') => {
                    if x > 0 { queue.push((x - 1, y, 'l')) };
                    if x < last_x { queue.push((x + 1, y, 'r')) };
                },
                ('u', '.') => if y > 0 { queue.push((x, y - 1, 'u')) },
                ('d', '/') => if x > 0 { queue.push((x - 1, y, 'l')) },
                ('d', '\\') => if x < last_x { queue.push((x + 1, y, 'r')) },
                ('d', '|') => if y < last_y { queue.push((x, y + 1, 'd')) },
                ('d', '-') => {
                    if x > 0 { queue.push((x - 1, y, 'l')) };
                    if x < last_x { queue.push((x + 1, y, 'r')) };
                },
                ('d', '.') => if y < last_y { queue.push((x, y + 1, 'd')) },
                ('l', '/') => if y < last_y { queue.push((x, y + 1, 'd')) },
                ('l', '\\') => if y > 0 { queue.push((x, y - 1, 'u')) },
                ('l', '|') => {
                    if y > 0 { queue.push((x, y - 1, 'u')) };
                    if y < last_y { queue.push((x, y + 1, 'd')) };
                },
                ('l', '-') => if x > 0 { queue.push((x - 1, y, 'l')) },
                ('l', '.') => if x > 0 { queue.push((x - 1, y, 'l')) },
                ('r', '/') => if y > 0 { queue.push((x, y - 1, 'u')) },
                ('r', '\\') => if y < last_y { queue.push((x, y + 1, 'd')) },
                ('r', '|') => {
                    if y > 0 { queue.push((x, y - 1, 'u')) };
                    if y < last_y { queue.push((x, y + 1, 'd')) };
                },
                ('r', '-') => if x < last_x { queue.push((x + 1, y, 'r')) },
                ('r', '.') => if x < last_x { queue.push((x + 1, y, 'r')) },
    
                _ => panic!("Invalid beam direction: {} and/or cell: {}", beam_direction, current_cell),
            }
        }
        let mut count_visited = 0;
        for (_, v) in count.iter() {
            if *v > 0 {
                count_visited += 1;
            }
        }
        count_visited
    }