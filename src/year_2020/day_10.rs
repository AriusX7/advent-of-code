use std::collections::HashMap;

#[aoc_generator(day10)]
fn get_adapters(input: &str) -> Vec<u64> {
    let mut adapters: Vec<u64> = input
        .lines()
        .map(|l| l.parse().expect("Expected valid number"))
        .collect();

    adapters.push(0);
    adapters.sort_unstable();
    adapters
}

#[aoc(day10, part1)]
fn part_one(adapters: &[u64]) -> u64 {
    let mut one_diff = 0;
    let mut three_diff = 0;

    for window in adapters.windows(2) {
        match window[1] - window[0] {
            1 => one_diff += 1,
            3 => three_diff += 1,
            _ => (),
        }
    }

    one_diff * (three_diff + 1)
}

#[aoc(day10, part2)]
fn part_two(adapters: &[u64]) -> u64 {
    let last = *adapters.last().expect("Expected at least one adapter.");

    let mut map = HashMap::new();
    map.insert(last, 1);

    for adapter in adapters.iter().rev().skip(1) {
        for i in 1..=3 {
            let prev = *map.get(&(adapter + i)).unwrap_or(&0);
            let entry = map.entry(*adapter).or_insert(0);

            *entry += prev;
        }
    }

    *map.get(&0).unwrap()
}
