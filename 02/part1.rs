use rayon::prelude::*;

const EXAMPLE_ANSWER: usize = 1227775554;

aoc::solution!(EXAMPLE_ANSWER, solve);

fn solve(input: &str) -> usize {
    input
        .split(',')
        .map(|range| {
            let mut parts = range.split('-');
            let start = parts.next().unwrap().trim().parse::<usize>().unwrap();
            let end = parts.next().unwrap().trim().parse::<usize>().unwrap();
            (start, end)
        })
        .map(|(start, end)| count_valid_in_range(start, end))
        .sum()
}

fn count_valid_in_range(start: usize, end: usize) -> usize {
    let mut total_sum = 0;
    // Use fact that  N = x * (10^k + 1)  where x is in [10^(k-1), 10^k - 1]

    for exp in 1..=9 {
        let half_start_base = 10usize.pow(exp - 1);
        let half_end_base = 10usize.pow(exp) - 1;
        let multiplier = 10usize.pow(exp) + 1;

        let min_x_needed = (start + multiplier - 1) / multiplier;
        let max_x_needed = end / multiplier;

        // Intersect the mathematical requirements with the digit-length constraints
        let actual_min_x = min_x_needed.max(half_start_base);
        let actual_max_x = max_x_needed.min(half_end_base);

        if actual_min_x <= actual_max_x {
            let count = actual_max_x - actual_min_x + 1;
            
            let sum_x = count as u128 * (actual_min_x + actual_max_x) as u128 / 2;
            total_sum += (sum_x * multiplier as u128) as usize;
        }
    }

    total_sum
}