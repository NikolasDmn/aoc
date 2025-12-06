use itertools::Itertools;

const EXAMPLE_ANSWER: usize = 3263827;

aoc::solution!(EXAMPLE_ANSWER, solve);


fn solve(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    if grid.is_empty() { return 0; }

    let width = grid[0].len();
    let height = grid.len();

    (0..width)
        .map(|x| {
            (0..height)
                .filter_map(|y| grid.get(y).and_then(|row| row.get(x)))
                .collect::<String>()
        })
        .chunk_by(|col| col.trim().is_empty()) 
        .into_iter()
        .filter_map(|(is_separator, group)| {
            if is_separator { None } else { Some(group) }
        })
        .map(|block_iter| {
            let cols: Vec<String> = block_iter.collect();
            let op_char = cols[0]
                .chars()
                .find(|c| matches!(*c, '+' | '*'))
                .expect("Block missing an operator. Chaos ensues.");
            let op = match op_char {
                '+' => |a, b| a + b,
                '*' => |a, b| a * b,
                _ => unreachable!(), 
            };

            cols.iter()
                .map(|col| {
                    col.trim_matches(|c: char| !c.is_numeric())
                       .parse::<usize>()
                       .expect("Parsing failed. Is that a letter? Disgusting.")
                })
                .reduce(op)
                .expect("Empty block encountered.")
        })
        .sum()
}
