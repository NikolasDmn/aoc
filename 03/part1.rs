const EXAMPLE_ANSWER: usize = 357;

aoc::solution!(EXAMPLE_ANSWER, solve);

fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.chars().map(|c| c as u8- '0' as u8).collect())
        .map(|bank: Vec<u8>| {
            let max = *bank[..bank.len()-1].iter().max().unwrap();
            let max_index = bank.iter().position(|&x| x == max).unwrap();
            let max_two = bank[max_index+1..].iter().max().unwrap();
            max*10 + max_two

        }).map(|x| x as usize)
        .sum()
}
