fn main() {
    let input = include_str!("../input1.txt");
    part1(input); // 4568778
    part2(input); // 28973936
}

fn part1(input: &str) {
    let (time_line, distance_line) = input.split_once('\n').unwrap();
    let (_, time) = time_line.split_once(": ").unwrap();
    let (_, distance) = distance_line.split_once(": ").unwrap();

    let time = remove_empties(time)
        .into_iter()
        .map(|t| t.parse::<u64>().unwrap());
    let distance = remove_empties(distance)
        .into_iter()
        .map(|t| t.parse::<u64>().unwrap());

    let mut product = 1u64;
    for (t, d) in time.into_iter().zip(distance) {
        let mut num_ways_to_win = 0;
        for i in 0..t {
            let remaining_time = t - i;

            if i * remaining_time > d {
                num_ways_to_win += 1;
            }
        }
        product *= num_ways_to_win;
    }
    println!("Output: {}", product);
}

fn part2(input: &str) {
    let (time_line, distance_line) = input.split_once('\n').unwrap();
    let (_, time) = time_line.split_once(": ").unwrap();
    let (_, distance) = distance_line.split_once(": ").unwrap();

    let time = remove_empties(time).join("").parse::<u64>().unwrap();
    let distance = remove_empties(distance).join("").parse::<u64>().unwrap();


    let mut num_ways_to_win = 0;
    for i in 0..time {
        let remaining_time = time - i;

        if i * remaining_time > distance {
            num_ways_to_win += 1;
        }
    }
    println!("Output: {}", num_ways_to_win);
}

fn remove_empties(input: &str) -> Vec<&str> {
    input
        .trim()
        .split(" ")
        .filter(|num| !num.is_empty())
        .collect::<Vec<_>>()
}
