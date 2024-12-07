use std::io;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin().lock();
    let input = parse_input(stdin)?;

    println!("Part 1: {}", solve_part_1(&input));
    println!("Part 2: {}", solve_part_2(&input));
    Ok(())
}

fn solve_part_1(input: &[(u64, Vec<u64>)]) -> u64 {
    fn eq_is_posible(val: u64, operands: &[u64]) -> bool {
        let Some(&first) = operands.first() else {
            return false;
        };
        let operands = &operands[1 .. ];
        let max_flagset: u128 = 1 << operands.len();
        
        (0 .. max_flagset).any(|flags| {
            let mut accumulator = first;
            for (i, operand) in operands.iter().enumerate() {
                let bit = 1 & (flags >> i);
                if bit == 0 {
                    accumulator += operand;
                } else {
                    accumulator *= operand;
                }
            }
            
            accumulator == val
        })
    }

    input.iter()
        .filter(|(val, nums)| eq_is_posible(*val, nums))
        .map(|(val, _)| val)
        .sum()
}

fn solve_part_2(input: &[(u64, Vec<u64>)]) -> u64 {
    fn eq_is_posible(acc: u64, target: u64, operands: &[u64]) -> bool {
        if acc > target {
            false
        } else if let Some(first) = operands.first() {
            let concat = format!("{acc}{first}").parse().unwrap();
            let tail = &operands[1..];
            eq_is_posible(concat, target, tail)
                || eq_is_posible(acc+first, target, tail)
                || eq_is_posible(acc*first, target, tail)
        } else {
            target == acc
        }
    }

    input.iter()
        .filter(|(val, nums)| 
            if let Some(&first) = nums.first() {
                eq_is_posible(first, *val, &nums[1..])
            } else {
                false
            }
        )
        .map(|(val, _)| val)
        .sum()
}

fn parse_input(input: impl io::BufRead) -> Result<Vec<(u64, Vec<u64>)>, Box<dyn Error>> {
    fn parse_line(line: &str) -> Result<(u64, Vec<u64>), Box<dyn Error>> {
        let parts = line.split_once(':')
            .ok_or_else(|| "Invalid input format")?;

        let value: u64 = parts.0.parse()?;
        let operands = parts.1.trim().split_whitespace()
            .map(str::parse::<u64>)
            .collect::<Result<Vec<_>, _>>()?;

        Ok((value, operands))
    }

    input.lines()
        .map(|line| parse_line(&line?))
        .collect()
}
