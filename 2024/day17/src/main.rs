use std::io;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin().lock();
    let input = parse_input(stdin)?;

    println!("Part 1: {}", solve_part_1(&input));
    // println!("Part 2: {}", solve_part_2(&input));
    Ok(())
}

#[derive(Debug, Clone)]
struct PuzzleInput {
    register_a: i64,
    register_b: i64,
    register_c: i64,
    program: Vec<u8>,
}

fn solve_part_1(input: &PuzzleInput) -> String {
    let mut out = String::new();
    let mut register_a = input.register_a;
    let mut register_b = input.register_b;
    let mut register_c = input.register_c;
    let mut ip = 0;


    while let Some(&opcode) = input.program.get(ip) {
        let literal_operand = input.program.get(ip + 1)
            .map(|&operand| operand as i64)
            .unwrap();
        let combo_operand = match literal_operand {
            0 | 1 | 2 | 3 => literal_operand,
            4 => register_a,
            5 => register_b,
            6 => register_c,
            _ => unreachable!(),
        };

        match opcode {
            0 => register_a = register_a / (1 << combo_operand),
            1 => register_b = register_b ^ literal_operand,
            2 => register_b = combo_operand % 8,
            3 => if register_a != 0 {
                ip = literal_operand as usize;
                continue;
            }
            4 => register_b = register_b ^ register_c,
            5 => {
                if !out.is_empty() {
                    out.push(',');
                }
                out.push_str(&(combo_operand % 8).to_string());
            }
            6 => register_b = register_a / (1 << combo_operand),
            7 => register_c = register_a / (1 << combo_operand),
            _ => unreachable!()
        }

        ip += 2;
    }
    out
}

fn parse_input(input: impl io::BufRead) -> Result<PuzzleInput, Box<dyn Error>> {
    let mut lines = input.lines();
    
    let line = lines.next().ok_or("Unexpected end of file")??;
    let Some(("Register A", a)) = line.split_once(':') else {
        return Err("Invalid input")?;
    };
    let register_a = a.trim().parse()?;

    let line = lines.next().ok_or("Unexpected end of file")??;
    let Some(("Register B", b)) = line.split_once(':') else {
        return Err("Invalid input")?;
    };
    let register_b = b.trim().parse()?;

    let line = lines.next().ok_or("Unexpected end of file")??;
    let Some(("Register C", c)) = line.split_once(':') else {
        return Err("Invalid input")?;
    };
    let register_c = c.trim().parse()?;

    let line = lines.skip(1).next().ok_or("Unexpected end of file")??;
    let Some(("Program", program)) = line.split_once(':') else {
        return Err("Invalid input")?;
    };
    let program = program.split(',')
        .map(|s| s.trim().parse())
        .collect::<Result<_,_>>()?;

    Ok(PuzzleInput{register_a, register_b, register_c, program})
}
