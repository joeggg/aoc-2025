use std::fs;

use aoc_tools::run_solution;

fn main() {
    let input = read_input();

    run_solution(|| find_num_accessible_rolls(input), 4);
}

fn find_num_accessible_rolls(mut area: Vec<Vec<char>>) -> u32 {
    let mut total = 0;
    loop {
        let accessible = get_accessible_rolls(&area);
        if accessible.is_empty() {
            break;
        }
        total += accessible.len() as u32;
        for (i, j) in accessible {
            area[i][j] = '.';
        }
    }
    total
}

fn get_accessible_rolls(area: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut coords = vec![];

    for (i, row) in area.iter().enumerate() {
        for (j, entry) in row.iter().enumerate() {
            if *entry != '@' {
                continue;
            }
            let i_int = i as i32;
            let j_int = j as i32;
            // Every adjacent coordinate
            let to_check = [
                (i_int - 1, j_int),
                (i_int + 1, j_int),
                (i_int, j_int - 1),
                (i_int, j_int + 1),
                (i_int - 1, j_int - 1),
                (i_int - 1, j_int + 1),
                (i_int + 1, j_int - 1),
                (i_int + 1, j_int + 1),
            ];
            let mut num_adjacent = 0;
            for (c_i, c_j) in to_check {
                // Out of bounds check
                if c_i < 0 || c_j < 0 || c_i as usize >= area.len() || c_j as usize >= row.len() {
                    continue;
                }
                let entry = area[c_i as usize][c_j as usize];
                if entry == '@' {
                    num_adjacent += 1;
                }
            }
            if num_adjacent < 4 {
                coords.push((i, j));
            }
        }
    }

    coords
}

fn read_input() -> Vec<Vec<char>> {
    fs::read_to_string("day-4/input.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}
