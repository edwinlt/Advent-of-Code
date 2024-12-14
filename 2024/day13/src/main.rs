use std::io;
use std::error::Error;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin().lock();
    let input = parse_input(stdin)?;

    println!("Part 1: {}", solve_part_1(&input));
    println!("Part 2: {}", solve_part_2(&input));
    Ok(())
}

fn solve_part_1(machines: &[ClawMachine]) -> i64 {
    machines.iter()
        .map(tokens_to_win)
        .filter_map(|opt| opt)
        .sum()
}

fn solve_part_2(machines: &[ClawMachine]) -> i64 {
    let p = 10_000_000_000_000;
    machines.iter()
        .map(|machine| machine.adjust_prize_location(p, p))
        .map(|machine| tokens_to_win(&machine))
        .filter_map(|opt| opt)
        .sum()
}

fn tokens_to_win(machine: &ClawMachine) -> Option<i64> {
    let (p_x, p_y) = machine.prize;
    let (a_x, a_y) = machine.button_a;
    let (b_x, b_y) = machine.button_b;

    // Solve for a and b
    // p_x = (a * a_x) + (b * b_x)
    // p_y = (a * a_y) + (b * b_y)
    let det = (a_x * b_y) - (a_y * b_x);
    let a = (p_x * b_y) + (p_y * -b_x);
    let b = (p_x * -a_y) + (p_y * a_x);
    if det == 0 || a % det != 0 || b % det != 0 {
        return None;
    }

    let a = a / det;
    let b = b / det;
    Some(3*a + b)
}


#[derive(Debug, Default, Clone, Copy)]
struct ClawMachine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

impl ClawMachine {
    fn adjust_prize_location(self, dx: i64, dy: i64) -> Self {
        let (px, py) = self.prize;
        Self {
            prize: (px+dx, py+dy),
            ..self
        }
    }
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
