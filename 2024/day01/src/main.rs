use std::{io, iter};
use std::error::Error;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin().lock();
    let input = parse_input(stdin)?;

    println!("Part 1: {}", solve_part_1(&input));
    println!("Part 2: {}", solve_part_2(&input));
    Ok(())
}

fn solve_part_1(input: &[(i32, i32)]) -> i32 {
    let mut left:  Vec<i32>;
    let mut right: Vec<i32>;
    (left, right) = input.iter().cloned().unzip();

    left.sort();
    right.sort();

    iter::zip(left, right)
        .map(|pair| (pair.0 - pair.1).abs())
        .sum()
}

fn solve_part_2(input: &[(i32, i32)]) -> i32 {
    let mut right_count = HashMap::new();
    for (_, right_num) in input {
        *right_count.entry(right_num).or_insert(0) += 1;
    }

    input.iter()
        .map(|(left_num, _)|
            left_num * right_count.get(&left_num).unwrap_or(&0)
        )
        .sum()
}

fn parse_input(input: impl io::BufRead) -> Result<Vec<(i32, i32)>, Box<dyn Error>> {
    fn parse_line(line: &str) -> Result<(i32, i32), Box<dyn Error>> {
        let mut split = line.split_whitespace();
        let first_num  = split.next().ok_or("Missing number in line")?.parse()?;
        let second_num = split.next().ok_or("Missing number in line")?.parse()?;
        Ok((first_num, second_num))
    }

    input.lines()
        .map(|line| parse_line(&line?))
        .collect()
}
