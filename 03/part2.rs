use std::fs::File;
use std::time::Instant;

const EXAMPLE_ANSWER: usize = 3121910778619;

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
        .lines()
        .map(|line| line
            .chars()
            .map(|c| c as u8 - '0' as u8)
            .collect())
        .map(|bank: Vec<u8>| {
            (0..12)
                .fold((0, vec![]), |(mut last_position, mut max_values), _| {
                    let slice = &bank[last_position..=bank.len() -  (12 - max_values.len())];
                    let max = *slice.iter().max().unwrap();
                    last_position = slice.iter().position(|&x| x == max).unwrap()+last_position+1;
                    max_values.push(max);
                    (last_position, max_values)
                })
                .1
                .into_iter()
                .enumerate()
                .map(|(i, x)| x as usize * 10_usize.pow(12 - i as u32 - 1))
                .sum::<usize>()
        })
        .map(|x| x as usize)
        .sum()
}
