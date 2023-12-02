use std::cmp::max;

fn main() {
    let contents = include_str!("../input1.txt");
    part1(contents); // 2879
    part2(contents); // 65122
}

fn part1(input: &str) {
    const NUM_RED: u32 = 12;
    const NUM_GREEN: u32 = 13;
    const NUM_BLUE: u32 = 14;

    let mut sum = 0;
    'lines: for line in input.lines() {
        let (game_number, remainder) = line.split_once(": ").unwrap();
        let (_, game_id) = game_number.split_once(" ").unwrap();

        for cubes in remainder.split("; ") {
            for cube_count in cubes.split(", ") {
                let (num, color) = cube_count.split_once(" ").unwrap();
                match color {
                    "red" if num.parse::<u32>().unwrap() > NUM_RED => {
                        continue 'lines;
                    }
                    "green" if num.parse::<u32>().unwrap() > NUM_GREEN => {
                        continue 'lines;
                    }
                    "blue" if num.parse::<u32>().unwrap() > NUM_BLUE => {
                        continue 'lines;
                    }
                    _ => {}
                }
            }
        }
        sum += game_id.parse::<u32>().unwrap();
    }
    println!("Output: {}", sum);
}

fn part2(input: &str) {
    let mut sum = 0;
    for line in input.lines() {
        let (_, remainder) = line.split_once(": ").unwrap();

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for cubes in remainder.split("; ") {
            for cube_count in cubes.split(", ") {
                let (num, color) = cube_count.split_once(" ").unwrap();
                match color {
                    "red" => max_red = max(max_red, num.parse::<u32>().unwrap()),
                    "green" => max_green = max(max_green, num.parse::<u32>().unwrap()),
                    "blue" => max_blue = max(max_blue, num.parse::<u32>().unwrap()),
                    _ => {}
                }
            }
        }
        sum += max_red * max_green * max_blue;
    }
    println!("Output: {}", sum);
}
