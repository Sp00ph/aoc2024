use ahash::AHashSet;
use itertools::Itertools;

struct Grid {
    width: usize,
    height: usize,
    // dense map from bytes to coord lists. there's some space savings
    // to be had here from only including printable ascii characters,
    // but there's only gonna be one instance of this created anyways
    // so the 6KiB that this uses up shouldn't be an issue.
    antennas: [Vec<(usize, usize)>; 256],
}

impl Grid {
    fn parse(input: &str) -> Grid {
        let (mut width, mut height) = (0, 0);
        let mut antennas = [const { Vec::new() }; 256];

        for line in input.lines() {
            width = line.len();
            for (col, b) in line.bytes().enumerate() {
                if b != b'.' {
                    antennas[b as usize].push((col, height));
                }
            }
            height += 1;
        }

        Grid {
            width,
            height,
            antennas,
        }
    }
}

pub fn part1(input: &str) -> String {
    let grid = Grid::parse(input);

    let mut locations = AHashSet::new();
    for antenna in grid.antennas.iter() {
        for ((ax, ay), (bx, by)) in antenna.iter().copied().tuple_combinations() {
            let (dx, dy) = (bx as isize - ax as isize, by as isize - ay as isize);

            for (cx, cy) in [
                (ax.wrapping_add_signed(-dx), ay.wrapping_add_signed(-dy)),
                (bx.wrapping_add_signed(dx), by.wrapping_add_signed(dy)),
            ] {
                if cx < grid.width && cy < grid.height {
                    locations.insert((cx, cy));
                }
            }
        }
    }

    locations.len().to_string()
}

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 { a } else { gcd(b, a % b) }
}

pub fn part2(input: &str) -> String {
    let grid = Grid::parse(input);

    let mut locations = AHashSet::new();
    for antenna in grid.antennas.iter() {
        for ((ax, ay), (bx, by)) in antenna.iter().copied().tuple_combinations() {
            let (dx, dy) = (bx as isize - ax as isize, by as isize - ay as isize);
            let gcd = gcd(dx, dy);
            let (dx, dy) = (dx / gcd, dy / gcd);
            // (dx, dy) is now the smallest step size one can take from one of the antennas
            // and land on a grid position again.

            // walk backwards from a and forwards from b until we leave the grid and mark
            // all grid positions along those directions as antinodes.
            for ((mut x, mut y), (dx, dy)) in [((ax, ay), (-dx, -dy)), ((bx, by), (dx, dy))] {
                while x < grid.width && y < grid.height {
                    locations.insert((x, y));
                    (x, y) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
                }
            }
        }
    }

    locations.len().to_string()
}
