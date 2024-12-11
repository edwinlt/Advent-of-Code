use std::io;
use std::error::Error;
use std::iter::successors;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin().lock();
    let input = parse_input(stdin)?;

    println!("Part 1: {}", solve_part_1(input.clone()));
    //println!("Part 2: {}", solve_part_2(input.clone()));
    Ok(())
}

fn solve_part_1(stones: Vec<usize>) -> usize {
    successors(Some(stones), |stones| Some(apply_rules(stones)))
        .nth(25).unwrap()
        .len()
}

fn solve_part_2(stones: Vec<usize>) -> usize {
    successors(Some(stones), |stones| Some(apply_rules(stones)))
        .nth(75).unwrap()
        .len()
}

fn apply_rules(stones: &[usize]) -> Vec<usize> {
    let mut next = vec![];
    for stone in stones {
        if *stone == 0 {
            next.push(1);
            continue;
        }

        let num_str = stone.to_string();
        let digit_count = num_str.len();
        if digit_count % 2 == 0 {
            let (left, right) = num_str.split_at(digit_count/2);
            next.push(left.parse().unwrap());
            next.push(right.parse().unwrap());
        } else {
            next.push(stone * 2024);
        }
    }
    next
}

fn parse_input(input: impl io::Read) -> Result<Vec<usize>, Box<dyn Error>> {
    io::read_to_string(input)?
        .split_whitespace()
        .map(str::parse::<usize>)
        .collect::<Result<_,_>>()
        .map_err(|err| err.into())
}
