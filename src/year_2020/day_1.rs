use std::collections::HashSet;

const REQUIRED_SUM: u32 = 2020;

#[aoc_generator(day1)]
fn get_numbers(input: &str) -> HashSet<u32> {
    input
        .lines()
        .map(|f| f.parse().expect("Expected input to only contain numbers."))
        .collect()
}

#[aoc(day1, part1)]
fn part_one(numbers: &HashSet<u32>) -> Option<u32> {
    for i in numbers {
        let want = REQUIRED_SUM - *i;

        if numbers.contains(&want) {
            return Some(i * want);
        }
    }

    None
}

#[aoc(day1, part2)]
fn part_two(numbers: &HashSet<u32>) -> Option<u32> {
    for i in numbers {
        for j in numbers {
            let sum = *i + *j;

            if sum > REQUIRED_SUM {
                continue;
            }

            let want = REQUIRED_SUM - sum;
            if numbers.contains(&want) {
                return Some(i * j * want);
            }
        }
    }

    None
}

#[cfg(test)]
mod test_day_1 {
    use super::*;

    fn get_input() -> &'static str {
        "1946\n1859\n16\n1432\n588\n1873\n1216\n347\n1657"
    }

    #[test]
    fn test_part1() {
        assert_eq!(part_one(&get_numbers(get_input())), Some(842016),)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_two(&get_numbers(get_input())), Some(9199664),)
    }
}
