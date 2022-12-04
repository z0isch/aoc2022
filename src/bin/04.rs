fn parse_and_count(input: &str, f: &dyn Fn((u32, u32), (u32, u32)) -> bool) -> u32 {
    input
        .split('\n')
        .filter(|i| {
            let mut line = i.split(',');

            let parse_interval = |x: &str| {
                let mut f = x.split('-');
                (
                    f.next().unwrap().parse::<u32>().unwrap(),
                    f.next().unwrap().parse::<u32>().unwrap(),
                )
            };

            f(
                parse_interval(line.next().unwrap()),
                parse_interval(line.next().unwrap()),
            )
        })
        .count()
        .try_into()
        .unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(parse_and_count(input, &|(a_x, a_y), (b_x, b_y)| {
        // Either interval A fits in B or B fits in A
        (a_x <= b_x && a_y >= b_y) || (b_x <= a_x && b_y >= a_y)
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(parse_and_count(input, &|(a_x, a_y), (b_x, b_y)| {
        // There's only 2 cases where there is no overlap:
        // 1. interval A is completely before B => a_y < b_x
        // 2. interval B is completely before A => b_y < a_x
        !(a_y < b_x || b_y < a_x)
    }))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
