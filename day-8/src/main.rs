use std::{collections::HashSet, fs};

use aoc_tools::run_solution;

fn main() {
    let coords = read_input();
    run_solution(|| find_top_3_product(coords), 8);
}

#[derive(Debug, PartialEq)]
struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

fn find_top_3_product(coords: Vec<Point>) -> f64 {
    let mut points_and_distances: Vec<((usize, usize), f64)> = vec![];

    for (i, coord_1) in coords.iter().enumerate() {
        for (j, coord_2) in coords.iter().enumerate().skip(i + 1) {
            // Compute length of vector between the 2 points
            let v = Point {
                x: coord_1.x - coord_2.x,
                y: coord_1.y - coord_2.y,
                z: coord_1.z - coord_2.z,
            };
            let distance = (v.x.powi(2) + v.y.powi(2) + v.z.powi(2)).sqrt();
            points_and_distances.push(((i, j), distance));
        }
    }
    // Sort by descending distance and extract just the indexes into a vector, so that popping a
    // value will return the indexes of the next 2 closest points
    points_and_distances.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    let mut indexes: Vec<(usize, usize)> =
        points_and_distances.into_iter().map(|(i, _)| i).collect();

    let mut circuits: Vec<HashSet<usize>> = vec![];
    let mut last_boxes: Option<(usize, usize)> = None;

    loop {
        if indexes.is_empty() {
            break;
        }
        if circuits.len() == 1 && circuits[0].len() == coords.len() {
            break;
        }
        let (i, j) = indexes.pop().unwrap();
        // Check if boxes we're connecting are already in a circuit
        let i_circuit = circuits.iter().position(|h| h.contains(&i));
        let j_circuit = circuits.iter().position(|h| h.contains(&j));

        match (i_circuit, j_circuit) {
            (Some(i_circuit), Some(j_circuit)) => {
                // Both in same circuit - skip
                if i_circuit == j_circuit {
                    continue;
                }
                // Merge 1 set into the other
                let other = circuits[i_circuit].clone();
                circuits[j_circuit].extend(other);
                circuits.remove(i_circuit);
                last_boxes = Some((i, j));
            }
            // Only 1 has a circuit, add other into it
            (None, Some(j_circuit)) => {
                circuits[j_circuit].insert(i);
                last_boxes = Some((i, j));
            }
            (Some(i_circuit), None) => {
                circuits[i_circuit].insert(j);
                last_boxes = Some((i, j));
            }
            // No existing circuit so create a new set
            (None, None) => {
                circuits.push(HashSet::from([i, j]));
                last_boxes = Some((i, j));
            }
        }
    }
    let (i, j) = last_boxes.unwrap();
    coords[i].x * coords[j].x
}

fn read_input() -> Vec<Point> {
    fs::read_to_string("day-8/input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split(",").collect();
            Point {
                x: parts[0].parse().unwrap(),
                y: parts[1].parse().unwrap(),
                z: parts[2].parse().unwrap(),
            }
        })
        .collect()
}
