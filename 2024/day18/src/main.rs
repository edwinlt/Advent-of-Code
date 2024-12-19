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
    let mut map = [[usize::MAX; MAX_Y+1]; MAX_X+1];
    for (i, (x, y)) in coords.iter().enumerate().take(1024) {
        map[*x][*y] = i;
    }
    bfs(&map, 1024).unwrap()
}

fn solve_part_2(coords: &[(usize, usize)]) -> String {
    let mut map = [[usize::MAX; MAX_Y+1]; MAX_X+1];
    for (i, (x, y)) in coords.iter().enumerate() {
        map[*x][*y] = i;
    }
    
    let mut lo = 0;
    let mut hi = coords.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if let Some(_) = bfs(&map, mid+1) {
            lo = mid+1;
        } else {
            hi = mid;
        }
    }

    if let Some((x, y)) = coords.get(lo) {
        format!("{x},{y}")
    } else {
        String::from("No answer found")
    }
}

fn bfs(map: &[[usize; MAX_Y+1]; MAX_X+1], t: usize) -> Option<u64> {
    let mut visited = [[false; MAX_Y+1]; MAX_X+1];
    visited[0][0] = true;

    let mut queue = VecDeque::new();
    queue.push_back((0usize, 0usize, 0));

    while let Some((x, y, dist)) = queue.pop_front() {
        let neighbors = [
            (x.overflowing_sub(1).0, y),
            (x, y.overflowing_sub(1).0),
            (x+1, y),
            (x, y+1),
        ];

        for (nx, ny) in neighbors {
            if nx > MAX_X || ny > MAX_Y || visited[nx][ny] {
                continue;
            }
            if map[nx][ny] < t {
                continue;
            }
            if (nx, ny) == (MAX_X, MAX_Y) {
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
