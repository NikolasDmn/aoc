use aoc;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashSet;

const EXPECTED: usize = 40;

aoc::solution!(EXPECTED, solve);

fn solve(input: &str) -> usize {
    let mut all_nodes = input
        .lines()
        .map(|line| {
            let coords: Vec<i32> = line.split(',').map(|s| s.parse().unwrap()).collect();
            (coords[0], coords[1], coords[2])
        })
        .tuple_combinations()
        .map(|((x1, y1, z1), (x2, y2, z2))| {
            let x_diff = x1 as f64 - x2 as f64;
            let y_diff = y1 as f64 - y2 as f64;
            let z_diff = z1 as f64 - z2 as f64;

            let dist = (x_diff.powi(2) + y_diff.powi(2) + z_diff.powi(2)).sqrt();
            ((x1, y1, z1), (x2, y2, z2), dist)
        })
        .collect::<Vec<((i32, i32, i32), (i32, i32, i32), f64)>>();
    let nodes = if all_nodes.len() < 1000 {
        all_nodes
            .select_nth_unstable_by(10, |a, b| a.2.total_cmp(&b.2))
            .0
    } else {
        all_nodes
            .select_nth_unstable_by(1000, |a, b| a.2.total_cmp(&b.2))
            .0
    };
    nodes.sort_by(|a, b| a.2.total_cmp(&b.2));
    nodes
        .iter()
        .fold(vec![], |acc: Vec<HashSet<(i32, i32, i32)>>, (c1, c2, _)| {
            let (to_merge, mut others): (Vec<_>, Vec<_>) = acc
                .into_iter()
                .partition(|set| set.contains(c1) || set.contains(c2));

            let mut new_group = HashSet::new();
            new_group.insert(*c1);
            new_group.insert(*c2);

            for set in to_merge {
                new_group.extend(set);
            }

            others.push(new_group);
            others
        })
        .into_iter()
        .map(|c| c.len())
        .collect::<Vec<usize>>()
        .select_nth_unstable_by(3, |a, b| b.cmp(a))
        .0
        .iter()
        .product()
}
