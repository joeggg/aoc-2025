use std::fs;

use aoc_tools::run_solution;

fn main() {
    let banks = read_input();
    run_solution(|| get_total_joltage(banks), 2);
}

fn get_total_joltage(banks: Vec<Vec<u8>>) -> u64 {
    let mut total = 0;
    for bank in banks.into_iter() {
        let mut digits_found = vec![];
        let mut space_used = 0;
        for i in 0..12 {
            // Remove extra 1 to account for the current digit
            let digits_left = 12 - i - 1;
            // Get free space to the left of the remaining digits if we pack them to the right
            // Start from the index of the last digit we added
            let free_space = bank.len() - space_used - digits_left;
            // Find max digit in space
            let (i, max) = bank
                .iter()
                .enumerate()
                .skip(space_used)
                .take(free_space)
                .rev() // Reverse so we get the first max rather
                // than last
                .max_by_key(|&(_, item)| item)
                .unwrap();
            // Update space used, add 1 to index so it equals the number of elements up to that
            // point
            space_used = i + 1;
            digits_found.push(max);
        }
        let mut num_str = String::new();
        for digit in digits_found {
            num_str.push_str(&digit.to_string());
        }
        total += num_str.parse::<u64>().unwrap();
    }
    total
}

fn read_input() -> Vec<Vec<u8>> {
    fs::read_to_string("day-3/input.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}
