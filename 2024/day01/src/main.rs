use std::{io, fs};

fn main() -> io::Result<()> {
    let file = fs::File::open("inputs/day01.txt")?;
    let input = io::read_to_string(file)?;

    println!("Part 1: {}", solve_part_1(&input));
    println!("Part 2: {}", solve_part_2(&input));
    Ok(())
}

fn solve_part_1(input: &str) -> String {
    "Not implemented".into()
}

fn solve_part_2(input: &str) -> i32 {
    "Not implemented".into()
}
