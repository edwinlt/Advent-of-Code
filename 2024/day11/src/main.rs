use std::io;
use std::error::Error;
use std::iter::successors;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin().lock();
    let input = parse_input(stdin)?;

    println!("Part 1: {}", solve_part_1(input.clone()));
    println!("Part 2: {}", solve_part_2(input.clone()));
    Ok(())
}

fn solve_part_1(stones: Vec<usize>) -> usize {
    successors(Some(stones), |stones| Some(apply_rules(stones)))
        .nth(25).unwrap()
        .len()
}

fn solve_part_2(stones: Vec<usize>) -> usize {
    let mut memo = HashMap::new();
    stones.iter().map(|&stone| count_succesors(stone, 75, &mut memo)).sum()
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

fn count_succesors(stone: usize, depth: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    if depth == 0 {
        return 1;
    }

    if let Some(val) = memo.get(&(stone, depth)) {
        return *val;
    }
    
    let num_str = stone.to_string();
    let digit_count = num_str.len();

    let count = if stone == 0 {
        count_succesors(1, depth-1, memo)
    } else if digit_count % 2 == 0 {
        let (left, right) = num_str.split_at(digit_count/2);
        let left: usize = left.parse().unwrap();
        let right: usize = right.parse().unwrap();
        count_succesors(left, depth-1, memo)
            + count_succesors(right, depth-1, memo)
    } else {
        count_succesors(stone*2024, depth-1, memo)
    };
    
    memo.insert((stone, depth), count);
    count
}

fn parse_input(input: impl io::Read) -> Result<Vec<usize>, Box<dyn Error>> {
    io::read_to_string(input)?
        .split_whitespace()
        .map(str::parse::<usize>)
        .collect::<Result<_,_>>()
        .map_err(|err| err.into())
}
