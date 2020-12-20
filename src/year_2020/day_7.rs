use regex::Regex;
use std::collections::HashMap;

use crate::split_once;

const BAG_COLOR: &str = "shiny gold";

struct Bag {
    color: String,
    bags: HashMap<String, u8>,
}

#[aoc_generator(day7)]
fn get_bags_map(input: &str) -> Vec<Bag> {
    let line_re = Regex::new(r#"(\d+)\s(\w+\s\w+)\sbags?"#).unwrap();

    input
        .lines()
        .map(|line| {
            let (clr, rest) = split_once(line, "bags ").expect("Expected valid line.");

            let color = clr.trim().to_string();

            let mut bags = HashMap::new();
            for cap in line_re.captures_iter(rest) {
                let quan = cap[1].parse().expect("Expected valid number.");
                let name = cap[2].to_string();

                bags.insert(name, quan);
            }

            Bag { color, bags }
        })
        .collect()
}

fn get_bag<'a>(bags: &'a [Bag], color: &str) -> Option<&'a Bag> {
    bags.iter().find(|&b| b.color == color)
}

fn find_shiny_gold_bag(bag: &Bag, bags: &[Bag]) -> bool {
    for sub_color in bag.bags.keys() {
        if sub_color == BAG_COLOR {
            return true;
        }

        if let Some(sub_bag) = get_bag(bags, sub_color) {
            if find_shiny_gold_bag(sub_bag, bags) {
                return true;
            }
        };
    }

    false
}

#[aoc(day7, part1)]
fn part_one(bags: &[Bag]) -> u32 {
    let mut total = 0;

    for bag in bags {
        if find_shiny_gold_bag(bag, bags) {
            total += 1;
        }
    }

    total
}

fn count_total_bags(bag: &Bag, bags: &[Bag]) -> u32 {
    let mut total = 0;

    for (sub_color, count) in &bag.bags {
        if let Some(sub_bag) = get_bag(bags, sub_color) {
            let count = *count as u32;
            total += count + count * count_total_bags(sub_bag, bags);
        };
    }

    total
}

#[aoc(day7, part2)]
fn part_two(bags: &[Bag]) -> u32 {
    let bag = get_bag(bags, BAG_COLOR).expect("Expected shiny gold bag.");

    count_total_bags(bag, bags)
}

#[cfg(test)]
mod test_day_7 {
    use super::*;

    #[test]
    fn test_count_total_bags() {
        let input = "shiny gold bags contain 2 dark red bags.
            dark red bags contain 2 dark orange bags.
            dark orange bags contain 2 dark yellow bags.
            dark yellow bags contain 2 dark green bags.
            dark green bags contain 2 dark blue bags.
            dark blue bags contain 2 dark violet bags.
            dark violet bags contain no other bags.";

        let bags = get_bags_map(input);

        let bag = get_bag(&bags, BAG_COLOR).unwrap();

        assert_eq!(count_total_bags(&bag, &bags), 126);
    }
}
