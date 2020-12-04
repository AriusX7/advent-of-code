use crate::split_once;

#[aoc_generator(day4)]
fn generate_passport_data(input: &str) -> Vec<String> {
    // input.lines().map(|l| l.to_string()).collect()
    input.split("\n\n").map(|l| l.to_string()).collect()
}

#[aoc(day4, part1)]
fn part_one(data: &[String]) -> u32 {
    let mut total = 0;

    for item in data {
        if item.contains("byr")
            && item.contains("iyr")
            && item.contains("eyr")
            && item.contains("hgt")
            && item.contains("hcl")
            && item.contains("ecl")
            && item.contains("pid")
        {
            total += 1;
        }
    }

    total
}

fn check_year(input: &str, pat: &str, low: u32, high: u32) -> bool {
    split_once(&input, pat)
        .and_then(|(_, rest)| {
            rest.get(0..4).and_then(|d_str| {
                d_str.parse::<u32>().ok().and_then(|d| {
                    if d >= low && d <= high && rest.get(4..5).unwrap_or(" ").trim().is_empty() {
                        Some(())
                    } else {
                        None
                    }
                })
            })
        })
        .is_some()
}

fn check_height(input: &str) -> bool {
    let res = if let Some((_, rest)) = split_once(input, "hgt:") {
        rest.split_ascii_whitespace().next().and_then(|hgt| {
            if hgt.len() == 4 {
                if hgt.get(2..4).unwrap() == "in" {
                    hgt.get(0..2).and_then(|d_str| d_str.parse::<u8>().ok().and_then(|d| {
                        if d >= 59 && d <= 76 {
                            Some(())
                        } else {
                            None
                        }
                    }))
                } else {
                    None
                }
            } else if hgt.len() == 5 {
                if hgt.get(3..5).unwrap() == "cm" {
                    hgt.get(0..3).and_then(|d_str| d_str.parse::<u8>().ok().and_then(|d| {
                        if d >= 150 && d <= 193 {
                            Some(())
                        } else {
                            None
                        }
                    }))
                } else {
                    None
                }
            } else {
                None
            }
        })
    } else {
        None
    };

    res.is_some()
}

#[aoc(day4, part2)]
fn part_two(data: &[String]) -> u32 {
    let mut total = 0;

    for item in data {
        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        if !check_year(item, "byr:", 1920, 2002) {
            continue;
        }

        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        if !check_year(item, "iyr:", 2010, 2020) {
            continue;
        }

        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        if !check_year(item, "eyr:", 2020, 2030) {
            continue;
        }

        // hgt (Height) - a number followed by either cm or in:
        //     - If cm, the number must be at least 150 and at most 193.
        //     - If in, the number must be at least 59 and at most 76.
        if !check_height(item) {
            continue;
        }

        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        if split_once(&item, "hcl:")
            .and_then(|(_, rest)| {
                rest.get(0..7).and_then(|pattern| {
                    if pattern.starts_with("#")
                        && pattern
                            .get(1..7)
                            .unwrap()
                            .chars()
                            .all(|c| c.is_alphanumeric())
                        && rest.get(7..8).unwrap_or(" ").trim().is_empty()
                    {
                        Some(())
                    } else {
                        None
                    }
                })
            })
            .is_none()
        {
            continue;
        }

        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        if split_once(&item, "ecl:")
            .and_then(|(_, rest)| {
                rest.get(0..3).and_then(|color| {
                    if ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&color)
                        && rest.get(3..4).unwrap_or(" ").trim().is_empty()
                    {
                        Some(())
                    } else {
                        None
                    }
                })
            })
            .is_none()
        {
            continue;
        }

        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        if split_once(&item, "pid:")
            .and_then(|(_, rest)| {
                rest.get(0..9).and_then(|d_str| {
                    if d_str.parse::<u32>().is_ok() && rest.get(9..10).unwrap_or(" ").trim().is_empty() {
                        Some(())
                    } else {
                        None
                    }
                })
            })
            .is_none()
        {
            continue;
        }

        total += 1;
    }

    total
}
