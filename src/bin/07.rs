use std::{collections::HashMap, ptr::null_mut};

use regex::Regex;

struct File {
    size: u32,
}

struct Dir {
    files: Vec<File>,
    dirs: HashMap<String, Dir>,
    up: *mut Dir,
}

enum Cmd {
    CdDir { dir: String },
    CdUp,
    Ls { files: Vec<File>, dirs: Vec<String> },
}

pub fn part_one(input: &str) -> Option<u32> {
    let cmds = parse(input);
    let root = mk_root(cmds);

    Some(get_sizes(root).1.into_iter().filter(|s| s <= &100000).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let cmds = parse(input);
    let root = mk_root(cmds);

    let (root_size, sizes) = get_sizes(root);
    let min_size_needed_to_delete = 30000000 - (70000000 - root_size);

    let mut possibles = sizes
        .into_iter()
        .filter(|s| s >= &min_size_needed_to_delete)
        .collect::<Vec<u32>>();
    possibles.sort();
    Some(possibles[0])
}

fn get_sizes(root: Dir) -> (u32, Vec<u32>) {
    let mut sizes: Vec<u32> = vec![];
    fn total_size(sizes: &mut Vec<u32>, dir: Dir) -> u32 {
        let mut size = dir.files.iter().map(|f| f.size).sum::<u32>();
        for (_, d) in dir.dirs {
            size += total_size(sizes, d)
        }
        sizes.push(size);
        size
    }
    let root_size = total_size(&mut sizes, root);
    (root_size, sizes)
}

fn mk_root(cmds: Vec<Cmd>) -> Dir {
    let mut root = Dir {
        files: vec![],
        dirs: HashMap::new(),
        up: null_mut(),
    };

    let mut curr: *mut Dir = &mut root;
    for c in cmds {
        match c {
            Cmd::Ls { files, dirs } => {
                unsafe { (*curr).files = files }

                let dir_map = dirs
                    .iter()
                    .map(|d| {
                        (
                            d.clone(),
                            Dir {
                                files: vec![],
                                dirs: HashMap::new(),
                                up: curr,
                            },
                        )
                    })
                    .collect::<HashMap<String, Dir>>();

                unsafe { (*curr).dirs = dir_map }
            }
            Cmd::CdDir { dir } => curr = unsafe { (*curr).dirs.get_mut(&dir).unwrap() },
            Cmd::CdUp => curr = unsafe { (*curr).up },
        }
    }
    root
}

fn parse(input: &str) -> Vec<Cmd> {
    let splits = input.split('$').into_iter().skip(2).collect::<Vec<&str>>();
    let mut cmds: Vec<Cmd> = vec![];
    let cd_up = Regex::new(r"cd \.\.").unwrap();
    let cd_dir = Regex::new(r"cd (?P<dir>.+)").unwrap();
    let dir = Regex::new(r"dir (?P<dir>.+)").unwrap();
    let file = Regex::new(r"(?P<size>[0-9]+) (?P<name>.+)").unwrap();
    for s in splits {
        if cd_up.is_match(s) {
            cmds.push(Cmd::CdUp)
        } else if let Some(captures) = cd_dir.captures(s) {
            cmds.push(Cmd::CdDir {
                dir: captures["dir"].to_string(),
            })
        } else {
            let mut dirs = vec![];
            let mut files = vec![];
            for l in s.split('\n') {
                if let Some(captures) = dir.captures(l) {
                    dirs.push(captures["dir"].to_string());
                } else if let Some(captures) = file.captures(l) {
                    files.push(File {
                        size: captures["size"].parse::<u32>().unwrap(),
                    });
                }
            }
            cmds.push(Cmd::Ls { files, dirs });
        }
    }
    cmds
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
