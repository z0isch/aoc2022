use itertools::Itertools;
use pathfinding::prelude::bfs;

pub fn part_one(input: &str) -> Option<usize> {
    let HeightMap {
        grid, start, end, ..
    } = parse(input);
    min_steps(&grid, start, end)
}

pub fn part_two(input: &str) -> Option<usize> {
    let HeightMap {
        grid,
        end,
        possible_starts,
        ..
    } = parse(input);

    Some(
        possible_starts
            .iter()
            .filter_map(|&start| min_steps(&grid, start, end))
            .sorted()
            .collect_vec()[0],
    )
}

fn min_steps(grid: &[Vec<char>], start: (i32, i32), end: (i32, i32)) -> Option<usize> {
    bfs(
        &start,
        |&(x, y)| {
            vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
                .into_iter()
                .filter(|&p| can_go(grid, (x, y), p))
                .collect_vec()
        },
        |&p| p == end,
    )
    .map(|r| r.len() - 1)
}

fn can_go(grid: &[Vec<char>], (x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> bool {
    grid.get(y2 as usize)
        .and_then(|g| g.get(x2 as usize))
        .map(|&to_char| grid[y1 as usize][x1 as usize] as u32 + 1 >= to_char as u32)
        .unwrap_or_default()
}

struct HeightMap {
    grid: Vec<Vec<char>>,
    start: (i32, i32),
    end: (i32, i32),
    possible_starts: Vec<(i32, i32)>,
}

fn parse(input: &str) -> HeightMap {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut possible_starts = vec![];

    let mut grid = input
        .split_terminator('\n')
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'S' {
                start = (x as i32, y as i32);
                grid[y][x] = 'a';
            }
            if grid[y][x] == 'E' {
                end = (x as i32, y as i32);
                grid[y][x] = 'z';
            }
            if grid[y][x] == 'a' {
                possible_starts.push((x as i32, y as i32))
            }
        }
    }
    HeightMap {
        grid,
        start,
        end,
        possible_starts,
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
