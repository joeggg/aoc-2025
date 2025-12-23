use std::{collections::HashSet, fs};

use aoc_tools::run_solution;

fn main() {
    let tiles = read_input();
    run_solution(|| get_largest_rectangle_area(tiles), 9);
}

fn get_largest_rectangle_area(tiles: Vec<(u64, u64)>) -> u64 {
    let mut borders: HashSet<(u64, u64)> = HashSet::new();
    // Record all the tiles that form the border of the green area
    for (i, (x_1, y_1)) in tiles.iter().enumerate() {
        // Include the red tiles
        borders.insert((*x_1, *y_1));
        for (x_2, y_2) in tiles.iter().skip(i + 1) {
            // If 2 red tiles are on the same axis, all tiles between them will be green and part
            // of the border
            if x_1 == x_2 {
                let upper = if y_1 > y_2 { y_1 } else { y_2 };
                let lower = if y_1 < y_2 { y_1 } else { y_2 };
                for n in (*lower + 1)..*upper {
                    borders.insert((*x_1, n));
                }
            } else if y_1 == y_2 {
                let upper = if x_1 > x_2 { x_1 } else { x_2 };
                let lower = if x_1 < x_2 { x_1 } else { x_2 };
                for n in (*lower + 1)..*upper {
                    borders.insert((n, *y_1));
                }
            }
        }
    }
    let mut max = 0;
    let mut count = 0;
    for (i, (x_1, y_1)) in tiles.iter().enumerate() {
        for (x_2, y_2) in tiles.iter().skip(i + 1) {
            println!("{}", count);
            count += 1;
            // Check opposite 2 corners are within the borders
            if !(within_borders(x_1, y_2, &borders) && within_borders(x_2, y_1, &borders)) {
                continue;
            }
            // +1 to include both tiles in area
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

fn within_borders(x: &u64, y: &u64, borders: &HashSet<(u64, u64)>) -> bool {
    if borders.contains(&(*x, *y)) {
        return true;
    }
    let mut below = false;
    let mut above = false;
    let mut left = false;
    let mut right = false;

    for (x_b, y_b) in borders {
        if *x_b == *x && *y_b < *y {
            above = true;
        } else if *x_b == *x && *y_b > *y {
            below = true;
        } else if *x_b < *x && *y_b == *y {
            left = true;
        } else if *x_b > *x && *y_b == *y {
            right = true;
        }

        if (above && below) || (right && left) {
            return true;
        }
    }
    false
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
