use itertools::Itertools;

pub fn part_one(input: &str) -> Option<i32> {
    Some(
        reg_val(input, 20) * 20
            + reg_val(input, 60) * 60
            + reg_val(input, 100) * 100
            + reg_val(input, 140) * 140
            + reg_val(input, 180) * 180
            + reg_val(input, 220) * 220,
    )
}

pub fn part_two(input: &str) -> Option<String> {
    Some(
        (1..241)
            .map(|cycle| {
                let center_sprite = reg_val(input, cycle);
                let pixel_loc = (cycle - 1) % 40;

                if pixel_loc >= (center_sprite - 1) as usize
                    && pixel_loc <= (center_sprite + 1) as usize
                {
                    return '#';
                }
                '.'
            })
            .into_iter()
            .chunks(40)
            .into_iter()
            .map(|x| x.collect::<String>())
            .join("\n"),
    )
}

fn reg_val(input: &str, cycle: usize) -> i32 {
    return 1 + input
        .split_terminator('\n')
        .flat_map(|l| match l {
            "noop" => vec![0],
            x => {
                vec![
                    0,
                    x.split(' ').collect::<Vec<&str>>()[1]
                        .parse::<i32>()
                        .unwrap(),
                ]
            }
        })
        .take(cycle - 1)
        .sum::<i32>();
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(
            part_two(&input),
            Some(String::from(
                "##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######....."
            ))
        );
    }
}
