use aoc;
use itertools::Itertools;
use std::collections::HashMap;

const EXPECTED: usize = 21;

aoc::solution!(EXPECTED, solve);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Start,
    Splitter,
    Beam,
}

#[derive(Debug)]
struct Node {
    children: Vec<usize>,
    memoized_timelines: Option<usize>,
}

fn solve(input: &str) -> usize {
     let in_table: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| line.chars())
        .map(|chars| {
            chars
                .map(|c| match c {
                    '.' => Tile::Empty,
                    'S' => Tile::Start,
                    '^' => Tile::Splitter,
                    '|' => Tile::Beam,
                    _ => panic!("Unknown character: {}", c),
                })
                .collect()
        })
        .collect();

    let beam_start_col = in_table[0]
        .iter()
        .position(|tile| tile == &Tile::Start)
        .unwrap();

    let mut arena: Vec<Node> = Vec::with_capacity(in_table.len() * in_table[0].len());
    let mut node_lookup: HashMap<(usize, usize), usize> = HashMap::new();

    let start_node = Node {
        children: vec![],
        memoized_timelines: None,
    };
    arena.push(start_node);
    let start_idx = 0; 
    node_lookup.insert((2, beam_start_col), start_idx);

    for row in 1..in_table.len() {
        for col in 0..in_table[row].len() {
            if in_table[row][col] != Tile::Splitter {
                continue;
            }
            let mut parent_indices = Vec::new();
            for r in (0..row).rev() {
                if in_table[r][col] != Tile::Empty {
                    break;
                }
                // Check Left
                if let Some(&idx) = node_lookup.get(&(r, col - 1)) {
                    parent_indices.push(idx);
                }
                // Check Right
                if let Some(&idx) = node_lookup.get(&(r, col + 1)) {
                    parent_indices.push(idx);
                }
            }
            // Only create a node if it's connected to something
            if !parent_indices.is_empty() {
                let new_node_idx = arena.len();
                for &p_idx in &parent_indices {
                    arena[p_idx].children.push(new_node_idx);
                }
                let current_node = Node {
                    children: vec![],
                    memoized_timelines: None,
                };

                arena.push(current_node);
                node_lookup.insert((row, col), new_node_idx);
            }
        }
    }
    arena.len()
}
