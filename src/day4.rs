use itertools::Itertools;
use memchr::memchr_iter;

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let data = input.lines().flat_map(str::bytes).collect_vec();
        let width = input.find('\n').unwrap();
        let height = data.len() / width;

        Self {
            width,
            height,
            data,
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<u8> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(self.data[y * self.width + x])
        }
    }
}

pub fn part1(input: &str) -> String {
    let grid = Grid::parse(input);

    memchr_iter(b'X', &grid.data)
        .map(|i| (i % grid.width, i / grid.width))
        .map(|(x, y)| {
            (-1..=1)
                .cartesian_product(-1..=1)
                .filter(|&(dx, dy)| {
                    (dx, dy) != (0, 0)
                        && (1..=3)
                            .map(|i| {
                                grid.get(
                                    x.wrapping_add_signed(dx * i),
                                    y.wrapping_add_signed(dy * i),
                                )
                            })
                            .eq(b"MAS".iter().copied().map(Some))
                })
                .count()
        })
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let grid = Grid::parse(input);

    memchr_iter(b'A', &grid.data)
        .map(|i| (i % grid.width, i / grid.width))
        .filter(|(x, y)| {
            [(-1, -1), (1, -1), (-1, 1), (1, 1)]
                .into_iter()
                .map(|(dx, dy)| (dx as usize, dy as usize))
                .any(|(dx, dy)| {
                    grid.get(x.wrapping_sub(dx), y.wrapping_sub(dy)) == Some(b'M')
                        && grid.get(x.wrapping_add(dx), y.wrapping_add(dy)) == Some(b'S')
                        && grid.get(x.wrapping_add(dy), y.wrapping_sub(dx)) == Some(b'M')
                        && grid.get(x.wrapping_sub(dy), y.wrapping_add(dx)) == Some(b'S')
                })
        })
        .count()
        .to_string()
}
