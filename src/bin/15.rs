use std::collections::HashSet;

use itertools::Itertools;
use regex::Regex;

pub fn part_one(input: &str) -> Option<i128> {
    let sensors = parse(input);
    let y_val = 2000000;

    let no_beacons = find_intervals(&sensors, y_val)
        .iter()
        .map(|(x, y)| y - x)
        .sum::<i128>();

    let num_beacons_at_y = sensors
        .iter()
        .map(|s| s.beacon)
        .collect::<HashSet<(i128, i128)>>()
        .iter()
        .filter(|(_, sy)| *sy == y_val)
        .collect_vec()
        .len();

    Some(no_beacons - num_beacons_at_y as i128)
}

pub fn part_two(input: &str) -> Option<i128> {
    let sensors = parse(input);
    let max_val = 4000000;
    let mut answer = None;
    for y in 0..(max_val + 1) {
        let no_beacons = find_intervals(&sensors, y);
        if no_beacons.len() == 2 {
            answer = Some(no_beacons[0].1 * 4000000 + y);
        }
    }
    answer
}

struct Sensor {
    sensor: (i128, i128),
    beacon: (i128, i128),
}

fn find_intervals(sensors: &[Sensor], y_val: i128) -> Vec<(i128, i128)> {
    let mut intervals = sensors
        .iter()
        .filter_map(
            |Sensor {
                 sensor: (sx, sy),
                 beacon: (bx, by),
             }| {
                let dist =
                    (sx.abs_diff(*bx) + sy.abs_diff(*by)) as i128 - (sy.abs_diff(y_val) as i128);
                if dist > 0 {
                    return Some((sx - dist, sx + dist + 1));
                }
                None
            },
        )
        .collect_vec();
        
    intervals.sort_by(|(x1, _), (x2, _)| x1.cmp(x2));

    let mut intervals_iter = intervals.iter();
    let mut smashed_intervals = vec![*intervals_iter.next().unwrap()];
    for inserting in intervals_iter {
        let (_, last_y) = smashed_intervals.last_mut().unwrap();

        if inserting.0 > *last_y {
            //Interval is strictly larger
            smashed_intervals.push(*inserting);
        } else if *last_y >= inserting.0 && *last_y <= inserting.1 {
            //Interval overlaps so extend the existing interval
            *last_y = inserting.1;
        }
    }
    smashed_intervals
}

fn parse(input: &str) -> Vec<Sensor> {
    Regex::new(r"Sensor at x=(?P<x>-?[0-9]+), y=(?P<y>-?[0-9]+): closest beacon is at x=(?P<bx>-?[0-9]+), y=(?P<by>-?[0-9]+)")
        .unwrap()
        .captures_iter(input)
        .map(|caps| Sensor
            { sensor: (
                    caps["x"].parse::<i128>().unwrap(),
                    caps["y"].parse::<i128>().unwrap(),
                )
            , beacon:(
                    caps["bx"].parse::<i128>().unwrap(),
                    caps["by"].parse::<i128>().unwrap(),
                )
            }
        )
        .collect_vec()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(56000011));
    }
}
