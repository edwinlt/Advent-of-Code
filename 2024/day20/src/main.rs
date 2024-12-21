use std::io;
use std::error::Error;
use std::collections::VecDeque;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin().lock();
    let input = parse_input(stdin)?;

    println!("Part 1: {}", solve_part_1(&input));
    // println!("Part 2: {}", solve_part_2(&input));
    Ok(())
}

struct PuzzleInput {
    map: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
}

fn solve_part_1(input: &PuzzleInput) -> u64 {
    let start = input.start;
    let end = input.end;

    let dists = bfs(&input.map, start, end);
    let end_dist = dists[end.0][end.1];

    let mut answer = 0;
    let mut new_map = input.map.clone();
    for i in 0..new_map.len() {
        for j in 0..new_map[i].len() {
            if new_map[i][j] != b'#' || dists[i][j] >= end_dist {
                continue;
            }

            new_map[i][j] = b'.';
            let new_dist = bfs(&new_map, input.start, end)[end.0][end.1];
            if new_dist + 100 <= end_dist {
                answer += 1;
            }
            new_map[i][j] = b'#';
        }
    }
    answer
}

fn bfs(maze: &[Vec<u8>], start: (usize, usize), end: (usize, usize)) -> Vec<Vec<u64>> {
    let mut dist = maze.iter()
        .map(|row| vec![u64::MAX; row.len()])
        .collect::<Vec<_>>();
    dist[0][0] = 0;

    let mut queue = VecDeque::new();
    queue.push_back((start.0, start.1, 0));

    while let Some((i, j, current_dist)) = queue.pop_front() {
        for (next_i, next_j) in neighbors(i, j) {
            if next_i >= maze.len() || next_j >= maze[next_i].len() {
                continue;
            }

            let next_dist = current_dist + 1;
            if next_dist >= dist[next_i][next_j] {
                continue;
            }
            dist[next_i][next_j] = next_dist;

            if (next_i, next_j) == end || maze[next_i][next_j] == b'#' {
                continue;
            }

            queue.push_back((next_i, next_j, next_dist));
        }
    }

    dist
}

fn neighbors(i: usize, j: usize) -> [(usize, usize); 4] {
    let up = (i.overflowing_sub(1).0, j);
    let left = (i, j.overflowing_sub(1).0);
    let down = (i+1, j);
    let right = (i, j+1);
    [up, down, left, right]
}

fn parse_input(input: impl io::Read) -> Result<PuzzleInput, Box<dyn Error>> {
    let raw = io::read_to_string(input)?;
    let map: Vec<_> = raw.lines()
        .map(Vec::from)
        .collect();

    let mut start = None;
    let mut end = None;
    for (i, row) in map.iter().enumerate() {
        for (j, byte) in row.iter().enumerate() {
            if *byte == b'S' {
                start = Some((i, j));
            } else if *byte == b'E' {
                end = Some((i, j));
            }
        }
    }

    let Some(start) = start else {
        return Err("No start location".into());
    };
    let Some(end) = end else {
        return Err("No end location".into());
    };
    Ok(PuzzleInput{map, start, end})
}
