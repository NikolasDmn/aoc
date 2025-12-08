use aoc;
use itertools::Itertools;
use rayon::prelude::*;

const EXPECTED: usize = 25272;

aoc::solution!(EXPECTED, solve);

pub fn solve(input: &str) -> usize {
    let points: Vec<[i64; 3]> = input
        .lines()
        .map(|line| {
            let mut nums = line.split(',').map(|s| s.trim().parse::<i64>().unwrap());
            [
                nums.next().unwrap(),
                nums.next().unwrap(),
                nums.next().unwrap(),
            ]
        })
        .collect();

    let n = points.len();

    let mut min_dists = vec![i64::MAX; n];
    let mut parents = vec![0; n];
    let mut visited = vec![false; n];

    min_dists[0] = 0;
    for _ in 0..n {
        let mut best_candidate_index = usize::MAX;
        let mut min_val = i64::MAX;

        for i in 0..n {
            if !visited[i] && min_dists[i] < min_val {
                min_val = min_dists[i];
                best_candidate_index = i;
            }
        }
        if best_candidate_index == usize::MAX {
            break;
        }
        visited[best_candidate_index] = true;
        for neighbor_index in 0..n {
            if !visited[neighbor_index] {
                let (x1, y1, z1) = (points[best_candidate_index][0], points[best_candidate_index][1], points[best_candidate_index][2]);
                let (x2, y2, z2) = (points[neighbor_index][0], points[neighbor_index][1], points[neighbor_index][2]);
                let dx = x1 - x2;
                let dy = y1 - y2;
                let dz = z1 - z2;
                let dist = dx * dx + dy * dy + dz * dz;

                if dist < min_dists[neighbor_index] {
                    min_dists[neighbor_index] = dist;
                    parents[neighbor_index] = best_candidate_index; 
                }
            }
        }
    }

    //Connector piece is the node with the maximum edge weight in the MST
    let max_weight_node = min_dists
        .iter()
        .position_max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    let max_weight_parent = parents[max_weight_node];

    (points[max_weight_node][0] as usize) * (points[max_weight_parent][0] as usize)
}
