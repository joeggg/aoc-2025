use std::{collections::HashMap, fs};

use aoc_tools::run_solution;

fn main() {
    let diagram = read_input();
    run_solution(|| find_number_splits(diagram), 7);
}

fn find_number_splits(diagram: Vec<Vec<char>>) -> u64 {
    let s_idx = diagram[0].iter().position(|c| *c == 'S').unwrap();
    let width = diagram[0].len();

    // Search front to check on next row, including number of paths that have converged to each
    // column
    let mut cols_to_check: HashMap<usize, u64> = HashMap::from([(s_idx, 1)]);
    let mut num_timelines = 1;

    let mut i = 1;

    while i < diagram.len() {
        for (j, paths) in cols_to_check.clone().into_iter() {
            if diagram[i][j] == '.' {
                continue;
            }
            // Hit a ^
            if j > 0 {
                cols_to_check
                    .entry(j - 1)
                    .and_modify(|num| *num += paths)
                    .or_insert(paths);
            }
            if j < width {
                cols_to_check
                    .entry(j + 1)
                    .and_modify(|num| *num += paths)
                    .or_insert(paths);
            }
            // Double the number of paths that reached the split
            num_timelines += paths;
            cols_to_check.remove(&j);
        }
        i += 1;
    }

    num_timelines
}

fn read_input() -> Vec<Vec<char>> {
    fs::read_to_string("day-7/input.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}
