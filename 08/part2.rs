use aoc;
use itertools::Itertools;
use std::collections::HashSet;

const EXPECTED: usize = 25272;

aoc::solution!(EXPECTED, solve);

fn solve(input: &str) -> usize {
    let nodes = input.lines().map(|line| {
        let coords: Vec<i32> = line.split(',').map(|s| s.parse().unwrap()).collect();
        (coords[0], coords[1], coords[2])
    });
    let num_nodes = nodes.clone().count();
    let mut node_combinations = nodes
        .tuple_combinations()
        .map(|((x1, y1, z1), (x2, y2, z2))| {
            let x_diff = x1 as f64 - x2 as f64;
            let y_diff = y1 as f64 - y2 as f64;
            let z_diff = z1 as f64 - z2 as f64;

            let dist = (x_diff.powi(2) + y_diff.powi(2) + z_diff.powi(2)).sqrt();
            ((x1, y1, z1), (x2, y2, z2), dist)
        })
        .collect::<Vec<((i32, i32, i32), (i32, i32, i32), f64)>>();
    node_combinations.sort_by(|a, b| a.2.total_cmp(&b.2));
    let mut networks = vec![];
    for (c1, c2, _) in node_combinations.iter() {
        let (to_merge, mut others): (Vec<HashSet<_>>, Vec<HashSet<_>>) = networks
            .into_iter()
            .partition(|set: &HashSet<_>| set.contains(&c1) || set.contains(&c2));

        let mut new_group = HashSet::new();
        new_group.insert(c1);
        new_group.insert(c2);

        for set in to_merge {
            new_group.extend(set);
        }

        if new_group.len() == num_nodes {
            return c1.0 as usize * c2.0 as usize;
        }
        others.push(new_group);
        networks = others;
    }
    0
}
