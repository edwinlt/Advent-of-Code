use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin().lock();
    let input = parse_input(stdin)?;

    println!("Part 1: {}", solve_part_1(&input));
    println!("Part 2: {}", solve_part_2(&input));
    Ok(())
}

struct PuzzleInput{
    grid: Vec<Vec<usize>>,
    region_count: usize,
}

fn solve_part_1(input: &PuzzleInput) -> usize {
    let grid = &input.grid;
    let mut areas = vec![0; input.region_count];
    let mut perimeters = vec![0; input.region_count];

    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            areas[cell] += 1;

            let left = j.overflowing_sub(1).0;
            let up = i.overflowing_sub(1).0;
            let neighbors = [(i, j + 1), (i, left), (i + 1, j), (up, j)];

            let perim = &mut perimeters[cell];
            for (i2, j2) in neighbors {
                if i2 >= grid.len() || j2 >= grid[i2].len() || grid[i2][j2] != cell {
                    *perim += 1;
                }
            }
        }
    }

    std::iter::zip(areas, perimeters)
        .map(|(a, p)| a * p)
        .sum()
}

fn solve_part_2(input: &PuzzleInput) -> usize {
    fn get_neighborhood(grid: &[Vec<usize>], i: usize, j: usize) -> [[usize; 3]; 3] {
        let mut neighborhood = [[usize::MAX; 3]; 3];
        for ni in 0 .. 3 {
            let i2 = (i + ni).overflowing_sub(1).0;
            for nj in 0 .. 3 {
                let j2 = (j + nj).overflowing_sub(1).0;
                if i2 < grid.len() && j2 < grid[i2].len() {
                    neighborhood[ni][nj] = grid[i2][j2];
                }
            }
        }
        neighborhood
    }

    let grid = &input.grid;
    let mut areas = vec![0; input.region_count];
    let mut corners = vec![0; input.region_count];

    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            let neighborhood = get_neighborhood(grid, i, j);

            let corner_count = &mut corners[cell];

            // Top left
            if neighborhood[0][0] != cell && neighborhood[0][1] == cell && neighborhood[1][0] == cell {
                *corner_count += 1;
            } else if neighborhood[0][1] != cell && neighborhood[1][0] != cell {
                *corner_count += 1;
            }

            // Top right
            if neighborhood[0][2] != cell && neighborhood[0][1] == cell && neighborhood[1][2] == cell {
                *corner_count += 1;
            } else if neighborhood[0][1] != cell && neighborhood[1][2] != cell {
                *corner_count += 1;
            }

            // Bottom left
            if neighborhood[2][0] != cell && neighborhood[2][1] == cell && neighborhood[1][0] == cell {
                *corner_count += 1;
            } else if neighborhood[2][1] != cell && neighborhood[1][0] != cell {
                *corner_count += 1;
            }

            // Bottom right
            if neighborhood[2][2] != cell && neighborhood[2][1] == cell && neighborhood[1][2] == cell {
                *corner_count += 1;
            } else if neighborhood[2][1] != cell && neighborhood[1][2] != cell {
                *corner_count += 1;
            }

            areas[cell] += 1;
        }
    }

    std::iter::zip(areas, corners)
        .map(|(a, c)| a * c)
        .sum()
}

fn parse_input(input: impl io::Read) -> Result<PuzzleInput, Box<dyn Error>> {
    fn flood_fill(out: &mut Vec<Vec<usize>>, val: usize, original: &[&[u8]], i: usize, j: usize) -> bool {
        if out[i][j] != usize::MAX {
            return false;
        }
        out[i][j] = val;

        let current_cell = original[i][j];
    
        let left = j.overflowing_sub(1).0;
        let up = i.overflowing_sub(1).0;
        let neighbors = [(i, j + 1), (i, left), (i + 1, j), (up, j)];

        for (i2, j2) in neighbors {
            if i2 < original.len() && j2 < original[i2].len() && original[i2][j2] == current_cell {
                flood_fill(out, val, original, i2, j2);
            }
        }
        true
    }

    let input: String = io::read_to_string(input)?;
    let input: Vec<_> = input.lines().map(str::as_bytes).collect();

    let mut grid: Vec<Vec<_>> = input.iter()
        .map(|row| vec![usize::MAX; row.len()])
        .collect();
    let mut regions = 0;

    for i in 0 .. grid.len() {
        for j in 0 .. grid[i].len() {
            if flood_fill(&mut grid, regions, &input, i, j) {
                regions += 1;
            }
        }
    }

    Ok(PuzzleInput{grid, region_count: regions})
}
