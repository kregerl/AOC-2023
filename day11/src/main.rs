fn main() {
    let input = include_str!("../input1.txt");
    solve(input); // 9312968, 597714117556
}

fn solve(input: &str) {
    let size = input.chars().position(|b| b == '\n').unwrap();
    let (mut xx, mut yy) = (vec![0; size], vec![0; size]);
    input.chars()
        .enumerate()
        .filter(|(_, b)| *b == '#')
        .for_each(|(pos, _)| {
            xx[pos % (size + 1)] += 1;
            yy[pos / (size + 1)] += 1;
        });

    println!("Output: {}", dist(&xx, 1) + dist(&yy, 1));
    println!("Output: {}", dist(&xx, 999_999) + dist(&yy, 999_999));
}

fn dist(counts: &[usize], increment: usize) -> usize {
    let (mut gaps, mut sum, mut items, mut dist) = (0, 0, 0, 0);
    for (i, count) in counts.iter().enumerate() {
        if *count > 0 {
            let expanded = i + increment * gaps;
            dist += count * (items * expanded - sum);
            sum += count * expanded;
            items += count;
        } else {
            gaps += 1;
        }
    }
    dist
}