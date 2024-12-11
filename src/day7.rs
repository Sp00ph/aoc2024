struct Row {
    expected: usize,
    values: Vec<usize>,
}

fn parse_input(input: &str) -> Vec<Row> {
    input
        .lines()
        .map(|l| {
            let (e, v) = l.split_once(": ").unwrap();
            Row {
                expected: e.parse().unwrap(),
                values: v.split(' ').map(|v| v.parse().unwrap()).collect(),
            }
        })
        .collect()
}

fn concat(l: usize, r: usize) -> Option<usize> {
    let num_digits = r.ilog10() + 1;
    l.checked_mul(10usize.checked_pow(num_digits)?)?
        .checked_add(r)
}

fn is_possible(r: &Row, allow_concat: bool) -> bool {
    fn rec(expected: usize, acc: Option<usize>, remaining: &[usize], allow_concat: bool) -> bool {
        let Some(acc) = acc else {
            return false;
        };
        if acc > expected {
            return false;
        }
        if remaining.is_empty() {
            return acc == expected;
        }

        let (&head, rest) = remaining.split_first().unwrap();
        rec(expected, acc.checked_add(head), rest, allow_concat)
            || rec(expected, acc.checked_mul(head), rest, allow_concat)
            || (allow_concat && rec(expected, concat(acc, head), rest, allow_concat))
    }

    rec(r.expected, Some(0), &r.values, allow_concat)
}

pub fn part1(input: &str) -> String {
    let rows = parse_input(input);

    rows.iter()
        .filter(|r| is_possible(r, false))
        .map(|r| r.expected)
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &str) -> String {

    let rows = parse_input(input);

    rows.iter()
        .filter(|r| is_possible(r, true))
        .map(|r| r.expected)
        .sum::<usize>()
        .to_string()
}
