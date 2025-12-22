use std::fs;

use aoc_tools::run_solution;

fn main() {
    let sheet = read_input();
    run_solution(|| get_solutions_total(sheet), 6);

    let sheet = read_input_chars();
    run_solution(|| get_real_solutions_total(sheet), 6);
}

fn get_real_solutions_total(sheet: Vec<Vec<char>>) -> u64 {
    let width = sheet[0].len();
    let mut answers: Vec<u64> = vec![];
    let mut buf: Vec<u64> = vec![];

    for j in (0..width).rev() {
        // Merge number characters
        let mut num = String::new();
        // Descend down a column
        for row in sheet.iter() {
            let entry = row[j];
            match entry {
                ' ' => (),
                // On reaching an operation we consume the current number and those in the buffer
                '+' => {
                    buf.push(num.parse().unwrap());
                    answers.push(buf.iter().sum());
                    buf.clear();
                    num.clear();
                }
                '*' => {
                    buf.push(num.parse().unwrap());
                    answers.push(buf.iter().product());
                    buf.clear();
                    num.clear();
                }
                // Add to current number string
                _ => num.push(entry),
            }
        }
        // If a number only row add to current buffer
        if !num.is_empty() {
            buf.push(num.parse().unwrap());
        }
    }
    answers.iter().sum()
}

fn get_solutions_total(sheet: Vec<Vec<String>>) -> u64 {
    let last_idx = sheet.len() - 1;
    // Get the values to start with
    let mut totals: Vec<u64> = sheet[last_idx]
        .iter()
        .map(|i| if i == "*" { 1 } else { 0 })
        .collect();

    for row in sheet.iter().take(last_idx) {
        for (i, item) in row.iter().enumerate() {
            if sheet[last_idx][i] == "*" {
                totals[i] *= item.parse::<u64>().unwrap();
            } else {
                totals[i] += item.parse::<u64>().unwrap();
            }
        }
    }
    totals.into_iter().sum()
}

fn read_input_chars() -> Vec<Vec<char>> {
    fs::read_to_string("day-6/input.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}

fn read_input() -> Vec<Vec<String>> {
    fs::read_to_string("day-6/input.txt")
        .unwrap()
        .lines()
        .map(|l| l.split_whitespace().map(String::from).collect())
        .collect()
}
