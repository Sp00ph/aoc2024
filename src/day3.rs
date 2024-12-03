use memchr::memchr2_iter;

enum Command {
    Do,
    Dont,
    Mul { lhs: usize, rhs: usize },
}

fn commands(input: &str) -> impl Iterator<Item = Command> {
    fn strip_digits(b: &[u8]) -> Option<(usize, &[u8])> {
        let idx = b.iter().take(3).position(|c| !c.is_ascii_digit()).unwrap_or(b.len().min(3));
        if idx == 0 {
            return None;
        }
        let (digits, rest) = b.split_at(idx);
        let value = digits.iter().fold(0usize, |acc, digit| acc * 10 + usize::from(digit - b'0'));
        Some((value, rest))
    }

    let bytes = input.as_bytes();

    memchr2_iter(b'm', b'd', bytes).filter_map(|i| {
        let b = &bytes[i..];
        if b.starts_with(b"don't()") {
            Some(Command::Dont)
        } else if b.starts_with(b"do()") {
            Some(Command::Do)
        } else if let Some(b) = b.strip_prefix(b"mul(") {
            let (lhs, b) = strip_digits(b)?;
            let b = b.strip_prefix(b",")?;
            let (rhs, b) = strip_digits(b)?;
            let _ = b.strip_prefix(b")")?;
            Some(Command::Mul { lhs, rhs })
        } else {
            None
        }
    })
}

pub fn part1(input: &str) -> String {
    commands(input)
        .filter_map(|cmd| match cmd {
            Command::Mul { lhs, rhs } => Some(lhs * rhs),
            _ => None,
        })
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    commands(input)
        .fold((0usize, true), |(sum, enabled), cmd| match cmd {
            Command::Do => (sum, true),
            Command::Dont => (sum, false),
            Command::Mul { .. } if !enabled => (sum, enabled),
            Command::Mul { lhs, rhs } => (sum + lhs * rhs, enabled),
        })
        .0
        .to_string()
}
