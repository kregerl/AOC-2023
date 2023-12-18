use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    part1(input); // 70253
    part2(input); // 131265059885080
}

const RE: &str = r#"(\w) (\d+) \(#(\S+)(\d)\)"#;

fn part1(input: &str){
    let re = Regex::new(RE).unwrap();
    let mut curr = (0, 0);
    let mut vertices = vec![curr];
    let mut boundary = 1;
    for [dir, steps, ..] in parse(input, &re) {
        let steps = steps.parse().unwrap();
        curr = proceed(curr, dir.as_bytes()[0], steps);
        vertices.push(curr);
        boundary += steps;
    }
    let output = solve(&vertices, boundary);
    println!("Output: {}", output);
}

fn part2(input: &str){
    let re = Regex::new(RE).unwrap();
    let mut curr = (0, 0);
    let mut vertices = vec![curr];
    let mut boundary = 1;
    for [.., steps, dir] in parse(input, &re) {
        let steps = isize::from_str_radix(steps, 16).unwrap();
        curr = proceed(curr, dir.as_bytes()[0], steps);
        vertices.push(curr);
        boundary += steps;
    }
    let output = solve(&vertices, boundary);
    println!("Output: {}", output);
}

fn solve(vertices: &[(isize, isize)], boundary: isize) -> isize {
    let area: isize = vertices
        .iter()
        .zip(vertices.iter().cycle().skip(1))
        .map(|(a, b)| a.0 * b.1 - a.1 * b.0)
        .sum();
    area.abs() / 2 + boundary / 2 + 1
}

const fn proceed(curr: (isize, isize), dir: u8, steps: isize) -> (isize, isize) {
    match dir {
        b'U' | b'3' => (curr.0, curr.1 - steps),
        b'D' | b'1' => (curr.0, curr.1 + steps),
        b'R' | b'0' => (curr.0 + steps, curr.1),
        b'L' | b'2' => (curr.0 - steps, curr.1),
        _ => unreachable!(),
    }
}

fn parse<'a>(input: &'a str, re: &'a Regex) -> impl Iterator<Item = [&'a str; 4]> {
    re.captures_iter(input).map(|cap| {
        let (_, group) = cap.extract::<4>();
        group
    })
}