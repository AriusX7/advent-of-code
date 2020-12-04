mod year_2020;

#[macro_use]
extern crate aoc_runner_derive;

aoc_lib! { year = 2020 }

fn split_once<'a>(input: &'a str, pat: &str) -> Option<(&'a str, &'a str)> {
    let mut split = input.splitn(2, pat);

    if let Some(first) = split.next() {
        if let Some(rest) = split.next() {
            return Some((first, rest));
        }
    }

    None
}
