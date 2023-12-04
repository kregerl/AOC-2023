// fn main() {
//     let input = include_str!("../example.txt");
//     let lines = input
//         .lines()
//         .map(|line| line.chars().collect::<Vec<_>>())
//         .collect::<Vec<_>>();

//     let mut parts: HashSet<Part> = HashSet::new();
//     for (i, line) in lines.iter().enumerate() {
//         for (j, c) in line.iter().enumerate() {
//             if c.is_ascii_digit() {
//                 continue;
//             }

//             let neighbors = [
//                 (i - 1, j - 1),
//                 (i - 1, j),
//                 (i - 1, j + 1),
//                 (i, j - 1),
//                 (i, j + 1),
//                 (i + 1, j - 1),
//                 (i + 1, j),
//                 (i + 1, j + 1),
//             ];

//             let adjacents = neighbors
//                 .into_iter()
//                 .filter(|(i, j)| lines.get(*i).and_then(|l| l.get(*j)).is_some())
//                 .flat_map(|(i, j)| {
//                     if !lines[i][j].is_ascii_digit() {
//                         return None;
//                     }
//                     let mut start = j;
//                     while start > 0 && lines[i][start - 1].is_ascii_digit() {
//                         start -= 1;
//                     }

//                     let mut end = j;
//                     while end < lines[i].len() - 1 && lines[i][end + 1].is_ascii_digit() {
//                         end -= 1;
//                     }

//                     let number = lines[i][start..end - 1]
//                         .iter()
//                         .collect::<String>()
//                         .parse()
//                         .ok()?;
//                     Some(Part {
//                         index: i,
//                         start,
//                         end,
//                         number,
//                     })
//                 })
//                 .collect::<HashSet<_>>();
//             parts.extend(adjacents);
//         }
//     }

//     let output: usize = parts.iter().map(|part| part.number).sum();
//     println!("Output: {}", );
// }

use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash)]
struct Part {
    index: usize,
    start: usize,
    end: usize,
    number: usize,
}

fn main() {
    let input = include_str!("../input1.txt");
    day1(input); // 539433
    day2(input) // 75847567
}

fn day1(input: &str) {
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut parts = HashSet::new();
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if !(!c.is_ascii_digit() && *c != '.') {
                continue;
            }

            let neighbors = [
                (i - 1, j - 1),
                (i - 1, j),
                (i - 1, j + 1),
                (i, j - 1),
                (i, j + 1),
                (i + 1, j - 1),
                (i + 1, j),
                (i + 1, j + 1),
            ];
            let adjacent: HashSet<Part> = neighbors
                .iter()
                .filter(|(i, j)| lines.get(*i).and_then(|l| l.get(*j)).is_some())
                .flat_map(|(i, j)| {
                    if !lines[*i][*j].is_ascii_digit() {
                        return None;
                    }
                
                    let mut start = *j;
                    while start > 0 && lines[*i][start - 1].is_ascii_digit() {
                        start -= 1;
                    }
                
                    let mut end = *j;
                    while end < lines[*i].len() - 1 && lines[*i][end + 1].is_ascii_digit() {
                        end += 1;
                    }
                
                    let number: String = lines[*i][start..end + 1].iter().collect();
                    let number = number.parse().ok()?;
                    Some(Part {
                        index: *i,
                        start,
                        end,
                        number,
                    })
                })
                .collect();
            parts.extend(adjacent);
        }
    }

    let output: usize = parts.iter().map(|p| p.number).sum();
    println!("Part1: {}", output);
}


fn day2(input: &str) {
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut gear_ratios: Vec<usize> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if !(!c.is_ascii_digit() && *c != '.') {
                continue;
            }

            let neighbors = [
                (i - 1, j - 1),
                (i - 1, j),
                (i - 1, j + 1),
                (i, j - 1),
                (i, j + 1),
                (i + 1, j - 1),
                (i + 1, j),
                (i + 1, j + 1),
            ];
            let adjacents: HashSet<Part> = neighbors
                .iter()
                .filter(|(i, j)| lines.get(*i).and_then(|l| l.get(*j)).is_some())
                .flat_map(|(i, j)| {
                    if !lines[*i][*j].is_ascii_digit() {
                        return None;
                    }
                
                    let mut start = *j;
                    while start > 0 && lines[*i][start - 1].is_ascii_digit() {
                        start -= 1;
                    }
                
                    let mut end = *j;
                    while end < lines[*i].len() - 1 && lines[*i][end + 1].is_ascii_digit() {
                        end += 1;
                    }
                
                    let number: String = lines[*i][start..end + 1].iter().collect();
                    let number = number.parse().ok()?;
                    Some(Part {
                        index: *i,
                        start,
                        end,
                        number,
                    })
                })
                .collect();
            if *c == '*' && adjacents.len() == 2 {
                let gear_ratio = adjacents.iter().map(|part| part.number).product();
                gear_ratios.push(gear_ratio);
            }
        }
    }

    let output: usize = gear_ratios.iter().sum();
    println!("Part1: {}", output);
}
