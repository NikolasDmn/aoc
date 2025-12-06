use rayon::prelude::*;

const EXAMPLE_ANSWER: usize = 1227775554;

aoc::solution!(EXAMPLE_ANSWER, solve);

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
