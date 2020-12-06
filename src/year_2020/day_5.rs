#[aoc_generator(day5)]
fn get_boarding_ids(input: &str) -> Vec<u16> {
    input
        .lines()
        .map(|l| {
            // Every pass string is composed of two binary strings.
            // The first seven characters form a 7-bit binary string with 0 replaced with "F"
            // and 1 replaced with "B". This binary number represents the row number.
            // Similarly, the last three characters form a 3-bit binary string with 0 replaced with "L"
            // and 1 replaced with "R". This binary number represents the column number.
            // Seat ID is 8 * row + column, which is the same as considering the pass as a
            // 10-bit binary string where "L" and "F" represent 0 and "R" and "B" represent 1.
            l.chars().fold(0, |acc, ch| match ch {
                'F' | 'L' => acc << 1,
                'B' | 'R' => (acc << 1) | 1,
                _ => unreachable!(),
            })
        })
        .collect()
}

#[aoc(day5, part1)]
fn part_one(ids: &[u16]) -> u16 {
    *ids.iter().max().expect("Expected at least one seat ID.")
}

#[aoc(day5, part2)]
fn part_two(ids: &[u16]) -> u16 {
    let mut min = u16::max_value();
    let mut max = u16::min_value();
    let mut actual_total = 0;

    for &id in ids {
        min = min.min(id);
        max = max.max(id);
        actual_total += id;
    }

    let mut total = 0;
    for id in min..=max {
        total += id;
    }

    total - actual_total
}
