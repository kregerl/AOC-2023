use std::collections::{
    hash_map::Entry::{Occupied, Vacant},
    HashMap, VecDeque,
};

#[derive(Debug, Clone, Copy)]
enum ModuleType {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(u64),
}

fn parse<'a>(input: &'a str) -> Vec<(ModuleType, Vec<usize>)> {
    let mut modules = Vec::new();
    let mut indexes_by_name = HashMap::<&'a _, _>::new();
    let mut next_index = 1;

    let mut get_index = |name: &'a _| match indexes_by_name.entry(name) {
        Occupied(entry) => *entry.get(),
        Vacant(entry) => {
            let index = next_index;
            next_index += 1;
            entry.insert(index);
            index
        }
    };

    for line in input.lines() {
        let (name, destinations) = line.split_once(" -> ").unwrap();
        let (module_type, index) = if name == "broadcaster" {
            (ModuleType::Broadcaster, 0)
        } else {
            (
                match &name[..1] {
                    "%" => ModuleType::FlipFlop(false),
                    "&" => ModuleType::Conjunction(u64::MAX),
                    _ => panic!(),
                },
                get_index(&name[1..]),
            )
        };

        let destinations: Vec<_> = destinations.split(", ").map(|x| get_index(x)).collect();
        if index >= modules.len() {
            modules.resize(index + 1, (ModuleType::Broadcaster, Vec::new()));
        }
        modules[index] = (module_type, destinations);
    }

    modules
}

fn part1<'a>(input: &'a str) {
    let mut modules = parse(input);

    for i in 0..modules.len() {
        for j in 0..modules[i].1.len() {
            let target = modules[i].1[j];
            if let ModuleType::Conjunction(states) = &mut modules[target].0 {
                *states &= !(1 << i);
            }
        }
    }

    let mut low_pulses = 0;
    let mut high_pulses = 0;
    for _ in 0..1000 {
        let mut pulses = VecDeque::new();
        pulses.push_back((false, 0, usize::MAX));
        low_pulses += 1;

        while let Some((pulse, target, source)) = pulses.pop_front() {
            if let Some(resulting_pulse) = match &mut modules[target].0 {
                ModuleType::Broadcaster => Some(pulse),
                ModuleType::Conjunction(state) => {
                    if pulse {
                        *state |= 1 << source;
                    } else {
                        *state &= !(1 << source);
                    }
                    Some(*state != u64::MAX)
                }
                ModuleType::FlipFlop(state) => {
                    if pulse {
                        None
                    } else {
                        *state = !*state;
                        Some(*state)
                    }
                }
            } {
                if resulting_pulse {
                    high_pulses += modules[target].1.len() as u32;
                } else {
                    low_pulses += modules[target].1.len() as u32;
                }

                for &new_target in &modules[target].1 {
                    pulses.push_back((resulting_pulse, new_target, target));
                }
            }
        }
    }

    let output = low_pulses * high_pulses;
    println!("Output: {}", output);
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn part2(input: &str) {
    let modules = parse(input);

    let mut cycler_members = Vec::new();
    let output = modules[0]
        .1
        .iter()
        .map(|&cycler_start| {
            let mut cycle_controller = 0;
            let mut current_module = cycler_start;
            let mut exit = false;

            while !exit {
                cycler_members.push(current_module);
                exit = true;
                for &target in &modules[current_module].1 {
                    match modules[target].0 {
                        ModuleType::Conjunction(_) => {
                            cycle_controller = target;
                        }
                        ModuleType::FlipFlop(_) => {
                            current_module = target;
                            exit = false;
                        }
                        ModuleType::Broadcaster => panic!(),
                    }
                }
            }

            let period = cycler_members
                .iter()
                .enumerate()
                .fold(0u64, |acc, (bit, &module)| {
                    if modules[module].1.contains(&cycle_controller) {
                        acc | (1 << bit)
                    } else {
                        acc
                    }
                });

            cycler_members.clear();
            period
        })
        .reduce(lcm)
        .unwrap();

    println!("Output: {}", output);
}

fn main() {
    let input = include_str!("../input.txt");
    part1(&input); // 794930686
    part2(&input); // 244465191362269
}
