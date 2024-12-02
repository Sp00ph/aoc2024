use itertools::Itertools;
use std::cmp::Reverse;

struct Report {
    levels: Vec<usize>,
}

fn parse_report(line: &str) -> Report {
    Report {
        levels: line
            .split_ascii_whitespace()
            .map(|i| i.parse().unwrap())
            .collect(),
    }
}

fn parse_input(input: &str) -> Vec<Report> {
    input.lines().map(parse_report).collect()
}

fn is_good(seq: impl Iterator<Item = usize> + Clone) -> bool {
    (seq.clone().is_sorted() || seq.clone().is_sorted_by_key(Reverse))
        && seq
            .tuple_windows()
            .all(|(a, b)| (1..=3).contains(&a.abs_diff(b)))
}

pub fn part1(input: &str) -> String {
    let reports = parse_input(input);
    reports
        .iter()
        .filter(|r| is_good(r.levels.iter().copied()))
        .count()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let reports = parse_input(input);
    reports
        .iter()
        .filter(|r| {
            (0..r.levels.len())
                .any(|i| is_good(r.levels[..i].iter().chain(&r.levels[i + 1..]).copied()))
        })
        .count()
        .to_string()
}
