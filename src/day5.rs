use ahash::{AHashMap, AHashSet};
use itertools::Itertools;

struct Input {
    rules: AHashMap<usize, AHashSet<usize>>,
    updates: Vec<Vec<usize>>,
}

fn parse_input(input: &str) -> Input {
    let (r, u) = input.split_once("\n\n").unwrap();
    let mut rules = AHashMap::<usize, AHashSet<usize>>::new();
    for (a, b) in r.lines().map(|l| {
        let (a, b) = l.split_once('|').unwrap();
        (a.parse().unwrap(), b.parse().unwrap())
    }) {
        rules.entry(a).or_default().insert(b);
    }

    let updates = u
        .lines()
        .map(|l| l.split(',').map(|n| n.parse().unwrap()).collect_vec())
        .collect_vec();

    Input { rules, updates }
}

pub fn part1(input: &str) -> String {
    let Input { rules, updates } = parse_input(input);

    updates
        .iter()
        .filter(|u| {
            for i in 0..u.len() {
                let Some(rule) = rules.get(&u[i]) else {
                    continue;
                };

                if u[..i].iter().any(|e| rule.contains(e)) {
                    return false;
                }
            }

            true
        })
        .map(|u| u[u.len() / 2])
        .sum::<usize>()
        .to_string()
}

fn sort_topo(update: &mut [usize], rules: &AHashMap<usize, AHashSet<usize>>) {
    let mut i = 0;
    while i < update.len() {
        let Some(rule) = rules.get(&update[i]) else {
            i += 1;
            continue;
        };

        if let Some(j) = update[..i].iter().position(|j| rule.contains(j)) {
            update.swap(i, j);
            i = j;
        }

        i += 1;
    }
}

pub fn part2(input: &str) -> String {
    let Input { rules, updates } = parse_input(input);

    updates
        .into_iter()
        .filter(|u| {
            for i in 0..u.len() {
                let Some(rule) = rules.get(&u[i]) else {
                    continue;
                };

                if u[..i].iter().any(|e| rule.contains(e)) {
                    return true;
                }
            }

            false
        })
        .map(|mut u| {
            sort_topo(&mut u, &rules);
            u
        })
        .map(|u| u[u.len() / 2])
        .sum::<usize>()
        .to_string()
}
