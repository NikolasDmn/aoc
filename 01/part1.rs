const EXAMPLE_ANSWER: usize = 3;

aoc::solution!(EXAMPLE_ANSWER, solve);

fn solve(input: &str) -> usize {
    let (_, num_of_zeros) = input
        .lines()
        .fold((50, 0), |(location, num_of_zeros), line| {
            let sign = if line.starts_with('R') { 1 } else { -1 };
            let length: isize = line[1..].parse().unwrap();
            let end_location = (location + (sign * length)).rem_euclid(100);
            (
                end_location,
                num_of_zeros + if end_location == 0 { 1 } else { 0 },
            )
        });
    num_of_zeros
}
