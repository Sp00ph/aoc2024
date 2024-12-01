use ahash::AHashMap;

pub fn part1(input: &str) -> String {
    let (mut left, mut right) = (vec![], vec![]);
    for line in input.lines() {
        let mut s = line.split_ascii_whitespace();
        left.push(s.next().unwrap().parse::<isize>().unwrap());
        right.push(s.next().unwrap().parse::<isize>().unwrap());
    }

    left.sort_unstable();
    right.sort_unstable();
    left.iter()
        .zip(right.iter())
        .map(|(&i, &j)| (i - j).unsigned_abs())
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let (mut left, mut right) = (vec![], AHashMap::new());
    for line in input.lines() {
        let mut s = line.split_ascii_whitespace();
        left.push(s.next().unwrap().parse::<usize>().unwrap());
        *right
            .entry(s.next().unwrap().parse::<usize>().unwrap())
            .or_default() += 1;
    }

    left.into_iter()
        .map(|i| i * right.get(&i).map_or(0, |&j| j))
        .sum::<usize>()
        .to_string()
}
