pub fn part_one(input: &str) -> Option<u32> {
    let mut highest = 0;
    let mut current = 0;
    for line in input.split('\n') {
        match line {
            "" => current = 0,
            x => {
                let cals: u32 = line.parse().expect(x);
                current = current + cals;
                if current > highest {
                    highest = current
                }
            }
        }
    }
    return Some(highest);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cals = Vec::new();
    let mut current = 0;
    for line in input.split('\n') {
        match line {
            "" => {
                cals.push(current);
                current = 0;
            }
            x => {
                let cals: u32 = line.parse().expect(x);
                current = current + cals;
            }
        }
    }
    cals.sort();
    return Some(cals[cals.len() - 1] + cals[cals.len() - 2] + cals[cals.len() - 3]);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(6));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(10));
    }
}
