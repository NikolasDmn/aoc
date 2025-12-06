const EXAMPLE_ANSWER: usize = 4277556;

aoc::solution!(EXAMPLE_ANSWER, solve);


fn solve(input: &str) -> usize {
    // 1. Parse into a grid once. No redundant trims.
    let grid: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    let width = grid[0].len();
    let height = grid.len();

    (0..width)
        .map(|x| {
            let op = match grid[height - 1][x] {
                "+" => |a, b| a + b,
                "*" => |a, b| a * b,
                x => panic!("Unknown operator '{}'. Math has left the building.", x),
            };

            (0..height - 1)
                .map(|y| grid[y][x].parse::<usize>().unwrap())
                .reduce(op)
                .unwrap()
        })
        .sum()
}
