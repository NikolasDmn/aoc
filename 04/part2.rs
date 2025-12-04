use std::fs::File;
use std::time::Instant;

const EXAMPLE_ANSWER: usize = 43;

const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const BLUE: &str = "\x1b[34m";
const BOLD: &str = "\x1b[1m";
const ITALIC: &str = "\x1b[3m";
const RESET: &str = "\x1b[0m";

fn main() {
    let sample = include_str!("sample.txt");
    let input = include_str!("input.txt");
    println!("{}🎄 ADVENT OF CODE 🎄{}", BOLD, RESET);
    println!("{}----------------------------------{}", BLUE, RESET);

    let start_sample = Instant::now();
    let sample_answer = solve(sample);
    let duration_sample = start_sample.elapsed();
    if sample_answer != EXAMPLE_ANSWER {
        println!("{}❌ SAMPLE FAILED.{}", RED, RESET);
        println!("   Expected: {}{}{}", GREEN, EXAMPLE_ANSWER, RESET);
        println!("   Got:      {}{}{}", RED, sample_answer, RESET);
        println!("{}----------------------------------{}", BLUE, RESET);
        return;
    }

    println!("{}✅ Sample Passed!{}", GREEN, RESET);
    println!("   Time: {:.2?}", duration_sample);
    println!("{}----------------------------------{}", BLUE, RESET);
    println!("{}🚀 Running Real Input...{}", BOLD, RESET);
    let start_input = Instant::now();
    let answer = solve(input);
    let duration_input = start_input.elapsed();
    println!("{}----------------------------------{}", BLUE, RESET);
    println!("{}🎉 FINAL ANSWER: {}{}", GREEN, BOLD, RESET);
    println!("   {} > {} < {}", BOLD, answer, RESET);
    println!("{}----------------------------------{}", BLUE, RESET);
    println!("   Time: {}{:.2?}{}", ITALIC, duration_input, RESET);
}

fn solve(input: &str) -> usize {
    let mut grid: Vec<Vec<bool>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '@' => true,
                    '.' => false,
                    _ => panic!("Unexpected character in input"),
                })
                .collect()
        })
        .collect();
    let mut reachable_papers = 0;
    let mut last_reachable = 0;
    let rows = grid.len();
    let cols = grid[0].len();

    let offsets: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    loop {
        for r in 0..rows {
            for c in 0..cols {
                if !grid[r][c] {
                    continue;
                }

                let mut neighbors = 0;

                for (dr, dc) in offsets {
                    // These will wrap around on underflow, which is fine because we check bounds after
                    let nr = r.wrapping_add(dr as usize);
                    let nc = c.wrapping_add(dc as usize);

                    if nr < rows && nc < cols && grid[nr][nc] {
                        neighbors += 1;
                    }
                }

                if neighbors < 4 {
                    grid[r][c] = false;
                    reachable_papers += 1;
                }
            }
        }
        if reachable_papers == last_reachable {
            break;
        }
        last_reachable = reachable_papers;
    }
    reachable_papers
}
