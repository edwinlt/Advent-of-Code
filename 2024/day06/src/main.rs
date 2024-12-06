use std::io;
use std::error::Error;
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin().lock();
    let input: String = io::read_to_string(stdin)?;
    let input: Vec<_> = input.lines().map(str::as_bytes).collect();

    println!("Part 1: {}", solve_part_1(&input));
    //println!("Part 2: {}", solve_part_2(&input));
    Ok(())
}

fn solve_part_1(grid: &[&[u8]]) -> usize {
    let Some((mut i, mut j)) = find_starting_coords(grid) else {
        return 0;
    };

    let m = grid.len();
    let n = grid[0].len();

    let mut dir = 0;
    let mut visited = HashSet::new();
    loop {
        visited.insert((i, j));
        let (i2, j2) = match dir {
            0 => if i != 0  {(i-1, j)} else {break;},
            1 => if j+1 < n {(i, j+1)} else {break;},
            2 => if i+1 < m {(i+1, j)} else {break;},
            3 => if j != 0  {(i, j-1)} else {break;},
            _ => unreachable!()
        };

        if grid[i2][j2] == b'#' {
            dir = (dir + 1) % 4;
        } else {
            (i, j) = (i2, j2)
        }
    }

    visited.len()
}

fn find_starting_coords(grid: &[&[u8]]) -> Option<(usize, usize)> {
    grid.iter().enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate()
                .map(move |(j, byte)| (i, j, byte))
        })
        .find(|(_, _, &byte)| byte == b'^')
        .map(|(i, j, _)| (i, j))
}
