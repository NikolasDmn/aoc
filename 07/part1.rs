use core::panic;

use aoc;
use itertools::Itertools;

const EXPECTED: usize = 21;

aoc::solution!(EXPECTED, solve);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Start,
    Splitter,
    Beam,
}

fn solve(input: &str) -> usize {
    let mut mushed_input: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| line.chars())
        .map(| chars| {
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

    let mut total = 0;
    let beam_start = mushed_input[0]
        .iter()
        .position(|tile| tile == &Tile::Start)
        .unwrap();

    mushed_input[0][beam_start] = Tile::Beam;
    for row in 1..mushed_input.len() {
        for col in 0..mushed_input[row].len() {
            match mushed_input[row][col] {
                Tile::Empty => {
                    if mushed_input[row - 1][col] == Tile::Beam {
                        mushed_input[row][col] = Tile::Beam;
                    }
                }
                Tile::Splitter => {
                    if mushed_input[row - 1][col] == Tile::Beam {
                        if col > 1 {
                            mushed_input[row][col - 1] = Tile::Beam;
                        }
                        if col < mushed_input[row].len() - 1 {
                            mushed_input[row][col + 1] = Tile::Beam;
                        }
                        total += 1;
                    }
                }
                _ => {}
               
            }
        }
    }
    total
}
