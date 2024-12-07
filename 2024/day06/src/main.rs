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

    let mut checked_spaces = HashSet::new();
    let mut new_obstacle_candidates = HashSet::new();

    for (i, j, dir) in WalkIterator::new(grid, start, Direction::Up) {
        let coords_in_front = dir.offset_coords(i, j);
        let (i2, j2) = coords_in_front;

        match grid.get(i2).map(|row| row.get(j2)).flatten() {
            None | Some(&b'#') | Some(&b'^') => continue,
            _ => {}
        }

        if checked_spaces.contains(&coords_in_front) {
            continue;
        }
        checked_spaces.insert(coords_in_front);

        if WalkIterator::new(grid, (i, j), dir)
            .with_added_obstacle(coords_in_front)
            .is_inifinite_loop()
        {
            new_obstacle_candidates.insert(coords_in_front);
        }
    }

    new_obstacle_candidates.len()
}

fn find_start(grid: &[&[u8]]) -> Option<(usize, usize)> {
    grid.iter().enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().map(move |(j, byte)| (i, j, byte))
        })
        .find(|(_, _, &byte)| byte == b'^')
        .map(|(i, j, _)| (i, j))
}
