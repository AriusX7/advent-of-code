#[derive(Debug, PartialEq)]
struct PasswordPolicy {
    letter: char,
    min: u8,
    max: u8
}

#[derive(Debug, PartialEq)]
struct Password {
    policy: PasswordPolicy,
    pass: String,
}

fn split_once<'a>(input: &'a str, pat: &str) -> Option<(&'a str, &'a str)> {
    let mut split = input.splitn(2, pat);

    if let Some(first) = split.next() {
        if let Some(rest) = split.next() {
            return Some((first, rest))
        }
    }

    None
}

#[aoc_generator(day2)]
fn get_passwords(input: &str) -> Vec<Password> {
    input.lines().map(|line| {
        let (min_str, rest) = split_once(line, "-").expect("Expected valid password policy.");

        let min = min_str.parse().expect("Expected valid password policy.");

        let (max_str, rest) = split_once(rest, " ").expect("Expected valid password policy.");

        let max = max_str.parse().expect("Expected valid password policy.");

        let (letter_str, rest) = split_once(rest, ":").expect("Expected valid password policy.");

        let letter = letter_str.parse().expect("Expected valid password policy.");

        Password {
            pass: rest.trim().to_string(),
            policy: PasswordPolicy {
                letter,
                min,
                max
            }
        }
    })
    .collect()
}

#[aoc(day2, part1)]
fn part_one(passwords: &Vec<Password>) -> u32 {
    let mut total = 0;

    for password in passwords {
        let count = password.pass.matches(password.policy.letter).count() as u8;
        if count >= password.policy.min && count <= password.policy.max {
            total += 1;
        }
    }

    total
}

#[aoc(day2, part2)]
fn part_two(passwords: &Vec<Password>) -> u32 {
    let mut total = 0;

    for password in passwords {
        let policy = &password.policy;

        let min_char = password.pass.chars().nth(policy.min as usize - 1).expect("Expected valid password.");
        let max_char = password.pass.chars().nth(policy.max as usize - 1).expect("Expected valid password.");

        if (min_char == policy.letter) ^ (max_char == policy.letter) {
            total += 1;
        }
    }

    total
}

#[cfg(test)]
mod test_day_2 {
    use super::*;

    #[test]
    fn test_input_parser() {
        assert_eq!(
            get_passwords("2-5 a: aaa\n3-14 c: abccc\n11-13 k: pqrs"),
            vec![
                Password {
                    pass: String::from("aaa"),
                    policy: PasswordPolicy {
                        letter: 'a',
                        min: 2,
                        max: 5,
                    }
                },
                Password {
                    pass: String::from("abccc"),
                    policy: PasswordPolicy {
                        letter: 'c',
                        min: 3,
                        max: 14,
                    }
                },
                Password {
                    pass: String::from("pqrs"),
                    policy: PasswordPolicy {
                        letter: 'k',
                        min: 11,
                        max: 13,
                    }
                },
            ]
        )
    }
}
