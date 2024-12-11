use ahash::AHashMap;
use memchr::memchr_iter;

struct Grid {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Grid {
    fn parse(input: &str) -> Grid {
        let (mut width, mut height) = (0, 0);
        let mut data = vec![];
        for line in input.lines() {
            width = line.len();
            data.extend(line.bytes().map(|b| b.wrapping_sub(b'0')));
            height += 1;
        }

        Grid { data, width, height }
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        assert!(x < self.width && y < self.height);

        self.data[y * self.width + x]
    }
}

fn tile_score(g: &Grid, x: usize, y: usize, part2: bool) -> usize {
    fn dfs(g: &Grid, x: usize, y: usize, level: u8, peaks: &mut AHashMap<(usize, usize), usize>) {
        if level == 9 {
            *peaks.entry((x, y)).or_default() += 1;
            return;
        }
        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
            if nx < g.width && ny < g.height && g.get(nx, ny) == level + 1 {
                dfs(g, nx, ny, level + 1, peaks);
            }
        }
    }

    if g.get(x, y) != 0 {
        0
    } else {
        let mut peaks = AHashMap::new();
        dfs(g, x, y, 0, &mut peaks);
        if !part2 {
            peaks.len()
        } else {
            peaks.values().sum()
        }
    }
}

pub fn part1(input: &str) -> String {
    let grid = Grid::parse(input);

    memchr_iter(0, &grid.data)
        .map(|i| tile_score(&grid, i % grid.width, i / grid.width, false))
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let grid = Grid::parse(input);

    memchr_iter(0, &grid.data)
        .map(|i| tile_score(&grid, i % grid.width, i / grid.width, true))
        .sum::<usize>()
        .to_string()
}
