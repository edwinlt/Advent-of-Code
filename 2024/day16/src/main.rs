use std::io;
use std::error::Error;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::cmp::{PartialOrd, Ord, Ordering};

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin().lock();
    let input = parse_input(stdin)?;

    println!("Part 1: {}", solve_part_1(&input));
    // println!("Part 2: {}", solve_part_2(&input));
    Ok(())
}

struct PuzzleInput {
    maze: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SearchState {
    pos: (usize, usize),
    dir: (usize, usize),
    score: usize,
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

fn solve_part_1(input: &PuzzleInput) -> usize {
    let maze = &input.maze;
    let start = input.start;
    let end = input.end;

    let mut visited = HashSet::new();
    let mut heap = BinaryHeap::new();
    heap.push(SearchState{
        pos: start,
        dir: (0, 1),
        score: 0,
    });

    let mut min_score = usize::MAX;
    while let Some(current) = heap.pop() {
        let (i, j) = current.pos;
        if current.pos == end {
            min_score = current.score.min(min_score);
            continue;
        }
        if maze[i][j] == b'#' || !visited.insert((current.pos, current.dir)) {
            continue;
        }
        
        let edges = match current.dir {
            (0, _) => [(current.dir, 1), ((1, 0), 1001), ((usize::MAX, 0), 1001)],
            (_, 0) => [(current.dir, 1), ((0, 1), 1001), ((0, usize::MAX), 1001)],
            _ => unreachable!()
        };

        for ((di, dj), cost) in edges {
            let next_i = i.overflowing_add(di).0;
            let next_j = j.overflowing_add(dj).0;
            heap.push(SearchState{
                score: cost + current.score,
                pos: (next_i, next_j),
                dir: (di, dj),
            });
        }
    }

    min_score
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
