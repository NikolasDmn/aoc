use rayon::prelude::*;

const EXAMPLE_ANSWER: usize = 4174379265;

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
        .map(|(start, end)| {
            (start..=end).into_par_iter().filter(|num| {
                let k = num.checked_ilog10().unwrap_or(0) + 1;
                (1..=(k / 2))
                    .filter(|&segment_size| k % segment_size == 0) // Check only valid divisors
                    .any(|segment_size| {
                        let divisor = 10usize.pow(segment_size);
                        let target_chunk = num % divisor;
                        (1..(k / segment_size)).all(|i| {
                            let shift = i * segment_size;
                            (num / 10usize.pow(shift)) % divisor == target_chunk
                        })
                    })
            })
        })
        .flatten()
        .sum()
}
