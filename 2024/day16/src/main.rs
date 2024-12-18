use std::io;
use std::error::Error;
use std::collections::{BinaryHeap, HashSet};
use std::cmp::{PartialOrd, Ord, Ordering};
use std::num::Wrapping;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin().lock();
    let input = parse_input(stdin)?;

    let answers = solve(&input);
    println!("Part 1: {}", answers.0);
    println!("Part 2: {}", answers.1);
    Ok(())
}

struct PuzzleInput {
    maze: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {North = 0, South = 1, East = 2, West = 3}

impl Direction {
    fn i_offset(self) -> Wrapping<usize> {
        match self {
            Self::North => Wrapping(usize::MAX),
            Self::South => Wrapping(1),
            _ => Wrapping(0)
        }
    }
    fn j_offset(self) -> Wrapping<usize> {
        match self {
            Self::West => Wrapping(usize::MAX),
            Self::East => Wrapping(1),
            _ => Wrapping(0)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SearchState {
    pos: (usize, usize),
    dir: Direction,
    score: u64,
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score).reverse()
    }
}
impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score).map(Ordering::reverse)
    }
}

fn solve(input: &PuzzleInput) -> (u64, usize) {
    use Direction as D;
    let maze = &input.maze;
    let start = input.start;
    let end = input.end;

    let mut scores: Vec<Vec<_>> = maze.iter()
        .map(|row| vec![[u64::MAX; 4]; row.len()])
        .collect();
    scores[start.0][start.1][D::East as usize] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(SearchState{
        pos: start,
        dir: Direction::West,
        score: 0,
    });

    while let Some(current) = heap.pop() {
        let (i, j) = current.pos;
        if current.score > scores[i][j][current.dir as usize] {
            continue;
        }
        
        let edges = match current.dir {
            D::North | D::South => [(current.dir, 1), (D::East, 1001), (D::West, 1001)],
            D::East | D::West => [(current.dir, 1), (D::North, 1001), (D::South, 1001)],
        };

        for (next_dir, cost) in edges {
            let next_i = (Wrapping(i) + next_dir.i_offset()).0;
            let next_j = (Wrapping(j) + next_dir.j_offset()).0;

            if maze[next_i][next_j] == b'#' {
                continue;
            }

            let next_score = cost + current.score;
            let s = &mut scores[next_i][next_j][next_dir as usize];
            if next_score < *s {
                *s = next_score;
            } else {
                continue;
            }

            if (next_i, next_j) == end {
                continue;
            }

            heap.push(SearchState{
                pos: (next_i, next_j),
                score: next_score,
                dir: next_dir,
            });
        }
    }

    let min_score = *scores[end.0][end.1].iter().min().unwrap();

    let mut path_set = HashSet::new();
    for dir in [D::North, D::South, D::East, D::West] {
        if scores[end.0][end.1][dir as usize] == min_score {
            retrace_paths(&mut path_set, &scores, (end, dir));
        }
    }

    (min_score, path_set.len())
}

fn retrace_paths(out: &mut HashSet<(usize, usize)>, scores: &[Vec<[u64; 4]>], state: ((usize, usize), Direction)) {
    let ((i, j), dir) = state;
    out.insert((i, j));
    let score = scores[i][j][dir as usize];
    
    let prev_i = (Wrapping(i) - dir.i_offset()).0;
    let prev_j = (Wrapping(j) - dir.j_offset()).0;

    use Direction as D;
    let prev_dirs = match dir {
        D::North | D::South => [(dir, 1), (D::East, 1001), (D::West, 1001)],
        D::East | D::West => [(dir, 1), (D::North, 1001), (D::South, 1001)],
    };

    for (prev_dir, cost) in prev_dirs {
        if (score >= cost) && (score - cost == scores[prev_i][prev_j][prev_dir as usize]) {
            retrace_paths(out, scores, ((prev_i, prev_j), prev_dir));
        }
    }
}

fn parse_input(input: impl io::Read) -> Result<PuzzleInput, Box<dyn Error>> {
    let raw = io::read_to_string(input)?;
    let maze: Vec<_> = raw.lines()
        .map(Vec::from)
        .collect();

    let mut start = None;
    let mut end = None;
    for (i, row) in maze.iter().enumerate() {
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
    Ok(PuzzleInput{maze, start, end})
}
