fn main() {
    let input = include_str!("../input1.txt");
    part1(input); // 2105961943
    part2(input); // 1019
}

fn part1(input: &str) {
    let mut sum = 0;
    for line in input.lines() {
        let line_numbers = line
            .split(" ")
            .map(|num| {
                num.parse::<i64>()
                    .expect(&format!("Cannot convert {}", num))
            })
            .collect::<Vec<_>>();

        let mut history = Vec::new();
        history.push(line_numbers.clone());
        let mut differences = diffs(&line_numbers);
        history.push(differences.clone());
        while differences.iter().any(|x| *x != 0) {
            differences = diffs(&differences);
            history.push(differences.clone());
        }
        history.reverse();

        let mut prediction = 0i64;
        for x in history.windows(2) {
            let _ = &x[0];
            let second = &x[1];
            prediction = prediction + second.last().unwrap();
        }
        sum += prediction;
    }
    println!("Output: {}", sum);
}

fn part2(input: &str) {
    let mut sum = 0;
    for line in input.lines() {
        let line_numbers = line
            .split(" ")
            .map(|num| {
                num.parse::<i64>()
                    .expect(&format!("Cannot convert {}", num))
            })
            .collect::<Vec<_>>();

        let mut history = Vec::new();
        history.push(line_numbers.clone());
        let mut differences = diffs(&line_numbers);
        history.push(differences.clone());
        while differences.iter().any(|x| *x != 0) {
            differences = diffs(&differences);
            history.push(differences.clone());
        }
        history.reverse();
        let mut prediction = 0i64;
        for x in history.windows(2) {
            let _ = &x[0];
            let second = &x[1];
            prediction = second.first().unwrap() - prediction;
        }
        sum += prediction;
    }
    println!("Output: {}", sum);
}

fn diffs(numbers: &[i64]) -> Vec<i64> {
    let mut differences = Vec::new();
    for numbers in numbers.windows(2) {
        let a = &numbers[0];
        let b = &numbers[1];
        differences.push(b - a);
    }
    differences
}
