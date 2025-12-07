use std::collections::HashMap;
use std::collections::HashSet;

use aoc;
use itertools::Itertools;
use std::cell::RefCell;
use std::rc::Rc;

const EXPECTED: usize = 40;

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

fn calculate_timelines(node_idx: usize, arena: &mut [Node]) -> usize {
    if let Some(weight) = arena[node_idx].memoized_timelines {
        return weight;
    }
    let children_indices = arena[node_idx].children.clone();
    if children_indices.is_empty() {
        arena[node_idx].memoized_timelines = Some(2);
        return 2;
    }
    let mut total: usize = 0;
    for child_idx in children_indices.iter() {
        total += calculate_timelines(*child_idx, arena);
    }
    // If odd number of children, add one more timeline for the unpaired beam
    if children_indices.len() % 2 == 1 {
        total += 1;
    }
    arena[node_idx].memoized_timelines = Some(total);
    total
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

    let start_idx = 0;
    let rows = in_table.len();
    let cols = in_table[0].len();

    let mut grid_lookup: Vec<Option<usize>> = vec![None; rows * cols];
    let mut arena: Vec<Node> = Vec::with_capacity(in_table.len() * in_table[0].len());

    let start_node = Node {
        children: vec![],
        memoized_timelines: None,
    };

    let get_flat_idx = |r: usize, c: usize| r * cols + c;
    grid_lookup[get_flat_idx(2, beam_start_col)] = Some(0); // Point to start node (idx 0)
    arena.push(start_node);

    let mut parent_indices: Vec<usize> = Vec::with_capacity(4);

    for row in 1..in_table.len() {
        for col in 0..in_table[row].len() {
            if in_table[row][col] != Tile::Splitter {
                continue;
            }

                parent_indices.clear();
            for r in (0..row).rev() {
                if in_table[r][col] != Tile::Empty {
                    break;
                }
                // Check Left
                if let Some(idx) = grid_lookup.get(get_flat_idx(r, col - 1)).and_then(|&x| x) {
                    parent_indices.push(idx);
                }
                // Check Right
                if let Some(idx) = grid_lookup.get(get_flat_idx(r, col + 1)).and_then(|&x| x) {
                    parent_indices.push(idx);
                }
            }
            if parent_indices.is_empty() {
                continue;
            }
            // Only create a node if it's connected to something
            let new_node_idx = arena.len();
            for &p_idx in &parent_indices {
                arena[p_idx].children.push(new_node_idx);
            }
            let current_node = Node {
                children: vec![],
                memoized_timelines: None,
            };

            arena.push(current_node);
            grid_lookup[get_flat_idx(row, col)] = Some(new_node_idx);
        }
    }
    calculate_timelines(start_idx, &mut arena)
}
