use std::io;
use std::error::Error;
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin().lock();
    let input = parse_input(stdin)?;

    println!("Part 1: {}", solve_part_1(&input));
    println!("Part 2: {}", solve_part_2(&input));
    Ok(())
}

fn solve_part_1(input: &[Vec<u32>]) -> usize {
    fn dfs(map: &[Vec<u32>], pos: (usize, usize), expected_num: u32, out: &mut HashSet<(usize,usize)>) {
        let (i, j) = pos;
        let Some(current) = map.get(i).and_then(|row| row.get(j)) else {
            return;
        };

        if *current != expected_num {
            return;
        }
        if *current >= 9 {
            out.insert(pos);
            return;
        }

        let i_above = i.overflowing_sub(1).0;
        let i_below = i + 1;
        let j_left = j.overflowing_sub(1).0;
        let j_right = j + 1;

        dfs(map, (i_above, j), current+1, out);
        dfs(map, (i_below, j), current+1, out);
        dfs(map, (i, j_left), current+1, out);
        dfs(map, (i, j_right), current+1, out);
    }

    let mut total_score = 0;
    for (i, row) in input.iter().enumerate() {
        for j in 0..row.len() {
            let mut reachable_9s = HashSet::new();
            dfs(input, (i, j), 0, &mut reachable_9s);
            total_score += reachable_9s.len();
        }
    }
    total_score
}

fn solve_part_2(input: &[Vec<u32>]) -> usize {
    fn dfs(map: &[Vec<u32>], pos: (usize, usize), expected_num: u32) -> usize {
        let (i, j) = pos;
        let Some(current) = map.get(i).and_then(|row| row.get(j)) else {
            return 0;
        };

        if *current != expected_num {
            return 0;
        }
        if *current >= 9 {
            return 1;
        }

        let i_above = i.overflowing_sub(1).0;
        let i_below = i + 1;
        let j_left = j.overflowing_sub(1).0;
        let j_right = j + 1;

        dfs(map, (i_above, j), current+1 )
            + dfs(map, (i_below, j), current+1)
            + dfs(map, (i, j_left), current+1)
            + dfs(map, (i, j_right), current+1)
    }

    let mut total_rating = 0;
    for (i, row) in input.iter().enumerate() {
        for j in 0..row.len() {
            total_rating += dfs(input, (i, j), 0);
        }
    }
    total_rating
}

fn parse_input(input: impl io::BufRead) -> Result<Vec<Vec<u32>>, Box<dyn Error>> {
    let mut grid = vec![];
    for line in input.lines() {
        let line = line?;
        let line = line.trim_end();
        let row = line.chars()
            .map(|c| c.to_digit(10))
            .collect::<Option<_>>()
            .ok_or_else(|| "Non-digit character")?;
        grid.push(row);
    }
    Ok(grid)
}
