
const EXAMPLE_ANSWER: usize = 3121910778619;

aoc::solution!(EXAMPLE_ANSWER, solve);

fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|line| line
            .chars()
            .map(|c| c as u8 - '0' as u8)
            .collect())
        .map(|bank: Vec<u8>| {
            (0..12)
                .fold((0, vec![]), |(mut last_position, mut max_values), _| {
                    let slice = &bank[last_position..=bank.len() -  (12 - max_values.len())];
                    let max = *slice.iter().max().unwrap();
                    last_position = slice.iter().position(|&x| x == max).unwrap()+last_position+1;
                    max_values.push(max);
                    (last_position, max_values)
                })
                .1
                .into_iter()
                .enumerate()
                .map(|(i, x)| x as usize * 10_usize.pow(12 - i as u32 - 1))
                .sum::<usize>()
        })
        .map(|x| x as usize)
        .sum()
}
