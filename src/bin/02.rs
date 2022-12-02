pub fn part_one(input: &str) -> Option<u32> {
    let res: u32 = input.split('\n').fold(0, |total, line| match line {
        "A X" => {
            return total + 1 + 3;
        }
        "A Y" => {
            return total + 2 + 6;
        }
        "A Z" => {
            return total + 3 + 0;
        }
        "B X" => {
            return total + 1 + 0;
        }
        "B Y" => {
            return total + 2 + 3;
        }
        "B Z" => {
            return total + 3 + 6;
        }
        "C X" => {
            return total + 1 + 6;
        }
        "C Y" => {
            return total + 2 + 0;
        }
        "C Z" => {
            return total + 3 + 3;
        }
        _ => return total,
    });
    return Some(res);
}

pub fn part_two(input: &str) -> Option<u32> {
    let res: u32 = input.split('\n').fold(0, |total, line| match line {
        "A X" => {
            return total + 3 + 0;
        }
        "A Y" => {
            return total + 1 + 3;
        }
        "A Z" => {
            return total + 2 + 6;
        }
        "B X" => {
            return total + 1 + 0;
        }
        "B Y" => {
            return total + 2 + 3;
        }
        "B Z" => {
            return total + 3 + 6;
        }
        "C X" => {
            return total + 2 + 0;
        }
        "C Y" => {
            return total + 3 + 3;
        }
        "C Z" => {
            return total + 1 + 6;
        }
        _ => return total,
    });
    return Some(res);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
