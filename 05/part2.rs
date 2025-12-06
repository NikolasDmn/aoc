use itertools::Itertools;
const EXAMPLE_ANSWER: usize = 14;

aoc::solution!(EXAMPLE_ANSWER, solve);

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
