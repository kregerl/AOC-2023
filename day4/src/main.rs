fn main() {
    let input = include_str!("../input1.txt");
    part1(input); // 21105
    part2(input); // 5329815
}

fn part1(input: &str) {
    let mut sum = 0;
    for line in input.lines() {
        let (_, numbers) = line.split_once(": ").unwrap();
        let (winning_numbers, my_numbers) = numbers.split_once(" | ").unwrap();

        let winning_numbers = winning_numbers
            .split(" ")
            .filter_map(|number_str| number_str.trim().parse::<u32>().ok())
            .collect::<Vec<_>>();

        let my_numbers = my_numbers
            .split(" ")
            .filter_map(|number_str| number_str.trim().parse::<u32>().ok())
            .collect::<Vec<_>>();

        let mut num_winning_numbers = 0;
        for number in my_numbers {
            if winning_numbers.contains(&number) {
                num_winning_numbers += 1;
            }
        }

        sum += if num_winning_numbers > 0 {
            2u32.pow(num_winning_numbers - 1)
        } else {
            0
        };
    }
    println!("Output: {}", sum);
}

fn part2(input: &str) {
    let mut cards: Vec<usize> = vec![0; input.trim().lines().count()];
    for card in input.trim().lines() {
        let (card_number, numbers) = card.split_once(": ").unwrap();
        let (_, card_number) = card_number.split_once(" ").unwrap();
        let game_id = card_number.trim().parse::<usize>().unwrap() - 1;

        let (winning_numbers, my_numbers) = numbers.split_once(" | ").unwrap();

        let winning_numbers = winning_numbers
            .split(" ")
            .filter_map(|number_str| number_str.trim().parse::<u32>().ok())
            .collect::<Vec<_>>();

        let my_numbers = my_numbers
            .split(" ")
            .filter_map(|number_str| number_str.trim().parse::<u32>().ok())
            .collect::<Vec<_>>();

        let current_card = cards[game_id as usize];
        let mut copy_idx = game_id;

        for number in my_numbers.iter() {
            if winning_numbers.contains(number) {
                copy_idx += 1;
                cards[copy_idx as usize] += 1 * (current_card + 1);
            }
        }
    }

    let answer = cards.iter().fold(0, |acc, x| acc + (x + 1));
    println!("Output: {}", answer);
}