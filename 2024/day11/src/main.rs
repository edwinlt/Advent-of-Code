use std::io;
use std::error::Error;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin().lock();
    let input = parse_input(stdin)?;

    println!("Part 1: {}", solve_part_1(&input));
    println!("Part 2: {}", solve_part_2(&input));
    Ok(())
}

fn solve_part_1(stones: &[usize]) -> usize {
    let mut memo = HashMap::new();
    stones.iter().map(|&stone| count_resulting_stones(stone, 25, &mut memo)).sum()
}

fn solve_part_2(stones: &[usize]) -> usize {
    let mut memo = HashMap::new();
    stones.iter().map(|&stone| count_resulting_stones(stone, 75, &mut memo)).sum()
}

fn count_resulting_stones(stone: usize, blinks: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    if blinks == 0 {
        return 1;
    }

    if let Some(&count) = memo.get(&(stone, blinks)) {
        return count;
    }

    let count = if stone == 0 {
        count_resulting_stones(1, blinks-1, memo)
    } else {
        let num_str = stone.to_string();
        let digit_count = num_str.len();

        if digit_count % 2 == 0 {
            let (left, right) = num_str.split_at(digit_count/2);
            let left: usize = left.parse().unwrap();
            let right: usize = right.parse().unwrap();
            count_resulting_stones(left, blinks-1, memo)
                + count_resulting_stones(right, blinks-1, memo)
        } else {
            count_resulting_stones(stone*2024, blinks-1, memo)
        }
    };

    memo.insert((stone, blinks), count);
    count
}

fn parse_input(input: impl io::Read) -> Result<Vec<usize>, Box<dyn Error>> {
    io::read_to_string(input)?
        .split_whitespace()
        .map(str::parse::<usize>)
        .collect::<Result<_,_>>()
        .map_err(|err| err.into())
}
