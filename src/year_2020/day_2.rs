use crate::split_once;

#[derive(Debug, PartialEq)]
struct PasswordPolicy {
    letter: char,
    min: u8,
    max: u8,
}

#[derive(Debug, PartialEq)]
struct Password {
    policy: PasswordPolicy,
    pass: String,
}

#[aoc_generator(day2)]
fn get_passwords(input: &str) -> Vec<Password> {
    input
        .lines()
        .map(|line| {
            let (min_str, rest) = split_once(line, "-").expect("Expected valid password policy.");

            let min = min_str.parse().expect("Expected valid password policy.");

            let (max_str, rest) = split_once(rest, " ").expect("Expected valid password policy.");

            let max = max_str.parse().expect("Expected valid password policy.");

            let (letter_str, rest) =
                split_once(rest, ":").expect("Expected valid password policy.");

            let letter = letter_str.parse().expect("Expected valid password policy.");

            Password {
                pass: rest.trim().to_string(),
                policy: PasswordPolicy { letter, min, max },
            }
        })
        .collect()
}

#[aoc(day2, part1)]
fn part_one(passwords: &[Password]) -> u32 {
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
fn part_two(passwords: &[Password]) -> u32 {
    let mut total = 0;

    for password in passwords {
        let policy = &password.policy;

        let mut chars = password.pass.chars();
        let min_idx = policy.min as usize - 1;

        let min_char = chars.nth(min_idx).expect("Expected valid password.");
        let max_char = if let Some(c) = chars.nth(policy.max as usize - min_idx - 2) {
            c
        } else {
            if min_char == policy.letter {
                total += 1;
            }
            continue;
        };

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

    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&get_passwords("2-5 a: aaa\n3-14 c: abccc\n11-13 k: pqrs")),
            2
        )
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&get_passwords(
                "2-5 a: aaa\n3-14 c: abccc\n11-13 k: pqrsssakwbbca"
            )),
            2
        )
    }
}
