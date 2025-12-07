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

type NodeRef = Rc<RefCell<Node>>;
#[derive(Debug)]
struct Node {
    parents: Vec<NodeRef>,
    children: Vec<NodeRef>,
    memoized_timelines: Option<usize>,
}
fn calculate_timelines(node_ref: &NodeRef) -> usize {
    let mut node = node_ref.borrow_mut();
    if let Some(weight) = node.memoized_timelines {
        return weight;
    }
    if node.children.is_empty() {
        node.memoized_timelines = Some(2);
        return 2;
    }
    let mut total: usize = node
        .children
        .iter()
        .map(|child| calculate_timelines(child))
        .sum();
    if node.children.len() % 2 ==  1 {
        total += 1; // Account for the split
    }
    node.memoized_timelines = Some(total);
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

    let beam_start = in_table[0]
        .iter()
        .position(|tile| tile == &Tile::Start)
        .unwrap();

    let mut nodes: HashMap<(usize, usize), NodeRef> = HashMap::new();
    let start_node = Rc::new(RefCell::new(Node {
        parents: vec![],
        children: vec![],
        memoized_timelines: None,
    }));
    nodes.insert((2, beam_start), Rc::clone(&start_node));
    for row in 1..in_table.len() {
        for col in 0..in_table[row].len() {
            if in_table[row][col] != Tile::Splitter {
                continue;
            }
            let current_node = Rc::new(RefCell::new(Node {
                parents: vec![],
                children: vec![],
                memoized_timelines: None,
            }));

            for r in (0..row).rev() {
                if in_table[r][col] != Tile::Empty {
                    break;
                }

                if let Some(node) = nodes.get(&(r, col - 1)) {
                    current_node.borrow_mut().parents.push(Rc::clone(node));
                    node.borrow_mut().children.push(Rc::clone(&current_node));
                }
                if let Some(node) = nodes.get(&(r, col + 1)) {
                    current_node.borrow_mut().parents.push(Rc::clone(node));
                    node.borrow_mut().children.push(Rc::clone(&current_node));
                }
            }
            if !current_node.borrow().parents.is_empty() {
                nodes.insert((row, col), current_node);
            }
        }
    }

    let total_timelines = calculate_timelines(nodes.get(&(2, beam_start)).unwrap());
    total_timelines
}
