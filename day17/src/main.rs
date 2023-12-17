use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    let input = include_str!("../input.txt");
    part1(input); // 851
    part2(input);
}

fn part1(input: &str) {
    let output = best_path::<1, 3>(input);
    println!("Output: {}", output);
}

fn part2(input: &str) {
    let output = best_path::<4, 10>(input);
    println!("Output: {}", output);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

struct Tile {
    heat_loss: u8,
    g_scores: [u16; 2],
}

fn best_path<const MIN_STEPS: usize, const MAX_STEPS: usize>(input: &str) -> u16 {
    let mut map: Vec<Vec<_>> = input.lines().map(|line| {
        line.chars().map(|c| Tile {
            heat_loss: c.to_digit(10).unwrap() as u8,
            g_scores: [u16::MAX; 2],
        }).collect()
    }).collect();

    const START: (u8, u8) = (0, 0);
    let goal = (map[0].len() - 1, map.len() - 1);

    let mut frontier = BinaryHeap::new();
    frontier.push((Reverse(0), Direction::None, START));

    map[START.1 as usize][START.0 as usize].g_scores = [0; 2];

    while let Some((Reverse(g_score), direction, (x, y))) = frontier.pop() {
        let (x, y) = (x as usize, y as usize);

        if g_score != map[y][x].g_scores[(direction as usize >> 1) & 0b1] {
            continue;
        }

        if (x, y) == goal {
            return g_score;
        }

        for new_direction in match direction {
            Direction::Up | Direction::Down => [Direction::Right, Direction::Left],
            Direction::Right | Direction::Left => [Direction::Up, Direction::Down],
            Direction::None => [Direction::Down, Direction::Right], // Special case for starting tile
        } {
            let mut tentative_g_score = g_score;
            for steps in 1..=MAX_STEPS {
                let (new_x, new_y) = match new_direction {
                    Direction::Up => (x, y.wrapping_sub(steps)),
                    Direction::Down => (x, y + steps),
                    Direction::Right => (x + steps, y),
                    Direction::Left => (x.wrapping_sub(steps), y),
                    Direction::None => panic!(),
                };

                if new_x >= map[0].len() || new_y >= map.len() {
                    continue;
                }

                let neighbor = &mut map[new_y][new_x];
                tentative_g_score += neighbor.heat_loss as u16;

                if steps >= MIN_STEPS {
                    let old_g_score = neighbor.g_scores[new_direction as usize / 2];

                    if tentative_g_score < old_g_score {
                        neighbor.g_scores[new_direction as usize / 2] = tentative_g_score;
                        frontier.push((Reverse(tentative_g_score), new_direction, (new_x as u8, new_y as u8)))
                    }
                }
            }
        }
    }

    unreachable!()
}