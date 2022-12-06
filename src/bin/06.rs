use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<usize> {
    Some(find_non_repeat(input, 4))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(find_non_repeat(input, 14))
}

fn find_non_repeat(input: &str, length: usize) -> usize {
    let mut i = 0;
    loop {
        if input[i..i + length]
            .chars()
            .collect::<HashSet<char>>()
            .len()
            == length
        {
            break;
        }
        i += 1;
    }
    i + length
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
