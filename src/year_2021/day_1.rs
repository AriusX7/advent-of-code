#[aoc_generator(day1)]
fn get_numbers(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|f| f.parse().expect("Expected input to only contain numbers."))
        .collect()
}

fn windows_count(numbers: &[u32], size: usize) -> usize {
    numbers
        .windows(size)
        .filter(|&w| w[0] < w[size - 1])
        .count()
}

#[aoc(day1, part1)]
fn part_one(numbers: &[u32]) -> usize {
    windows_count(numbers, 2)
}

#[aoc(day1, part2)]
fn part_two(numbers: &Vec<u32>) -> usize {
    // We don't need to add sums because `w + x + y < x + y + z => w < z`.
    windows_count(numbers, 4)
}

#[cfg(test)]
mod test_day_1 {
    use super::*;

    fn get_input() -> &'static str {
        "199\n200\n208\n210\n200\n207\n240\n269\n260\n263"
    }

    #[test]
    fn test_part1() {
        assert_eq!(part_one(&get_numbers(get_input())), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_two(&get_numbers(get_input())), 5);
    }
}
