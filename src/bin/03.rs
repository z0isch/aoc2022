use std::{collections::HashSet, hash::Hash};

fn char_priority(c: &char) -> u32 {
    let dec = *c as u32;
    if c.is_uppercase() {
        return dec - 64 + 26;
    }
    dec - 96
}
pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split('\n')
            .map(|l| {
                let length = l.chars().count();
                let first: HashSet<char> = HashSet::from_iter(l.chars().take(length / 2));
                let second = HashSet::from_iter(l.chars().skip(length / 2));
                first.intersection(&second).map(char_priority).sum::<u32>()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut xs = input.split('\n');
    let mut total = 0;
    while let Some(x) = xs.next() {
        let y: HashSet<char> = HashSet::from_iter(xs.next().unwrap().chars());
        let z: HashSet<char> = HashSet::from_iter(xs.next().unwrap().chars());
        let t: HashSet<char> =
            HashSet::from_iter(HashSet::from_iter(x.chars()).intersection(&y).map(|t| *t));
        total += t.intersection(&z).map(char_priority).sum::<u32>();
    }
    Some(total)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
