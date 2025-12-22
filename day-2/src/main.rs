use std::fs;

use aoc_tools::run_solution;

fn main() {
    let ranges = read_input();
    run_solution(|| get_invalid_id_sum(ranges), 2);
}

fn get_invalid_id_sum(ranges: Vec<(u64, u64)>) -> u64 {
    let mut total = 0;
    for (lower, upper) in ranges.into_iter() {
        for num in lower..=upper {
            let num_str = num.to_string();
            let num_chars: Vec<char> = num_str.chars().collect();
            // Largest possible repeat is half the length (or lower if not even)
            let max_pattern = num_chars.len() / 2;
            let mut found = false;
            for size in 1..=max_pattern {
                // Skip patterns that can't fit
                if !num_chars.len().is_multiple_of(size) {
                    continue;
                }
                if contains_repeated_pattern_of_size(&num_chars, size) {
                    found = true;
                    break;
                }
            }
            if found {
                total += num;
            }
        }
    }
    total
}

fn contains_repeated_pattern_of_size(num_chars: &[char], size: usize) -> bool {
    let pattern: String = num_chars.iter().take(size).collect();
    let num_parts = num_chars.len() / size;
    // Check each segment to see if it matches the first
    for i in 1..num_parts {
        let current: String = num_chars.iter().skip(i * size).take(size).collect();
        if current != pattern {
            return false;
        }
    }
    true
}

fn read_input() -> Vec<(u64, u64)> {
    fs::read_to_string("day-2/input.txt")
        .unwrap()
        .replace("\n", "")
        .replace(" ", "")
        .split(",")
        .map(|range| {
            let (lower, upper) = range.split_once("-").unwrap();
            (lower.parse().unwrap(), upper.parse().unwrap())
        })
        .collect()
}
