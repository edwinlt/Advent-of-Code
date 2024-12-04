use std::io;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin().lock();
    let input = parse_input(stdin)?;

    println!("Part 1: {}", solve_part_1(&input));
    println!("Part 2: {}", solve_part_2(&input));
    Ok(())
}

fn solve_part_1(reports: &[Vec<i32>]) -> usize {
    fn is_safe(report: &[i32]) -> bool {
        let sign = if report.len() > 1 {
            (report[1] - report[0]).signum()
        } else {
            return true;
        };

        report.windows(2)
            .map(|win| win[1] - win[0])
            .all(|d| (d.signum() == sign) && (1..=3).contains(&d.abs()))
    }

    reports.iter()
        .filter(|report| !report.is_empty())
        .filter(|report| is_safe(report))
        .count()
}

fn solve_part_2(reports: &[Vec<i32>]) -> usize {
    fn is_safe(report: &[i32], diff_range: std::ops::Range<i32>) -> bool {
        let n = report.len();
        let index_of_first_error =
            report.windows(2).enumerate()
                .find(|(_, win)| !diff_range.contains(&(win[1] - win[0])))
                .map(|(i, _)| i + 1)
                .unwrap_or(n);
        if index_of_first_error >= n-1 {return true;}
        let i = index_of_first_error;

        let can_remove_ith = diff_range.contains(&(report[i+1] - report[i-1]));
        let can_remove_prev = (i < 2 || diff_range.contains(&(report[i] - report[i-2])))
            && diff_range.contains(&(report[i+1] - report[i]));

        let is_safe_after_first_error =
            report[i+1 .. ].windows(2)
                .map(|win| win[1] - win[0])
                .all(|d| diff_range.contains(&d));

        is_safe_after_first_error && (can_remove_ith || can_remove_prev)
    }

    reports.iter()
        .filter(|report| !report.is_empty())
        .filter(|report| is_safe(report, 1..4) || is_safe(report, -3..0))
        .count()
}

fn parse_input(input: impl io::BufRead) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    fn parse_line(line: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
        line.split_whitespace().map(str::parse::<i32>).collect()
    }

    input.lines()
        .map(|line | Ok( parse_line(&line?)? ))
        .collect()
}
