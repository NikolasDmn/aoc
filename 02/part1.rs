use rayon::prelude::*;
use std::time::Instant;

const EXAMPLE_ANSWER: usize = 1227775554;

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
    input
        .split(",")
        .map(|range| range.split("-").collect::<Vec<&str>>())
        .map(|range_vec| {
            (
                range_vec[0].trim().parse::<usize>().unwrap(),
                range_vec[1].trim().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<(usize, usize)>>()
        .into_par_iter()
        .flat_map(|(start, end)| {
            (start..=end).into_par_iter().filter(|num| {
                let k = num.checked_ilog10().unwrap_or(0) + 1;
                if k % 2 == 1 {
                    return false;
                }
                let mid_point = k / 2;
                let first_part = num / 10usize.pow(mid_point);
                let second_part = num % 10usize.pow(mid_point);
                first_part == second_part
            })
        })
        .sum()
}
