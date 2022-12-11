use std::iter::repeat;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u128> {
    let monkeys = parse(input);

    Some(solve(monkeys, 20, &|x| x / 3))
}

pub fn part_two(input: &str) -> Option<u128> {
    let monkeys = parse(input);

    let all_worries = monkeys.iter().map(|m| m.divisible_by).product::<u128>();

    Some(solve(monkeys, 10000, &|x| x % all_worries))
}

fn solve(
    mut monkeys: Vec<Monkey>,
    num_rounds: u128,
    worry_reduction: &dyn Fn(u128) -> u128,
) -> u128 {
    let mut inspections_per_monkey = repeat(0).take(monkeys.len()).collect::<Vec<u128>>();

    for _ in 0..num_rounds {
        for monkey_num in 0..monkeys.len() {
            let monkeys_mut = &mut monkeys;
            let if_true = monkeys_mut[monkey_num].if_true;
            let if_false = monkeys_mut[monkey_num].if_false;

            while let Some(current_item) = monkeys_mut[monkey_num].items.pop() {
                let worry_level = match monkeys_mut[monkey_num].operation {
                    MonkeyOp::AddNum { num } => current_item + num,
                    MonkeyOp::MultNum { num } => current_item * num,
                    MonkeyOp::AddSelf => current_item + current_item,
                    MonkeyOp::MultSelf => current_item * current_item,
                };
                let new_worry_level = worry_reduction(worry_level);
                if new_worry_level % monkeys_mut[monkey_num].divisible_by == 0 {
                    monkeys_mut[if_true].items.push(new_worry_level);
                } else {
                    monkeys_mut[if_false].items.push(new_worry_level);
                }
                inspections_per_monkey[monkey_num] += 1;
            }
        }
    }

    inspections_per_monkey
        .iter()
        .sorted_by(|a, b| b.cmp(a))
        .take(2)
        .product()
}

fn parse(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|ls| {
            let l = ls.split_terminator('\n').collect::<Vec<&str>>();

            let items = l[1]
                .chars()
                .skip(18)
                .collect::<String>()
                .split(", ")
                .map(|x| x.parse::<u128>().unwrap())
                .collect::<Vec<u128>>();

            let divisible_by = l[3]
                .chars()
                .skip(21)
                .collect::<String>()
                .parse::<u128>()
                .unwrap();

            let mut op = l[2].chars().skip(23);
            let m_num = l[2].chars().skip(25).collect::<String>().parse::<u128>();

            let operation = match op.next() {
                Some('*') => m_num
                    .map(|num| MonkeyOp::MultNum { num })
                    .unwrap_or(MonkeyOp::MultSelf),
                _ => m_num
                    .map(|num| MonkeyOp::AddNum { num })
                    .unwrap_or(MonkeyOp::AddSelf),
            };

            let if_true = l[4]
                .chars()
                .skip(29)
                .collect::<String>()
                .parse::<usize>()
                .unwrap();

            let if_false = l[5]
                .chars()
                .skip(30)
                .collect::<String>()
                .parse::<usize>()
                .unwrap();

            Monkey {
                items,
                operation,
                divisible_by,
                if_false,
                if_true,
            }
        })
        .collect()
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u128>,
    operation: MonkeyOp,
    divisible_by: u128,
    if_true: usize,
    if_false: usize,
}

#[derive(Debug)]
enum MonkeyOp {
    AddNum { num: u128 },
    MultNum { num: u128 },
    AddSelf,
    MultSelf,
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
