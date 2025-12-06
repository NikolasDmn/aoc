
const EXAMPLE_ANSWER: usize = 6;

aoc::solution!(EXAMPLE_ANSWER, solve);

fn solve(input: &str) -> usize {
    input
        .lines()
        .fold((50isize, 0usize), |(pos, total_clicks), line| {
            let is_left = (line.starts_with('L')) as isize;
            let sign = 1 - (is_left * 2);
            let length: isize = line[1..].parse().unwrap();
            let next_pos = pos + (sign * length);
            let boundary_crossings =
                ((pos - is_left).div_euclid(100) - (next_pos - is_left).div_euclid(100)).abs();

            (next_pos, total_clicks + boundary_crossings as usize)
        })
        .1
}
