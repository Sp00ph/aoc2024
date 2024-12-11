use ahash::AHashMap;

fn parse_input(input: &str) -> Vec<usize> {
    input.trim().split_ascii_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn split(n: usize) -> Option<(usize, usize)> {
    let digits = n.ilog10() + 1;
    if digits % 2 == 0 {
        let div = 10usize.pow(digits / 2);
        Some((n / div, n % div))
    } else {
        None
    }
}

fn count(stones: &[usize]) -> AHashMap<usize, usize> {
    let mut res = AHashMap::default();
    for &s in stones {
        *res.entry(s).or_default() += 1;
    }

    res
}

fn blink_count(stones: &AHashMap<usize, usize>, out: &mut AHashMap<usize, usize>) {
    for (&s, &count) in stones {
        if s == 0 {
            *out.entry(1).or_default() += count;
        } else if let Some((l, r)) = split(s) {
            *out.entry(l).or_default() += count;
            *out.entry(r).or_default() += count;
        } else {
            *out.entry(s.checked_mul(2024).unwrap()).or_default() += count;
        }
    }
}

pub fn part1(input: &str) -> String {
    let mut stones = count(&parse_input(input));
    let mut prev = AHashMap::default();
    for _ in 0..25 {
        std::mem::swap(&mut stones, &mut prev);
        stones.clear();
        blink_count(&prev, &mut stones);
    }

    stones.values().sum::<usize>().to_string()
}

pub fn part2(input: &str) -> String {
    let mut stones = count(&parse_input(input));
    let mut prev = AHashMap::default();
    for _ in 0..75 {
        std::mem::swap(&mut stones, &mut prev);
        stones.clear();
        blink_count(&prev, &mut stones);
    }

    stones.values().sum::<usize>().to_string()
}
