use std::{collections::HashSet, fs};

use aoc_tools::run_solution;

fn main() {
    let tiles = read_input();
    run_solution(|| get_largest_rectangle_area(tiles), 9);
}

fn get_largest_rectangle_area(tiles: Vec<(u64, u64)>) -> u64 {
    let mut borders: HashSet<(u64, u64)> = HashSet::new();
    for (i, (x_1, y_1)) in tiles.iter().enumerate() {
        for (x_2, y_2) in tiles.iter().skip(i + 1) {
            if x_1 == x_2 {
                let upper = if y_1 > y_2 { y_1 } else { y_2 };
                let lower = if y_1 < y_2 { y_1 } else { y_2 };
                for i in (*lower + 1)..*upper {
                    borders.insert((*x_1, i));
                }
            } else if y_1 == y_2 {
                let upper = if x_1 > x_2 { x_1 } else { x_2 };
                let lower = if x_1 < x_2 { x_1 } else { x_2 };
                for i in (*lower + 1)..*upper {
                    borders.insert((i, *y_1));
                }
            }
        }
    }
    let mut max = 0;
    for (i, (x_1, y_1)) in tiles.iter().enumerate() {
        for (x_2, y_2) in tiles.iter().skip(i + 1) {
            // +1 to include both tiles
            let dx = x_1.abs_diff(*x_2) + 1;
            let dy = y_1.abs_diff(*y_2) + 1;
            let area = dy * dx;
            if area > max {
                max = area;
            }
        }
    }
    max
}

fn read_input() -> Vec<(u64, u64)> {
    fs::read_to_string("day-9/input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}
