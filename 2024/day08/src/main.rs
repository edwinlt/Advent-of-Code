use std::io;
use std::error::Error;
use std::collections::{HashMap, HashSet};
use std::num::Wrapping;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin().lock();
    let input: String = io::read_to_string(stdin)?;
    let input: Vec<_> = input.lines().map(str::as_bytes).collect();

    println!("Part 1: {}", solve_part_1(&input));
    println!("Part 2: {}", solve_part_2(&input));
    Ok(())
}

fn solve_part_1(map: &[&[u8]]) -> usize {
    let mut antinodes = HashSet::new();

    for (_freq, antennas) in find_all_antennas(map) {
        for (i, antenna_1) in antennas.iter().enumerate() {
            for antenna_2 in antennas[i+1..].iter() {
                let (r1, c1) = (Wrapping(antenna_1.0), Wrapping(antenna_1.1));
                let (r2, c2) = (Wrapping(antenna_2.0), Wrapping(antenna_2.1));

                let delta_r = r2 - r1;
                let delta_c = c2 - c1;

                antinodes.insert((r2 + delta_r, c2 + delta_c));
                antinodes.insert((r1 - delta_r, c1 - delta_c));
            }
        }
    }
    
    antinodes.iter()
        .filter(|(r, c)|
            is_in_bounds((r.0, c.0), map)
        )
        .count()
}

fn solve_part_2(map: &[&[u8]]) -> usize {
    let mut antinodes = HashSet::new();

    for (_freq, antennas) in find_all_antennas(map) {
        for (i, antenna_1) in antennas.iter().enumerate() {
            for antenna_2 in antennas[i+1..].iter() {
                let (r1, c1) = (Wrapping(antenna_1.0), Wrapping(antenna_1.1));
                let (r2, c2) = (Wrapping(antenna_2.0), Wrapping(antenna_2.1));

                let delta_r = r2 - r1;
                let delta_c = c2 - c1;

                let (mut y, mut x) = (r2, c2);
                while is_in_bounds((y.0, x.0), map) {
                    antinodes.insert((y, x));
                    y += delta_r;
                    x += delta_c;
                }

                (y, x) = (r1, c1);
                while is_in_bounds((y.0, x.0), map) {
                    antinodes.insert((y, x));
                    y -= delta_r;
                    x -= delta_c;
                }
            }
        }
    }
    
    antinodes.len()
}

fn find_all_antennas(map: &[&[u8]]) -> HashMap<u8, Vec<(usize, usize)>> {
    let mut antennas = HashMap::<u8, Vec<(usize, usize)>>::new();
    for (i, row) in map.iter().enumerate() {
        for (j, node) in row.iter().enumerate() {
            if !node.is_ascii_alphanumeric() {
                continue;
            }
            antennas.entry(*node).or_default()
                .push((i, j));
        }
    }
    antennas
}

fn is_in_bounds(coords: (usize, usize), grid: &[&[u8]]) -> bool {
    grid.get(coords.0)
        .map(|&row| coords.1 < row.len())
        .unwrap_or_else(|| false)
}
