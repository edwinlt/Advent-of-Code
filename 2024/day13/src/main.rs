use std::io;
use std::error::Error;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin().lock();
    let input = parse_input(stdin)?;

    println!("Part 1: {}", solve_part_1(&input));
    // println!("Part 2: {}", solve_part_2(&input));
    Ok(())
}

fn solve_part_1(machines: &[ClawMachine]) -> i64 {
    let mut answer = 0;

    for machine in machines {
        let (px, py) = machine.prize;
        let (ax, ay) = machine.button_a;
        let (bx, by) = machine.button_b;

        let mut min_tokens = None;
        for a in 0 ..= 100 {
            let x = a * ax;
            let y = a * ay;

            if ax > px || ay > py {
                break;
            }

            let rx = px - x;
            let ry = py - y;
            if rx % bx != 0 || ry % by != 0 {
                continue;
            }
            if bx == 0 || by == 0 {
                continue;
            }
            
            let b = rx / bx;
            if b != ry / by {
                continue;
            }

            if min_tokens.is_some() {
                min_tokens = min_tokens.min(Some(3*a + b));
            } else {
                min_tokens = Some(3*a + b);
            }
        }

        answer += min_tokens.unwrap_or(0);
    }
    answer
}

#[derive(Debug, Default, Clone, Copy)]
struct ClawMachine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

fn parse_input(input: impl io::BufRead) -> Result<Vec<ClawMachine>, Box<dyn Error>> {
    let lines = input.lines().collect::<Result<Vec<_>, _>>()?;

    let button_a_rgx = Regex::new(r"^Button A: X\+(\d+), Y\+(\d+)$").unwrap();
    let button_b_rgx = Regex::new(r"^Button B: X\+(\d+), Y\+(\d+)$").unwrap();
    let prize_rgx = Regex::new(r"^Prize: X=(\d+), Y=(\d+)$").unwrap();

    let mut machines = vec![];
    for lines in lines.chunks(4) {
        let [line1, line2, line3, ..] = lines else {
            return Err("Invalid input".into());
        };
        if let Some(line4) = lines.get(3) {
            if !line4.trim().is_empty() {
                return Err("Invalid input".into());
            }
        }
        let mut machine = ClawMachine::default();

        if let Some(capture) = button_a_rgx.captures(line1.trim()) {
            let (_, [x, y]) = capture.extract();
            machine.button_a = (x.parse().unwrap(), y.parse().unwrap());
        } else {
            return Err("Invalid input".into());
        }

        if let Some(capture) = button_b_rgx.captures(line2.trim()) {
            let (_, [x, y]) = capture.extract();
            machine.button_b = (x.parse().unwrap(), y.parse().unwrap());
        } else {
            return Err("Invalid input".into());
        }

        if let Some(capture) = prize_rgx.captures(line3.trim()) {
            let (_, [x, y]) = capture.extract();
            machine.prize = (x.parse().unwrap(), y.parse().unwrap());
        } else {
            return Err("Invalid input".into());
        }

        machines.push(machine);
    }

    Ok(machines)
}
