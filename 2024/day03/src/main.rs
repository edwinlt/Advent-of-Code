use std::io;
use std::error::Error;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin().lock();
    let input = io::read_to_string(stdin)?;

    println!("Part 1: {}", solve_part_1(&input));
    println!("Part 2: {}", solve_part_2(&input));
    Ok(())
}

fn solve_part_1(input: &str) -> i32 {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    let mut answer = 0;
    for (_, [x, y]) in re.captures_iter(input).map(|c| c.extract()) {
        let x = x.parse::<i32>().unwrap();
        let y = y.parse::<i32>().unwrap();
        answer += x * y;
    }
    answer
}

fn solve_part_2(input: &str) -> i32 {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    let mut answer = 0;
    let mut enabled = true;
    for capture in re.captures_iter(input) {
        match capture.get(0).unwrap().as_str() {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => if enabled {
                let x: i32 = capture.get(1).unwrap().as_str().parse().unwrap();
                let y: i32 = capture.get(2).unwrap().as_str().parse().unwrap();
                answer += x * y;
            }
        }
    }
    answer
}
