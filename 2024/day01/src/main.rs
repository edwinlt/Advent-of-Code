use std::{io, iter};
use std::error::Error;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin().lock();
    let input = io::read_to_string(stdin)?;
    assert!(!input.is_empty(), "Input was empty!");

    println!("Part 1: {}", solve_part_1(&input)?);
    println!("Part 2: {}", solve_part_2(&input)?);
    Ok(())
}

fn solve_part_1(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let (left_num, right_num) = parse_line(line)?;

        left.push(left_num);
        right.push(right_num);
    }

    left.sort();
    right.sort();

    let answer = iter::zip(left, right)
        .map(|pair| (pair.0 - pair.1).abs())
        .sum();
    Ok(answer)
}

fn solve_part_2(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut left = HashMap::new();
    let mut right = HashMap::new();
    let mut answer = 0;

    for line in input.lines() {
        let (left_num, right_num) = parse_line(line)?;

        answer += left_num * right.get(&left_num).unwrap_or(&0);
        *left.entry(left_num).or_insert(0) += 1;

        answer += right_num * left.get(&right_num).unwrap_or(&0);
        *right.entry(right_num).or_insert(0) += 1;
    }

    Ok(answer)
}

fn parse_line(line: &str) -> Result<(i32, i32), Box<dyn Error>> {
    let mut split = line.split_whitespace();
    let first_num  = split.next().unwrap_or("").parse()?;
    let second_num = split.next().unwrap_or("").parse()?;

    Ok((first_num, second_num))
}
