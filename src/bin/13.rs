use itertools::Itertools;
use pathfinding::directed::topological_sort;

pub fn part_one(input: &str) -> Option<usize> {
    let pairs = input
        .split("\n\n")
        .map(|l| l.split_terminator('\n').map(tokenize).collect_vec())
        .collect_vec();

    Some(
        pairs
            .iter()
            .enumerate()
            .filter(|(_, p)| in_correct_order(&mut p[0].clone(), &mut p[1].clone()))
            .map(|(i, _)| i + 1)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut pairs = input
        .split("\n\n")
        .flat_map(|l| l.split_terminator('\n').map(tokenize).collect_vec())
        .collect_vec();

    let divider1 = vec![
        Token::OpenParen,
        Token::OpenParen,
        Token::Int { i: 2 },
        Token::ClosedParen,
        Token::ClosedParen,
    ];
    let divider2 = vec![
        Token::OpenParen,
        Token::OpenParen,
        Token::Int { i: 6 },
        Token::ClosedParen,
        Token::ClosedParen,
    ];

    pairs.push(divider1.clone());
    pairs.push(divider2.clone());

    Some(
        topological_sort::topological_sort(&pairs, |before| {
            pairs
                .clone()
                .into_iter()
                .filter(|after| {
                    before != after && in_correct_order(&mut before.clone(), &mut after.clone())
                })
                .collect_vec()
        })
        .unwrap()
        .iter()
        .enumerate()
        .filter(|&(_, p)| divider1 == *p || divider2 == *p)
        .map(|(i, _)| i + 1)
        .product(),
    )
}

fn in_correct_order(packet1: &mut Vec<Token>, packet2: &mut Vec<Token>) -> bool {
    let mut i = 0;
    loop {
        match (packet1[i], packet2[i]) {
            (Token::OpenParen, Token::OpenParen) => i += 1,
            (Token::ClosedParen, Token::ClosedParen) => i += 1,
            (_, Token::ClosedParen) => {
                return false;
            }
            (Token::ClosedParen, _) => {
                return true;
            }
            (Token::OpenParen, Token::Int { .. }) => {
                packet2.insert(i, Token::OpenParen);
                packet2.insert(i + 2, Token::ClosedParen);
            }
            (Token::Int { .. }, Token::OpenParen) => {
                packet1.insert(i, Token::OpenParen);
                packet1.insert(i + 2, Token::ClosedParen);
            }
            (Token::Int { i: i1 }, Token::Int { i: i2 }) => match i1.cmp(&i2) {
                std::cmp::Ordering::Less => {
                    return true;
                }
                std::cmp::Ordering::Equal => {
                    i += 1;
                }
                std::cmp::Ordering::Greater => {
                    return false;
                }
            },
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug, Copy)]
enum Token {
    OpenParen,
    ClosedParen,
    Int { i: u32 },
}

fn tokenize(packet: &str) -> Vec<Token> {
    let mut ret = vec![];
    let mut curr_int: Vec<char> = vec![];
    for c in packet.chars() {
        match c {
            '[' => {
                ret.push(Token::OpenParen);
            }
            ']' => {
                if !curr_int.is_empty() {
                    ret.push(Token::Int {
                        i: curr_int.iter().collect::<String>().parse::<u32>().unwrap(),
                    });
                    curr_int = vec![];
                }
                ret.push(Token::ClosedParen)
            }
            ',' => {
                if !curr_int.is_empty() {
                    ret.push(Token::Int {
                        i: curr_int.iter().collect::<String>().parse::<u32>().unwrap(),
                    });
                    curr_int = vec![];
                }
            }
            x => {
                curr_int.push(x);
            }
        }
    }
    ret
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), None);
    }
}
