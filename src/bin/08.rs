use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<usize> {
    let parsed = parse(input);

    let mut total_vis_trees: HashSet<(usize, usize)> = HashSet::new();

    for x in 0..parsed.len() {
        let mut max_so_far = parsed[x][0];
        let mut vis_trees = HashSet::from([(x, 0)]);

        for y in 0..parsed.len() {
            let f = parsed[x][y];
            if f > max_so_far {
                vis_trees.insert((x, y));
                max_so_far = f
            }
        }
        for t in vis_trees {
            total_vis_trees.insert(t);
        }
    }

    for x in 0..parsed.len() {
        let mut max_so_far = parsed[x][parsed.len() - 1];
        let mut vis_trees = HashSet::from([(x, parsed.len() - 1)]);

        for y in (0..parsed.len()).rev() {
            let f = parsed[x][y];
            if f > max_so_far {
                vis_trees.insert((x, y));
                max_so_far = f
            }
        }
        for t in vis_trees {
            total_vis_trees.insert(t);
        }
    }

    #[allow(clippy::needless_range_loop)]
    for y in 0..parsed.len() {
        let mut max_so_far = parsed[0][y];
        let mut vis_trees = HashSet::from([(0, y)]);

        for x in 0..parsed.len() {
            let f = parsed[x][y];
            if f > max_so_far {
                vis_trees.insert((x, y));
                max_so_far = f
            }
        }
        for t in vis_trees {
            total_vis_trees.insert(t);
        }
    }

    for y in 0..parsed.len() {
        let mut max_so_far = parsed[parsed.len() - 1][y];
        let mut vis_trees = HashSet::from([(parsed.len() - 1, y)]);

        for x in (0..parsed.len()).rev() {
            let f = parsed[x][y];
            if f > max_so_far {
                vis_trees.insert((x, y));
                max_so_far = f
            }
        }
        for t in vis_trees {
            total_vis_trees.insert(t);
        }
    }

    Some(total_vis_trees.len())
}

pub fn part_two(input: &str) -> Option<u32> {
    let parsed = parse(input);
    let scenic_score = |x: usize, y: usize| -> u32 {
        let curr = parsed[x][y];

        let mut a = 0;
        for x1 in x..parsed.len() {
            a += 1;
            if x1 + 1 == parsed.len() - 1 || parsed[x1 + 1][y] >= curr {
                break;
            }
        }

        let mut b = 0;
        for x1 in 0..x {
            b += 1;
            if x1 + 1 == x || parsed[x - x1 - 1][y] >= curr {
                break;
            }
        }

        let mut c = 0;
        for y1 in y..parsed.len() {
            c += 1;
            if y1 + 1 == parsed.len() - 1 || parsed[x][y1 + 1] >= curr {
                break;
            }
        }

        let mut d = 0;
        for y1 in 0..y {
            d += 1;
            if y1 + 1 == y || parsed[x][y - y1 - 1] >= curr {
                break;
            }
        }

        a * b * c * d
    };
    Some(
        (1..(parsed.len() - 1))
            .map(|x| {
                (1..(parsed.len() - 1))
                    .map(|y| scenic_score(x, y))
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap(),
    )
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .split_terminator('\n')
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
