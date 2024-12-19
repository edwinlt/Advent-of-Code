use std::io;
use std::error::Error;
use std::collections::VecDeque;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin().lock();
    let input = parse_input(stdin)?;

    println!("Part 1: {}", solve_part_1(&input));
    println!("Part 2: {}", solve_part_2(&input));
    Ok(())
}

const MAX_X: usize = 70;
const MAX_Y: usize = 70;

fn solve_part_1(coords: &[(usize, usize)]) -> u64 {
    let mut blocked = [[false; MAX_Y+1]; MAX_X+1];
    for (x, y) in coords.iter().take(1024) {
        blocked[*x][*y] = true;
    }
    bfs(&blocked, (0, 0), (MAX_X, MAX_Y)).unwrap()
}

fn solve_part_2(coords: &[(usize, usize)]) -> String {
    let mut blocked = [[false; MAX_Y+1]; MAX_X+1];
    for (x, y) in coords.iter() {
        blocked[*x][*y] = true;
        if bfs(&blocked, (0, 0), (MAX_X, MAX_Y)).is_none() {
            return format!("{x},{y}");
        }
    }

    String::from("No answer found")
}

fn bfs(blocked: &[[bool; MAX_X+1]; MAX_Y+1], start: (usize, usize), end: (usize, usize)) -> Option<u64> {
    let mut visited = [[false; MAX_Y+1]; MAX_X+1];
    visited[0][0] = true;

    let mut queue = VecDeque::new();
    queue.push_back((start.0, start.1, 0));

    while let Some((x, y, dist)) = queue.pop_front() {
        let neighbors = [
            (x.overflowing_sub(1).0, y),
            (x, y.overflowing_sub(1).0),
            (x+1, y),
            (x, y+1),
        ];

        for (nx, ny) in neighbors {
            if nx > MAX_X || ny > MAX_Y || blocked[nx][ny] || visited[nx][ny] {
                continue;
            }
            if (nx, ny) == end {
                return Some(dist+1);
            }
            visited[nx][ny] = true;
            queue.push_back((nx, ny, dist+1));
        }
    }

    None
}

fn parse_input(input: impl io::BufRead) -> Result<Vec<(usize, usize)>, Box<dyn Error>> {
    let mut coords = vec![];
    for line in input.lines() {
        let line = line?;
        let (x, y) = line.split_once(',').ok_or("Invalid input")?;
        coords.push((x.parse()?, y.parse()?));
    }
    Ok(coords)
}
