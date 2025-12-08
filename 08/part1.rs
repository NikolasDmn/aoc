use aoc;
use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap};

const EXPECTED: usize = 40;

aoc::solution!(EXPECTED, solve);

use std::cmp::Ordering;
#[derive(PartialEq, Debug)]
struct Edge {
    dist_sq: i64,
    u: usize,
    v: usize,
}

impl Eq for Edge {}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.dist_sq.partial_cmp(&other.dist_sq)
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist_sq
            .partial_cmp(&other.dist_sq)
            .unwrap_or(Ordering::Equal)
    }
}

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
        }
    }
    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            i
        } else {
            let root = self.find(self.parent[i]);
            self.parent[i] = root;
            root
        }
    }
    fn union(&mut self, i: usize, j: usize) {
        let root_i = self.find(i);
        let root_j = self.find(j);
        if root_i != root_j {
            self.parent[root_i] = root_j;
        }
    }
}

pub fn solve(input: &str) -> usize {
    let points: Vec<(i64, i64, i64)> = input
        .lines()
        .map(|line| {
            let mut nums = line.split(',').map(|s| s.trim().parse::<i64>().unwrap());
            (
                nums.next().unwrap(),
                nums.next().unwrap(),
                nums.next().unwrap(),
            )
        })
        .collect();

    let n = points.len();

    let total_edges = n * (n - 1) / 2;
    let limit = if total_edges < 1000 { 10 } else { 1000 };

    // limit + 1 to never realocate when pushing
    let mut heap: BinaryHeap<Edge> = BinaryHeap::with_capacity(limit + 1);

    for i in 0..n {
        for j in (i + 1)..n {
            let (x1, y1, z1) = points[i];
            let (x2, y2, z2) = points[j];

            let dx = (x1 - x2) as i64;
            let dy = (y1 - y2) as i64;
            let dz = (z1 - z2) as i64;
            let dist = dx * dx + dy * dy + dz * dz;

            // worse than the worst edge in our heap, skip it immediately.
            if heap.len() == limit {
                if let Some(worst_best) = heap.peek() {
                    if dist >= worst_best.dist_sq {
                        continue;
                    }
                }
            }

            heap.push(Edge {
                dist_sq: dist,
                u: i,
                v: j,
            });

            if heap.len() > limit {
                heap.pop();
            }
        }
    }

    let mut dsu = UnionFind::new(n);
    for edge in heap {
        dsu.union(edge.u, edge.v);
    }

    let mut group_counts = HashMap::new();
    for i in 0..n {
        *group_counts.entry(dsu.find(i)).or_insert(0) += 1;
    }

    group_counts
        .values()
        .sorted_unstable_by(|a, b| b.cmp(a))
        .take(3)
        .product()
}
