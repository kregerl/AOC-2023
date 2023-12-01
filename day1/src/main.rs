fn main() {
    let input = include_str!("../input1.txt");
    part1(input); // 56049
    part2(input); // 54530
}

fn part1(input: &str) {
    let mut sum = 0;
    for line in input.lines() {
        let numbers = line
            .chars()
            .filter_map(|c| match c {
                '0'..='9' => Some(c.to_digit(10).unwrap()),
                _ => None,
            })
            .collect::<Vec<_>>();
        let mut number = numbers.first().unwrap() * 10u32;
        number += numbers.last().unwrap();
        sum += number;
    }
    println!("Sum: {}", sum);
}

fn part2(input: &str) {
    let mut sum = 0;
    for line in input.lines() {
        // Ugly but works
        let line = line
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e");
        let numbers = line
            .chars()
            .filter_map(|c| match c {
                '0'..='9' => Some(c.to_digit(10).unwrap()),
                _ => None,
            })
            .collect::<Vec<_>>();

        let mut number = numbers.first().unwrap() * 10u32;
        number += numbers.last().unwrap();
        sum += number;
    }
    println!("Sum: {}", sum);
}
