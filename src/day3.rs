use regex::Regex;

pub fn part1(input: &str) -> String {
    let r = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    r.captures_iter(input)
        .map(|m| m.extract())
        .map(|(_, [l, r])| l.parse::<usize>().unwrap() * r.parse::<usize>().unwrap())
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let r = Regex::new(r"(mul|do|don't)\((?:(\d{1,3}),(\d{1,3}))?\)").unwrap();

    r.captures_iter(input)
        .fold((0usize, true), |(sum, enabled), m| match &m[1] {
            "do" => (sum, true),
            "don't" => (sum, false),
            "mul" if !enabled => (sum, enabled),
            "mul" => (
                sum + m[2].parse::<usize>().unwrap() * m[3].parse::<usize>().unwrap(),
                enabled,
            ),
            _ => unreachable!()
        })
        .0
        .to_string()
}
