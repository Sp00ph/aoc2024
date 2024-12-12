#[derive(Debug, Clone, Copy)]
struct Segment {
    file_id: usize,
    start: usize,
    len: usize,
}

fn parse_input(input: &str) -> Vec<Segment> {
    let input = input.trim();
    let mut offset = 0;
    let mut segs = Vec::with_capacity(input.len() / 2);

    for (i, s) in input.as_bytes().chunks(2).enumerate() {
        let len = (s[0] - b'0') as usize;
        segs.push(Segment {
            file_id: i,
            start: offset,
            len,
        });

        offset += len;
        if let Some(empty) = s.get(1) {
            offset += (*empty - b'0') as usize;
        }
    }

    segs
}

fn compact_part1(mut segs: &[Segment]) -> Vec<Segment> {
    let mut cur = 0;
    let mut compacted = vec![];
    let mut to_copy = {
        let [rest @ .., tail] = segs else {
            return compacted;
        };
        segs = rest;
        *tail
    };

    loop {
        let [next_seg, rest @ ..] = segs else {
            break;
        };

        if cur >= next_seg.start {
            compacted.push(*next_seg);
            segs = rest;
            cur = next_seg.start + next_seg.len;
            continue;
        }

        if to_copy.len == 0 {
            let [rest @ .., tail] = segs else {
                break;
            };
            segs = rest;
            to_copy = *tail;
            continue;
        }

        let scratch_space = next_seg.start - cur;
        let copy_len = scratch_space.min(to_copy.len);
        compacted.push(Segment {
            file_id: to_copy.file_id,
            start: cur,
            len: copy_len,
        });
        cur += copy_len;
        to_copy.len -= copy_len;
    }

    if to_copy.len != 0 {
        compacted.push(Segment {
            start: cur,
            ..to_copy
        });
    }

    compacted
}

fn checksum(seg: &Segment) -> usize {
    // we want to calculate
    // (seg.start..seg.start+seg.len).map(|block| block * seg.file_id).sum()
    // = seg.start * seg.len * seg.file_id + (0..seg.len).map(|block| block * seg.file_id).sum()
    // = seg.file_id * (seg.start * seg.len + (0..seg.len).sum())
    // = seg.file_id * (seg.start * seg.len + seg.len * (seg.len - 1) / 2)
    // = seg.file_id * seg.len * (2 * seg.start + seg.len - 1) / 2

    seg.file_id * seg.len * (2 * seg.start + seg.len - 1) / 2
}

pub fn part1(input: &str) -> String {
    compact_part1(&parse_input(input))
        .iter()
        .map(checksum)
        .sum::<usize>()
        .to_string()
}

fn compact_part2(segs: &[Segment]) -> Vec<Segment> {
    let mut compacted = segs.to_vec();

    let mut num_swaps = 0;

    for (seg_idx, seg) in segs.iter().enumerate().skip(1).rev() {
        let gap_idx = (0..seg_idx + num_swaps).position(|i| {
            compacted[i + 1].start - (compacted[i].start + compacted[i].len) >= seg.len
        });
        let Some(gap_idx) = gap_idx else {
            continue;
        };

        let orig_seg_idx = compacted
            .iter()
            .position(|s| s.file_id == seg.file_id)
            .unwrap();
        let new_start = compacted[gap_idx].start + compacted[gap_idx].len;
        if gap_idx < orig_seg_idx {
            compacted[gap_idx + 1..=orig_seg_idx].rotate_right(1);
            compacted[gap_idx + 1].start = new_start;
        }

        num_swaps += 1;
    }

    compacted
}

pub fn part2(input: &str) -> String {
    compact_part2(&parse_input(input))
        .iter()
        .map(checksum)
        .sum::<usize>()
        .to_string()
}
