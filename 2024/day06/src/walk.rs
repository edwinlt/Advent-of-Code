use std::collections::HashSet;

pub struct WalkIterator<'a> {
    grid: &'a [&'a [u8]],
    i: usize,
    j: usize,
    dir: Direction,
    obstacles: Vec<(usize, usize)>,
}

impl<'a> Iterator for WalkIterator<'a> {
    type Item = (usize, usize, Direction);

    fn next(&mut self) -> Option<Self::Item> {
        let (i, j, dir) = (self.i, self.j, self.dir);
        if i >= self.grid.len() || j >= self.grid[i].len() {
            return None;
        }

        for _ in 0..4 {
            let (i2, j2) = self.dir.offset_coords(i, j);

            let in_front = self.grid.get(i2).and_then(|row| row.get(j2));
            if let Some(b'#') = in_front {
                self.dir = dir.rotate();
            } else if self.obstacles.contains(&(i2, j2)) {
                self.dir = dir.rotate();
            } else {
                self.i = i2;
                self.j = j2;
                break;
            }
        }

        Some((i, j, dir))
    }
}

impl<'a> WalkIterator<'a> {
    pub fn new(grid: &'a [&'a [u8]], start: (usize, usize), dir: Direction) -> Self {
        Self{
            grid,
            i: start.0,
            j: start.1,
            dir,
            obstacles: vec![]
        }
    }

    pub fn with_added_obstacle(mut self, coords: (usize, usize)) -> Self {
        self.obstacles.push(coords);
        self
    }

    pub fn is_inifinite_loop(self) -> bool {
        let mut seen_states = HashSet::new();
        for state in self {
            if seen_states.contains(&state) {
                return true;
            }
            seen_states.insert(state);
        }
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {Up, Right, Down, Left}
impl Direction {
    fn rotate(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up
        }
    }

    fn offset_coords(self, i: usize, j: usize) -> (usize, usize) {
        let i2 = match self {
            Direction::Up   => i.overflowing_sub(1).0,
            Direction::Down => i.overflowing_add(1).0,
            _ => i
        };
        let j2 = match self {
            Direction::Left  => j.overflowing_sub(1).0,
            Direction::Right => j.overflowing_add(1).0,
            _ => j
        };
        (i2, j2)
    }
}
