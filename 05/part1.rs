const EXAMPLE_ANSWER: usize = 3;

aoc::solution!(EXAMPLE_ANSWER, solve);

fn solve(input: &str) -> usize {
    let sanitized_in = input.replace("\r\n", "\n");
    let mut split_inp = sanitized_in.split("\n\n");
    let ranges: Vec<(usize, usize)> = split_inp
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.split("-")
                .map(|n| n.trim().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .map(|r| (r[0], r[1]))
        .collect();
    split_inp
        .next()
        .unwrap()
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .filter(|product| {
            ranges
                .iter()
                .any(|&(start, end)| *product >= start && *product <= end)
        })
        .count()
}
