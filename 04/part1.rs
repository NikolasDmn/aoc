const EXAMPLE_ANSWER: usize = 13;

aoc::solution!(EXAMPLE_ANSWER, solve);

fn solve(input: &str) -> usize {
    let grid: Vec<Vec<bool>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '@' => true,
                    '.' => false,
                    _ => panic!("Unexpected character in input"),
                })
                .collect()
        })
        .collect();
    let mut reachable_papers = 0;
    let rows = grid.len();
    let cols = grid[0].len();

    let offsets: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for r in 0..rows {
        for c in 0..cols {
            if !grid[r][c] {
                continue;
            }

            let mut neighbors = 0;

            for (dr, dc) in offsets {
                // These will wrap around on underflow, which is fine because we check bounds after
                let nr = r.wrapping_add(dr as usize);
                let nc = c.wrapping_add(dc as usize);

                if nr < rows && nc < cols && grid[nr][nc] {
                    neighbors += 1;
                }
            }

            if neighbors < 4 {
                reachable_papers += 1;
            }
        }
    }
    reachable_papers
}
