use regex::{Captures, Regex};

pub fn part_one(input: &str) -> Option<String> {
    let parsed = parse(input);
    let mut stacks = parsed.stacks;
    for (num, from, to) in parsed.instructions {
        for _ in 0..num {
            let popped = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(popped);
        }
    }

    Some(top_of_stacks(stacks))
}

pub fn part_two(input: &str) -> Option<String> {
    let parsed = parse(input);
    let mut stacks = parsed.stacks;
    for (num, from, to) in parsed.instructions {
        let split = stacks[from - 1].len() - num;
        let mut picked_up = stacks[from - 1].split_off(split);
        stacks[to - 1].append(&mut picked_up);
    }

    Some(top_of_stacks(stacks))
}

struct CrateInstructions {
    stacks: Vec<Vec<char>>,
    instructions: Vec<(usize, usize, usize)>,
}

fn parse(input: &str) -> CrateInstructions {
    let (stack_str, instr_str) = input.split_once("\n\n").unwrap();

    let mut stacks: Vec<Vec<char>> = Vec::new();
    let orders_regex = Regex::new(r"\[(?P<name>[A-Za-z])\]").unwrap();
    for line in stack_str.split_terminator('\n') {
        let mut i = 0;
        loop {
            let crate_str = line.chars().skip(i * 4).take(4).collect::<String>();
            if crate_str.is_empty() {
                break;
            }

            if stacks.len() == i {
                stacks.push(Vec::new());
            }
            match orders_regex.captures(&crate_str) {
                None => {}
                Some(captures) => {
                    let name = &captures["name"].chars().next().unwrap();
                    stacks[i].insert(0, *name);
                }
            }
            i += 1;
        }
    }

    let mut instructions = Vec::new();
    let orders_regex = Regex::new(r"move (?P<num>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
    let parse_as_num =
        |captures: &Captures, group: &str| -> usize { captures[group].parse::<usize>().unwrap() };
    for i in instr_str.split_terminator('\n') {
        let caps = orders_regex.captures(i).unwrap();

        let num = parse_as_num(&caps, "num");
        let from = parse_as_num(&caps, "from");
        let to = parse_as_num(&caps, "to");

        instructions.push((num, from, to));
    }

    CrateInstructions {
        stacks,
        instructions,
    }
}

fn top_of_stacks(stacks: Vec<Vec<char>>) -> String {
    stacks.iter().flat_map(|i| i.last()).collect::<String>()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
