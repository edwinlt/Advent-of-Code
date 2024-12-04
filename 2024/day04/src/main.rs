use std::io;
use std::error::Error;
use itertools::izip;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin().lock();
    let input: String = io::read_to_string(stdin)?;
    let input: Vec<_> = input.lines().map(str::as_bytes).collect();

    println!("Part 1: {}", solve_part_1(&input));
    println!("Part 2: {}", solve_part_2(&input));
    Ok(())
}

fn solve_part_1(grid: &[&[u8]]) -> usize {
    let horizontal_matches = grid.iter()
        .flat_map(|row| row.windows(4))
        .filter(|word| word == b"XMAS" || word == b"SAMX")
        .count();

    let mut vertical_matches = 0;
    let mut diagonal_matches = 0;
    for rows in grid.windows(4) {
        for col in izip!(rows[0], rows[1], rows[2], rows[3]) {
            let word: [u8; 4]= [*col.0, *col.1, *col.2, *col.3];
            if (&word == b"XMAS") || (&word == b"SAMX") {
                vertical_matches += 1;
            }
        }
        for diag in izip!(rows[0], &rows[1][1..], &rows[2][2..], &rows[3][3..]) {
            let word: [u8; 4]= [*diag.0, *diag.1, *diag.2, *diag.3];
            if (&word == b"XMAS") || (&word == b"SAMX") {
                diagonal_matches += 1;
            }
        }
        for diag in izip!(rows[3], &rows[2][1..], &rows[1][2..], &rows[0][3..]) {
            let word: [u8; 4]= [*diag.0, *diag.1, *diag.2, *diag.3];
            if (&word == b"XMAS") || (&word == b"SAMX") {
                diagonal_matches += 1;
            }
        }
    }

    horizontal_matches + vertical_matches + diagonal_matches
}

fn solve_part_2(grid: &[&[u8]]) -> usize {
    let mut matches = 0;
    for rows in grid.windows(3) {
        for subgrid in izip!(rows[0].windows(3), rows[1].windows(3), rows[2].windows(3)) {
            if subgrid.1[1] != b'A' {
                continue;
            }

            let corners = [subgrid.0[0], subgrid.0[2], subgrid.2[2], subgrid.2[0]];
            if (&corners == b"MMSS") || (&corners == b"MSSM") || (&corners == b"SSMM") || (&corners == b"SMMS") {
                matches += 1;
            }
        }
    }

    matches
}
