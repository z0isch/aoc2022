use regex::Regex;
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<usize> {
    let rocks = parse(input);

    let lowest_y = rocks.iter().map(|r| r.1).max().unwrap();
    let mut sand = (500, 0);
    let mut sands: HashSet<(u32, u32)> = HashSet::new();
    while sand.1 <= lowest_y {
        let down_sand = (sand.0, sand.1 + 1);
        let down_left = (sand.0 - 1, sand.1 + 1);
        let down_right = (sand.0 + 1, sand.1 + 1);
        if !sands.contains(&down_sand) && !rocks.contains(&down_sand) {
            sand = down_sand;
        } else if !sands.contains(&down_left) && !rocks.contains(&down_left) {
            sand = down_left;
        } else if !sands.contains(&down_right) && !rocks.contains(&down_right) {
            sand = down_right;
        } else {
            sands.insert(sand);
            sand = (500, 0);
        }
    }
    Some(sands.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let rocks = parse(input);

    let lowest_y = rocks.iter().map(|r| r.1).max().unwrap();
    let mut sand = (500, 0);
    let mut sands: HashSet<(u32, u32)> = HashSet::new();
    loop {
        let down_sand = (sand.0, sand.1 + 1);
        let down_left = (sand.0 - 1, sand.1 + 1);
        let down_right = (sand.0 + 1, sand.1 + 1);
        if sand.1 + 1 < lowest_y + 2 {
            if !sands.contains(&down_sand) && !rocks.contains(&down_sand) {
                sand = down_sand;
            } else if !sands.contains(&down_left) && !rocks.contains(&down_left) {
                sand = down_left;
            } else if !sands.contains(&down_right) && !rocks.contains(&down_right) {
                sand = down_right;
            } else {
                sands.insert(sand);
                if sand == (500, 0) {
                    break;
                }
                sand = (500, 0);
            }
        } else {
            sands.insert(sand);
            if sand == (500, 0) {
                break;
            }
            sand = (500, 0);
        }
    }
    Some(sands.len())
}

fn parse(input: &str) -> HashSet<(u32, u32)> {
    input
        .split_terminator('\n')
        .flat_map(|l| {
            let r = Regex::new(r"(?P<x>[0-9]+),(?P<y>[0-9]+)").unwrap();
            let mut coords = r.captures_iter(l).map(|caps| {
                (
                    caps["x"].parse::<u32>().unwrap(),
                    caps["y"].parse::<u32>().unwrap(),
                )
            });

            let (mut x1, mut y1) = coords.next().unwrap();
            let mut rocks = vec![(x1, y1)];
            for (x2, y2) in coords {
                let mut xn = x1 + 1;
                while xn < x2 {
                    rocks.push((xn, y1));
                    xn += 1;
                }
                let mut yn = y1 + 1;
                while yn < y2 {
                    rocks.push((x1, yn));
                    yn += 1;
                }
                let mut xn = x1 - 1;
                while x2 < xn {
                    rocks.push((xn, y1));
                    xn -= 1;
                }
                let mut yn = y1 - 1;
                while y2 < yn {
                    rocks.push((x1, yn));
                    yn -= 1;
                }
                rocks.push((x2, y2));
                x1 = x2;
                y1 = y2;
            }
            rocks
        })
        .collect::<HashSet<(u32, u32)>>()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
