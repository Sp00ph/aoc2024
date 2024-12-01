
pub fn part1(input: &str) -> String {
    let (mut left, mut right) = (vec![], vec![]);
    for line in input.lines() {
        let mut s = line.split_ascii_whitespace();
        left.push(s.next().unwrap().parse::<isize>().unwrap());
        right.push(s.next().unwrap().parse::<isize>().unwrap());
    }
    
    left.sort_unstable();
    right.sort_unstable();
    left.iter().zip(right.iter()).map(|(&i, &j)| (i-j).unsigned_abs()).sum::<usize>().to_string()
}

pub fn part2(input: &str) -> String {
    let (mut left, mut right) = (vec![], vec![]);
    for line in input.lines() {
        let mut s = line.split_ascii_whitespace();
        left.push(s.next().unwrap().parse::<usize>().unwrap());
        right.push(s.next().unwrap().parse::<usize>().unwrap());
    }
    
    left.into_iter().map(|i| i * right.iter().filter(|&&j| i == j).count()).sum::<usize>().to_string()
}