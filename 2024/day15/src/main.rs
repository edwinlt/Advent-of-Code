use std::io;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin().lock();
    let input = parse_input(stdin)?;

    println!("Part 1: {}", solve_part_1(&input));
    println!("Part 2: {}", solve_part_2(&input));
    Ok(())
}

#[derive(Debug)]
struct PuzzleInput {
    grid: Vec<Vec<u8>>,
    robot: (i32, i32),
    moves: Vec<(i32, i32)>,
}

fn solve_part_1(input: &PuzzleInput) -> usize {
    let mut grid = input.grid.clone();
    let mut robot = input.robot;

    for (d0, d1) in input.moves.iter() {
        let p1 = (robot.0 + d0, robot.1 + d1);
        let mut p2 = p1;

        while grid[p2.0 as usize][p2.1 as usize] == b'O' {
            p2.0 += d0;
            p2.1 += d1;
        }

        if grid[p2.0 as usize][p2.1 as usize] != b'#' {
            grid[p2.0 as usize][p2.1 as usize] = b'O';
            grid[p1.0 as usize][p1.1 as usize] = b'@';
            grid[robot.0 as usize][robot.1 as usize] = b'.';
            robot = p1;
        }
    }
    
    gps_sum(&grid, b'O')
}

fn solve_part_2(input: &PuzzleInput) -> usize {
    let mut grid = widen_grid(&input.grid);
    let mut robot = input.robot;
    robot.1 *= 2;

    for (d0, d1) in input.moves.iter() {
        let destination = (robot.0 + d0, robot.1 + d1);

        match (d0, d1) {
            (&d0, 0) => { // Vertical move
                let mut next = grid.clone();
                if push_vertically(&mut next, destination, d0) {
                    next[destination.0 as usize][destination.1 as usize] = b'@';
                    next[robot.0 as usize][robot.1 as usize] = b'.';
                    robot = destination;
                    grid = next;
                }
            },
            (0, &d1) => { // Horizontal move
                let row = &mut grid[robot.0 as usize];
                let mut j = destination.1;
                while b"[]".contains(&row[j as usize]) {
                    j += d1;
                }
                if row[j as usize] == b'.' {
                    while j != destination.1 {
                        row[j as usize] = row[(j-d1) as usize];
                        j -= d1;
                    }
                    row[destination.1 as usize] = b'@';
                    row[robot.1 as usize] = b'.';
                    robot = destination;
                }
            },
            _ => unreachable!()
        }
    }

    gps_sum(&grid, b'[')
}

fn gps_sum(grid: &[Vec<u8>], val: u8) -> usize {
    grid.iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, tile)| (i, j, tile)))
        .filter(|(_, _, tile)| **tile == val)
        .map(|(i, j, _)| i*100 + j)
        .sum()
}

fn push_vertically(grid: &mut [Vec<u8>], pos: (i32, i32), d: i32) -> bool {
    assert!(d == 1 || d == -1);
    let (i, j) = match grid[pos.0 as usize][pos.1 as usize] {
        b'[' => pos,
        b']' => (pos.0, pos.1 - 1),
        b'.' => return true,
        b'#' => return false,
        _ => unreachable!()
    };

    let front_left = (i+d, j);
    let front_right = (i+d, j+1);
    if push_vertically(grid, front_left, d) && push_vertically(grid, front_right, d) {
        grid[i as usize][j as usize] = b'.';
        grid[i as usize][(j+1) as usize] = b'.';
        grid[front_left.0 as usize][front_left.1 as usize] = b'[';
        grid[front_right.0 as usize][front_right.1 as usize] = b']';
        true
    } else {
        false
    }
}

fn widen_grid(grid: &[Vec<u8>]) -> Vec<Vec<u8>> {
    fn widen_row(row: &[u8]) -> Vec<u8> {
        row.iter()
            .flat_map(|tile| match tile {
                b'@' => *b"@.",
                b'#' => *b"##",
                b'O' => *b"[]",
                b'.' => *b"..",
                _ => *b"..",
            })
            .collect()
    }

    grid.iter()
        .map(|row| widen_row(&row))
        .collect()
}

fn parse_input(input: impl io::BufRead) -> Result<PuzzleInput, Box<dyn Error>> {
    let mut grid = vec![];
    let mut moves = vec![];
    let mut robot = None;

    let mut is_first_section = true;
    for (line_num, line) in input.lines().enumerate() {
        let line = line?;

        if line.trim().is_empty() {
            is_first_section = false;
            continue;
        }

        if is_first_section {
            grid.push(Vec::from(line.as_bytes()));
            let row = grid.last_mut().unwrap();
            for (j, tile) in row.iter().enumerate() {
                if *tile == b'@' {
                    if robot.is_some() {
                        return Err("Multiple robot locations found in input".into());
                    }
                    robot = Some((line_num as i32, j as i32));
                }
            }
        } else {
            let parsed = line.chars().filter_map(|c| match c {
                '^' => Some((-1, 0)),
                'v' => Some((1, 0)),
                '<' => Some((0, -1)),
                '>' => Some((0, 1)),
                _ => None,
            });
            moves.extend(parsed);
        }
    }

    if let Some(robot) = robot {
        Ok(PuzzleInput{grid, robot, moves})
    } else {
        Err("No robot location found in input".into())
    }
}
