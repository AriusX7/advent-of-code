#[aoc_generator(day9)]
fn get_xmas(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|l| l.parse().expect("Expected valid number"))
        .collect()
}

fn is_valid(number: &u64, preamble: &[u64]) -> bool {
    for i in preamble {
        if number >= i && preamble.contains(&(number - i)) {
            return true;
        }
    }

    false
}

#[aoc(day9, part1)]
fn part_one(data: &[u64]) -> u64 {
    for (idx, number) in data.iter().skip(25).enumerate() {
        let preamble = data.get(idx..idx + 25).expect("msg");
        if !is_valid(number, preamble) {
            return *number;
        }
    }

    unreachable!()
}

fn min_and_max(data: &[u64]) -> (u64, u64) {
    let mut max = &u64::MIN;
    let mut min = &u64::MAX;

    for num in data {
        if num > &max {
            max = num;
        }

        if num < &min {
            min = num;
        }
    }

    (*min, *max)
}

fn remove_large_numbers(number: u64, stack: &mut Vec<u64>) {
    fn func(number: u64, sum: u64, stack: &mut Vec<u64>) {
        if sum > number && stack.len() > 1 {
            let item = stack.remove(0);
            func(number, sum - item, stack);
        }
    }

    let sum = stack.iter().sum::<u64>();
    func(number, sum, stack);
}

#[aoc(day9, part2)]
fn part_two(data: &[u64]) -> u64 {
    let invalid_number = part_one(data);

    let mut stack = Vec::new();

    for number in data {
        remove_large_numbers(invalid_number, &mut stack);

        let sum = stack.iter().sum::<u64>();
        if sum == invalid_number {
            let (min, max) = min_and_max(&stack);
            return min + max;
        }

        stack.push(*number);
    }

    unreachable!()
}
