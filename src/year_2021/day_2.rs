#[derive(Debug, Clone, Copy)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

#[aoc_generator(day2)]
fn commands_generator(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|s| {
            let last = s
                .chars()
                .last()
                .expect("Expected every line to contain at least one character")
                .to_digit(10)
                .expect("Expected last character to be a valid base-10 digit.");

            if s.starts_with('f') {
                Command::Forward(last)
            } else if s.starts_with('d') {
                Command::Down(last)
            } else {
                Command::Up(last)
            }
        })
        .collect()
}

#[aoc(day2, part1)]
fn part_one(commands: &[Command]) -> u32 {
    let mut horizontal = 0;
    let mut vertical = 0;

    for command in commands {
        match command {
            Command::Forward(s) => horizontal += s,
            Command::Down(s) => vertical += s,
            Command::Up(s) => vertical -= s,
        }
    }

    horizontal * vertical
}

#[aoc(day2, part2)]
fn part_two(commands: &[Command]) -> u32 {
    let mut horizontal = 0;
    let mut aim = 0;
    let mut depth = 0;

    for command in commands {
        match command {
            Command::Forward(s) => {
                horizontal += s;
                depth += s * aim;
            }
            Command::Down(s) => aim += s,
            Command::Up(s) => aim -= s,
        }
    }

    horizontal * depth
}

#[cfg(test)]
mod test_day_2 {
    use super::*;

    fn get_input() -> &'static str {
        "f 5\nd 5\nf 8\nup 3\ndown 8\nfor 2"
    }

    #[test]
    fn test_part1() {
        assert_eq!(part_one(&commands_generator(get_input())), 150);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_two(&commands_generator(get_input())), 900);
    }
}
