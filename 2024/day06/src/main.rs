use std::io;
use std::error::Error;
use std::collections::HashSet;

mod walk;
use walk::*;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin().lock();
    let input: String = io::read_to_string(stdin)?;
    let input: Vec<_> = input.lines().map(str::as_bytes).collect();

    println!("Part 1: {}", solve_part_1(&input));
    println!("Part 2: {}", solve_part_2(&input));
    Ok(())
}


fn solve_part_1(grid: &[&[u8]]) -> usize {
    let Some(start) = find_start(grid) else {
        return 0;
    };

    WalkIterator::new(grid, start, Direction::Up)
        .map(|(i, j, _)| (i, j))
        .collect::<HashSet<_>>()
        .len()
}

fn solve_part_2(grid: &[&[u8]]) -> usize {
    let Some(start) = find_start(grid) else {
        return 0;
    };

    let mut answer = 0;

    let mut checked_spaces = HashSet::new();
    checked_spaces.insert(start);
    
    let mut prev = start;
    for (i, j, dir) in WalkIterator::new(grid, start, Direction::Up).skip(1) {
        if checked_spaces.insert((i, j)) {
            let branched_path = WalkIterator::new(grid, prev, dir)
                .with_added_obstacle((i, j));
            if branched_path.is_inifinite_loop() {
                answer += 1;
            }
        }

        prev = (i, j);
    }

    answer
}

fn find_start(grid: &[&[u8]]) -> Option<(usize, usize)> {
    grid.iter().enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().map(move |(j, byte)| (i, j, byte))
        })
        .find(|(_, _, &byte)| byte == b'^')
        .map(|(i, j, _)| (i, j))
}
