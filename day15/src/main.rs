fn main() {
    let input = include_str!("../input.txt");
    part1(input); // 495972
    part2(input); // 245223
}

fn part1(input: &str) {
    let sequence: Vec<&str> = input.split(",").collect();
    let mut sum = 0;
    for seq in sequence {
        sum += ascii_hash(seq);
    }

    println!("Output: {}", sum);
}

#[derive(Debug, Clone)]
struct Box {
    lenses: Vec<(usize, u8)>,
}

impl Box {
    fn new() -> Self {
        Self {
            lenses: Vec::with_capacity(6),
        }
    }

    fn add(&mut self, hash: usize, focal_length: u8) {
        if let Some(pos) = self.lenses.iter().position(|(pos, _)| *pos == hash) {
            self.lenses[pos].1 = focal_length;
        } else {
            self.lenses.push((hash, focal_length));
        }
    }

    fn remove(&mut self, hash: usize) {
        if let Some(pos) = self.lenses.iter().position(|(pos, _)| *pos == hash) {
            self.lenses.remove(pos);
        }
    }
}

fn part2(input: &str) {
    let sequence: Vec<&str> = input.split(",").collect();
    let mut boxes = vec![Box::new(); 256];

    for seq in sequence {
        if seq.ends_with("-") {
            let (label, _) = seq.split_once("-").unwrap();
            let hash = ascii_hash(label);
            boxes[hash].remove(hash + label.as_bytes()[0] as usize);
        } else {
            let (label, focal_len) = seq.split_once("=").unwrap();
            let hash = ascii_hash(label);
            let focal_length = focal_len.parse::<u8>().unwrap();
            boxes[hash].add(hash + label.as_bytes()[0] as usize, focal_length);
        }

    }
    let output = boxes.iter().enumerate().filter(|(i, b)| !b.lenses.is_empty()).map(|(box_index, b)| {
        b.lenses.iter().enumerate().map(|(lens_index, lens)| {
            (1 + box_index) * (lens_index + 1) * lens.1 as usize
        }).sum::<usize>()
    }).sum::<usize>();
    println!("Output: {}",output);
}

fn ascii_hash(s: &str) -> usize {
    let mut hash = 0;
    for c in s.chars() {
        hash += c as usize;
        hash *= 17;
        hash %= 256;
    }
    hash
}