// use std::ops::RangeInclusive;

use ahash::AHashSet;
use bitvec::vec::BitVec;
use itertools::Itertools;

struct Grid {
    width: usize,
    height: usize,
    matrix: BitVec,
}

impl Grid {
    fn parse(input: &str) -> (Grid, (usize, usize)) {
        let (mut width, mut height) = (0, 0);
        let mut start_pos = (0, 0);
        let mut matrix = BitVec::new();

        for line in input.lines() {
            width = line.len();
            assert!(width < 256 && height < 256);
            for (col, b) in line.bytes().enumerate() {
                if b == b'^' {
                    start_pos = (col, height);
                }
                matrix.push(b == b'#');
            }
            height += 1;
        }

        (
            Grid {
                width,
                height,
                matrix,
            },
            start_pos,
        )
    }

    fn get(&self, x: usize, y: usize) -> bool {
        assert!(x < self.width && y < self.height);
        self.matrix[self.width * y + x]
    }

    fn set(&mut self, x: usize, y: usize, val: bool) {
        assert!(x < self.width && y < self.height);
        self.matrix.set(self.width * y + x, val);
    }
}

pub fn part1(input: &str) -> String {
    let (grid, start_pos) = Grid::parse(input);

    let mut visited = Grid {
        width: grid.width,
        height: grid.height,
        matrix: BitVec::repeat(false, grid.matrix.len()),
    };

    visited.set(start_pos.0, start_pos.1, true);

    let (mut x, mut y) = start_pos;
    let (mut dx, mut dy) = (0, -1);

    loop {
        // if we walk off the top or left edge, this will overflow but the bounds check
        // below still works correctly
        let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
        if nx >= grid.width || ny >= grid.height {
            break;
        }

        if grid.get(nx, ny) {
            (dx, dy) = (-dy, dx);
            continue;
        }

        visited.set(nx, ny, true);
        (x, y) = (nx, ny);
    }

    visited.matrix.count_ones().to_string()
}

#[derive(Debug)]
struct Segment {
    start: (usize, usize),
    dir: (isize, isize),
    end: (usize, usize),
}

impl Segment {
    // fn ranges(&self) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
    //     (
    //         self.start.0.min(self.end.0)..=self.start.0.max(self.end.0),
    //         self.start.1.min(self.end.1)..=self.start.1.max(self.end.1),
    //     )
    // }

    fn coords_iter(&self) -> impl Iterator<Item = (usize, usize)> {
        let (mut x, mut y) = self.start;
        let (dx, dy) = self.dir;
        let end = self.end;
        (x, y) = (x.wrapping_add_signed(-dx), y.wrapping_add_signed(-dy));

        std::iter::from_fn(move || {
            if (x, y) == end {
                return None;
            }
            (x, y) = (x.wrapping_add_signed(dx), (y.wrapping_add_signed(dy)));
            Some((x, y))
        })
    }
}

fn get_segments(grid: &Grid, start_pos: (usize, usize)) -> Vec<Segment> {
    let mut segs = vec![];

    let (mut x, mut y) = start_pos;
    let mut seg_start = (x, y);
    let (mut dx, mut dy) = (0, -1);

    loop {
        // if we walk off the top or left edge, this will overflow but the bounds check
        // below still works correctly
        let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
        if nx >= grid.width || ny >= grid.height {
            segs.push(Segment {
                start: seg_start,
                dir: (dx, dy),
                end: (x, y),
            });
            break;
        }

        if grid.get(nx, ny) {
            segs.push(Segment {
                start: seg_start,
                dir: (dx, dy),
                end: (x, y),
            });
            seg_start = (x, y);
            (dx, dy) = (-dy, dx);
            continue;
        }
        (x, y) = (nx, ny);
    }

    segs
}

fn has_loop(grid: &Grid, start_pos: (usize, usize), dir: (isize, isize)) -> bool {
    let mut visited = AHashSet::new();
    visited.insert((start_pos, dir));

    let (mut x, mut y) = start_pos;
    let (mut dx, mut dy) = dir;

    loop {
        // if we walk off the top or left edge, this will overflow but the bounds check
        // below still works correctly
        let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
        if nx >= grid.width || ny >= grid.height {
            return false;
        }

        if grid.get(nx, ny) {
            (dx, dy) = (-dy, dx);
            continue;
        }

        (x, y) = (nx, ny);
        if !visited.insert(((x, y), (dx, dy))) {
            return true;
        }
    }
}

pub fn part2(input: &str) -> String {
    let (mut grid, start_pos) = Grid::parse(input);
    let segments = get_segments(&grid, start_pos);

    segments
        .iter()
        .flat_map(|s| {
            s.coords_iter().map(|(x, y)| {
                (
                    x.wrapping_add_signed(s.dir.0),
                    y.wrapping_add_signed(s.dir.1),
                )
            })
        })
        .filter(|&(x, y)| {
            x < grid.width && y < grid.width && (x, y) != start_pos && !grid.get(x, y)
        })
        .collect_vec()
        .into_iter()
        .filter(|&(x, y)| {
            grid.set(x, y, true);
            let has_loop = has_loop(&grid, start_pos, (0, -1));
            grid.set(x, y, false);
            has_loop
        })
        .collect::<AHashSet<_>>()
        .len()
        .to_string()

    // for (i, seg) in segments.iter().enumerate() {
    //     for prev in &segments[..i] {
    //         // only consider previous segments that are a right turn away from the current one
    //         if prev.dir == (-seg.dir.1, seg.dir.0) {
    //             let (ox, oy) = if seg.dir.0 == 0 {
    //                 let y = prev.start.1;
    //                 if !seg.ranges().1.contains(&y) {
    //                     continue;
    //                 }

    //                 let goal_x = prev.start.0;

    //                 // obstacle in the way :(
    //                 if (seg.start.0.min(goal_x)..=seg.start.0.max(goal_x)).any(|x| grid.get(x, y)) {
    //                     continue;
    //                 }

    //                 (seg.start.0, y.wrapping_add_signed(seg.dir.1))
    //             } else {
    //                 let x = prev.start.0;
    //                 if !seg.ranges().0.contains(&x) {
    //                     continue;
    //                 }

    //                 let goal_y = prev.start.1;

    //                 // obstacle in the way :(
    //                 if (seg.start.1.min(goal_y)..=seg.start.1.max(goal_y)).any(|y| grid.get(x, y)) {
    //                     continue;
    //                 }

    //                 (x.wrapping_add_signed(seg.dir.0), seg.start.1)
    //             };

    //             if ox >= grid.width || oy >= grid.height {
    //                 continue;
    //             }

    //             if (ox, oy) != start_pos {
    //                 num_candidates += 1;
    //             }
    //         }
    //     }
    // }

    // num_candidates.to_string()
}
