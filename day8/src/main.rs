use std::collections::HashMap;

fn main() {
    let input = include_str!("../input1.txt");
    part1(input); // 12169
    part2(input); // 12030780859469
}

fn part1(input: &str) {
    let (instructions, remaining) = input.split_once('\n').unwrap();

    let movements = remaining.lines().filter(|line| !line.is_empty()).map(|line| {
        let (key, value_pair) = line.split_once(" = ").unwrap();
        let no_parens = value_pair.replace("(", "").replace(")", "");
        let (left, right) = no_parens.split_once(", ").unwrap();
        (key.to_owned(), (left.to_owned(), right.to_owned()))
    }).collect::<HashMap<String, (String, String)>>();

    let mut steps = 0;
    let mut current = "AAA";
    'outer: loop {
        for c in instructions.chars() {
            let (left, right) = movements.get(current).unwrap();
            if c == 'L' {
                current = left;
            } else {
                current = right;
            }
            steps += 1;
            if current == "ZZZ" {
                break 'outer;
            }
        }
    }
    println!("Output: {}", steps);
}

fn part2(input: &str) {
    let (instructions, remaining) = input.split_once('\n').unwrap();

    let movements = remaining.lines().filter(|line| !line.is_empty()).map(|line| {
        let (key, value_pair) = line.split_once(" = ").unwrap();
        let no_parens = value_pair.replace("(", "").replace(")", "");
        let (left, right) = no_parens.split_once(", ").unwrap();
        (key.to_owned(), (left.to_owned(), right.to_owned()))
    }).collect::<HashMap<String, (String, String)>>();

    let mut cursors: Vec<&String> = movements
        .keys()
        .filter(|key| key.ends_with("A"))
        .collect();

    let instruction_counts: Vec<usize> = cursors
        .iter_mut()
        .map(|cursor| {
            let mut i: usize = 0;
            while !cursor.ends_with("Z") {
                for instruction in instructions.chars() {
                    let (left, right) = movements.get(&cursor.to_owned()).unwrap();
                    match instruction {
                        'L' => *cursor = left,
                        'R' => *cursor = right,
                        _ => (),
                    }
                    i += 1;
                }
            }
            i
        })
        .collect();

    let steps = instruction_counts
        .iter()
        .fold(instruction_counts[0], |acc, instruction| {
            lcm(acc, *instruction)
        });

    println!("Output: {}", steps);
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}