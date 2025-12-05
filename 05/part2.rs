use itertools::Itertools;
use std::fs::File;
use std::time::Instant;

const EXAMPLE_ANSWER: usize = 14;

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
    let sanitized_in = input.replace("\r\n", "\n");
    let ranges: Vec<(usize, usize)> = sanitized_in
        .split("\n\n")
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.split("-")
                .map(|n| n.trim().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .map(|r| (r[0], r[1]))
        .sorted_by_key(|k| k.0)
        .collect();

    let mut merged_ranges: Vec<(usize, usize)> = vec![];
    for (start, end) in ranges {
        if merged_ranges.is_empty() {
            merged_ranges.push((start, end));
            continue;
        }
        let last_idx = merged_ranges.len() - 1;
        let last = &mut merged_ranges[last_idx];
        if start <= last.1 {
            last.1 = last.1.max(end);
        } else {
            merged_ranges.push((start, end));
        }
    }

    merged_ranges
        .into_iter()
        .map(|(start, end)| end - start + 1)
        .sum()
}
