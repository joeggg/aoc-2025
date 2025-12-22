use std::fs;

use aoc_tools::run_solution;

fn main() {
    let input = read_input();
    run_solution(|| find_password(input), 1);
}

#[derive(Debug)]
enum Direction {
    Right,
    Left,
}

fn find_password(input: Vec<(Direction, i16)>) -> i16 {
    let mut total = 0;
    let mut current: i16 = 50;
    for (dir, mut num) in input {
        // Ignore any full rotations, but add the 0 crossings to the total
        total += num / 100;
        num %= 100;
        let mut next_val = match dir {
            Direction::Right => current + num,
            Direction::Left => current - num,
        };
        // Check if we have rotated, ignoring if we were already on 0 since then we wouldn't have
        // crossed it
        let mut rotated = false;
        if current != 0 && !(0..100).contains(&next_val) {
            total += 1;
            rotated = true;
        }
        // Wrap around
        next_val = if next_val >= 0 {
            next_val % 100
        } else {
            100 + next_val
        };
        // Check if we landed on 0 without rotating (moving to the left)
        if current != 0 && next_val == 0 && !rotated {
            total += 1
        }
        current = next_val;
    }
    total
}

fn read_input() -> Vec<(Direction, i16)> {
    fs::read_to_string("day-1/input.txt")
        .unwrap()
        .lines()
        .filter_map(|l| {
            let chars: Vec<char> = l.chars().collect();
            if chars.is_empty() {
                return None;
            }
            let direction = if chars[0] == 'R' {
                Direction::Right
            } else {
                Direction::Left
            };
            Some((
                direction,
                chars
                    .into_iter()
                    .skip(1)
                    .collect::<String>()
                    .parse()
                    .unwrap(),
            ))
        })
        .collect()
}
