use std::{collections::HashSet, iter};

pub fn part_one(input: &str) -> Option<usize> {
    let initial_state = RopeState {
        knots: vec![(0, 0), (0, 0)],
        tail_moves: vec![(0, 0)],
    };

    Some(
        input
            .split_terminator('\n')
            .fold(initial_state, mv)
            .tail_moves
            .iter()
            .collect::<HashSet<&(i32, i32)>>()
            .len(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let initial_state = RopeState {
        knots: iter::repeat((0, 0)).take(10).collect(),
        tail_moves: vec![(0, 0)],
    };

    Some(
        input
            .split_terminator('\n')
            .fold(initial_state, mv)
            .tail_moves
            .iter()
            .collect::<HashSet<&(i32, i32)>>()
            .len(),
    )
}

struct RopeState {
    knots: Vec<(i32, i32)>,
    tail_moves: Vec<(i32, i32)>,
}

fn mv(RopeState { knots, tail_moves }: RopeState, mv: &str) -> RopeState {
    let mut s = mv.split_terminator(' ');
    let m = s.next().unwrap();
    let n = s.next().unwrap().parse::<i32>().unwrap();
    let mut steps = tail_moves;

    let do_mv = |head_fn: &dyn Fn((i32, i32)) -> (i32, i32)| {
        (0..n).fold(knots, |ks, _b| {
            let mut ks_iter = ks.iter();
            let mut new_ks = vec![];
            new_ks.push(head_fn(*ks_iter.next().unwrap()));
            for k in ks_iter {
                new_ks.push(mv_tail(*new_ks.last().unwrap(), *k))
            }
            steps.push(*new_ks.last().unwrap());
            new_ks
        })
    };

    match m {
        "R" => RopeState {
            knots: do_mv(&|(hx, hy)| (hx + 1, hy)),
            tail_moves: steps,
        },
        "L" => RopeState {
            knots: do_mv(&|(hx, hy)| (hx - 1, hy)),
            tail_moves: steps,
        },
        "U" => RopeState {
            knots: do_mv(&|(hx, hy)| (hx, hy + 1)),
            tail_moves: steps,
        },
        "D" => RopeState {
            knots: do_mv(&|(hx, hy)| (hx, hy - 1)),
            tail_moves: steps,
        },
        x => panic!("Bad move: {:?}", x),
    }
}

fn mv_tail((hx, hy): (i32, i32), (tx, ty): (i32, i32)) -> (i32, i32) {
    let mx = hx - tx;
    let my = hy - ty;

    match (mx, my) {
        (2, 0) => (tx + 1, ty),
        (2, 1) => (tx + 1, ty + 1),
        (2, 2) => (tx + 1, ty + 1),
        (2, -1) => (tx + 1, ty - 1),
        (2, -2) => (tx + 1, ty - 1),

        (-2, 0) => (tx - 1, ty),
        (-2, -1) => (tx - 1, ty - 1),
        (-2, -2) => (tx - 1, ty - 1),
        (-2, 1) => (tx - 1, ty + 1),
        (-2, 2) => (tx - 1, ty + 1),

        (0, 2) => (tx, ty + 1),
        (1, 2) => (tx + 1, ty + 1),
        (-1, 2) => (tx - 1, ty + 1),

        (0, -2) => (tx, ty - 1),
        (-1, -2) => (tx - 1, ty - 1),
        (1, -2) => (tx + 1, ty - 1),

        _ => (tx, ty),
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(88));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
