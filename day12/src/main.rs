use std::{collections::HashMap, iter::once};

fn main() {
    let input = include_str!("../input.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let data = input
        .lines()
        .map(|l| {
            let mut f = l.split(' ');
            let springs = f.next().unwrap().to_string();
            let groups = f
                .next()
                .unwrap()
                .split(',')
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (springs, groups)
        })
        .collect::<Vec<_>>();

    let mut cache: HashMap<(String, Vec<usize>), usize> = HashMap::new();
    let output = data
        .iter()
        .map(|v: &(String, Vec<usize>)| validate_springs(&v.0, &v.1, &mut cache))
        .sum::<usize>();
    println!("Output: {}", output);
}

fn part2(input: &str) {
    let data = input
        .lines()
        .map(|l| {
            let mut f = l.split(' ');
            let springs = f.next().unwrap().to_string();
            let groups = f
                .next()
                .unwrap()
                .split(',')
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (springs, groups)
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    let mut cache: HashMap<(String, Vec<usize>), usize> = HashMap::new();
    for (springs, counts) in data {
        let new_springs = springs
            .chars()
            .chain(once('?'))
            .cycle()
            .take(5 * (springs.len() + 1) - 1)
            .collect::<String>();
        let new_counts: Vec<_> = counts
            .iter()
            .cycle()
            .take(counts.len() * 5)
            .copied()
            .collect();
        let result = validate_springs(&new_springs, &new_counts, &mut cache);
        sum += result
    }
    println!("Output: {}", sum);
}

fn validate_springs(
    springs: &str,
    counts: &[usize],
    cache: &mut HashMap<(String, Vec<usize>), usize>,
) -> usize {
    let cache_key = (springs.to_string(), counts.to_vec());
    let result = if let Some(res) = cache.get(&cache_key) {
        *res
    } else if springs.starts_with('.') {
        validate_springs(&springs[1..], counts, cache)
    } else if springs.starts_with('?') {
        let unknown_is_spring: String = springs.replacen('?', "#", 1);
        validate_springs(&unknown_is_spring, counts, cache)
            + validate_springs(&springs[1..], counts, cache)
    } else if springs.is_empty() {
        counts.is_empty() as usize
    } else if counts.is_empty() && springs.contains('#')
        || springs.len() < counts[0]
        || springs[..counts[0]].contains('.')
    {
        0
    } else if springs.len() == counts[0] {
        (counts.len() == 1) as usize
    } else if springs.chars().nth(counts[0]) == Some('#') {
        0
    } else {
        validate_springs(&springs[counts[0] + 1..], &counts[1..], cache)
    };
    cache.insert(cache_key, result);

    result
}
