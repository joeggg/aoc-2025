use std::{collections::HashSet, fs};

use aoc_tools::run_solution;

fn main() {
    let (ranges, ids) = read_input();
    run_solution(|| find_number_fresh(ranges, ids), 5);

    let (ranges, _) = read_input();
    run_solution(|| find_all_possible_fresh(ranges), 5);
}

fn find_all_possible_fresh(ranges: Vec<(u64, u64)>) -> u64 {
    let mut new_ranges = HashSet::new();
    for (lower, upper) in ranges.iter() {
        let (mut new_lower, mut new_upper) = (*lower, *upper);
        loop {
            let mut to_remove = None;
            // Check all already checked ranges
            for (other_lower, other_upper) in new_ranges.iter() {
                let mut remove = false;
                // Remove range if smaller than the new one
                if new_lower <= *other_lower && new_upper >= *other_upper {
                    remove = true;
                } else {
                    // If lower end already in a range, extend this range
                    if new_lower >= *other_lower && new_lower <= *other_upper {
                        new_lower = *other_lower;
                        remove = true;
                    }
                    // Same for upper range
                    if new_upper >= *other_lower && new_upper <= *other_upper {
                        new_upper = *other_upper;
                        remove = true;
                    }
                }
                if remove {
                    to_remove = Some((*other_lower, *other_upper));
                    break;
                }
            }
            // Remove the range that we found overlapping
            // Repeat again over all ranges until no more left overlapping
            if let Some(to_remove) = to_remove {
                new_ranges.remove(&to_remove);
            } else {
                break;
            }
        }
        new_ranges.insert((new_lower, new_upper));
    }

    // Sum all the ranges
    new_ranges
        .into_iter()
        // add 1 to the diff due to including both the start and end
        .fold(0, |acc, range| acc + ((range.1 - range.0) + 1))
}

fn find_number_fresh(ranges: Vec<(u64, u64)>, ids: Vec<u64>) -> u32 {
    let mut total = 0;
    for id in ids {
        let mut fresh = false;
        for range in ranges.iter() {
            if id >= range.0 && id <= range.1 {
                fresh = true;
                break;
            }
        }
        if fresh {
            total += 1;
        }
    }
    total
}

fn read_input() -> (Vec<(u64, u64)>, Vec<u64>) {
    let data = fs::read_to_string("day-5/input.txt").unwrap();
    let (ranges, ids) = data.split_once("\n\n").unwrap();
    (
        ranges
            .split("\n")
            .map(|l| {
                let (start, end) = l.split_once("-").unwrap();
                (start.parse().unwrap(), end.parse().unwrap())
            })
            .collect(),
        ids.split("\n")
            .filter_map(|l| {
                if l.is_empty() {
                    None
                } else {
                    Some(l.parse().unwrap())
                }
            })
            .collect(),
    )
}
