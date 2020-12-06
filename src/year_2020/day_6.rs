use std::collections::{HashMap, HashSet};

#[aoc_generator(day6)]
fn get_groups(input: &str) -> Vec<String> {
    input.split("\n\n").map(|l| l.to_string()).collect()
}

#[aoc(day6, part1)]
fn part_one(groups: &[String]) -> usize {
    let mut total = 0;

    for group in groups {
        let mut set = HashSet::new();
        for ch in group.chars() {
            if !ch.is_whitespace() {
                set.insert(ch);
            }
        }

        total += set.len();
    }

    total
}

#[aoc(day6, part2)]
fn part_two(groups: &[String]) -> usize {
    let mut total = 0;

    for group in groups {
        let mut map = HashMap::new();
        let total_people = group.lines().count();

        for person in group.lines() {
            for ch in person.chars() {
                let count = map.entry(ch).or_insert(0);
                *count += 1;
            }
        }

        for (_, num) in map {
            if num == total_people {
                total += 1;
            }
        }
    }

    total
}
