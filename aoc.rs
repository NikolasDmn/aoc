use std::fmt::Display;
use std::time::Instant;
use std::hint::black_box;
use memory_stats::memory_stats;

// Colors
pub const GREEN: &str = "\x1b[32m";
pub const RED: &str = "\x1b[31m";
pub const BLUE: &str = "\x1b[34m";
pub const BOLD: &str = "\x1b[1m";
pub const ITALIC: &str = "\x1b[3m";
pub const RESET: &str = "\x1b[0m";

#[macro_export]
macro_rules! solution {
    ($expected:expr, $solve:ident) => {
        fn main() {
            let sample = include_str!("sample.txt");
            let input = include_str!("input.txt");
            $crate::run($solve, $expected, sample, input);
        }
    };
}

pub fn run<T, F>(solve_func: F, expected: T, sample: &str, input: &str)
where
    T: Display + Eq + std::fmt::Debug + Copy,
    F: Fn(&str) -> T,
{
    println!("{}🎄 ADVENT OF CODE 🎄{}", BOLD, RESET);
    println!("{}----------------------------------{}", BLUE, RESET);

    // 1. Sample Run
    let start_sample = Instant::now();
    let sample_ans = solve_func(sample);
    let dur_sample = start_sample.elapsed();

    if sample_ans != expected {
        println!("{}❌ SAMPLE FAILED.{}", RED, RESET);
        println!("   Expected: {}{:?}{}", GREEN, expected, RESET);
        println!("   Got:      {}{:?}{}", RED, sample_ans, RESET);
        println!("{}----------------------------------{}", BLUE, RESET);
        return;
    }

    println!("{}✅ Sample Passed!{}", GREEN, RESET);
    println!("   Time: {:.2?}", dur_sample);
    println!("{}----------------------------------{}", BLUE, RESET);

    // 2. Real Input Run
    println!("{}🚀 Running Real Input...{}", BOLD, RESET);
    
    // Snapshot memory before (baseline)
    let mem_before = memory_stats().map(|s| s.physical_mem).unwrap_or(0);
    
    let start_input = Instant::now();
    let ans = black_box(solve_func(input));
    let dur_input = start_input.elapsed();

    // Snapshot memory after (peak-ish)
    let mem_after = memory_stats().map(|s| s.physical_mem).unwrap_or(0);
    let mem_used = mem_after.saturating_sub(mem_before);

    println!("{}----------------------------------{}", BLUE, RESET);
    println!("{}🎉 FINAL ANSWER: {}{}", GREEN, BOLD, RESET);
    println!("   {} > {} < {}", BOLD, ans, RESET);
    println!("{}----------------------------------{}", BLUE, RESET);
    println!("   Time: {}{:.2?}{}", ITALIC, dur_input, RESET);
    println!("   Mem:  {}{}{}", ITALIC, format_mem(mem_used), RESET);
}

// Helper to make numbers look human
fn format_mem(bytes: usize) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.2} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}