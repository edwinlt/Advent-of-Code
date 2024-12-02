use std::io;
use std::error::Error;
use std::ops::Range;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin().lock();
    let input = parse_input(stdin)?;

    println!("Part 1: {}", solve_part_1(&input));
    println!("Part 2: {}", solve_part_2(&input));
    Ok(())
}

fn solve_part_1(reports: &[Vec<i32>]) -> usize {
    fn is_safe(report: &[i32]) -> bool {
        let sign = (report[1] - report[0]).signum();
        report.windows(2)
            .map(|win| win[1] - win[0])
            .all(|d| (d.signum() == sign) && (1..=3).contains(&d.abs()))
    }

    reports.iter()
        .filter(|report| is_safe(report))
        .count()
}

fn solve_part_2(reports: &[Vec<i32>]) -> usize {
    fn is_safe(report: &[i32], diff_range: Range<i32>) -> bool {
        let n = report.len();
        let mut dp = vec![[true, true]; n];

        for (i, x) in report.iter().enumerate().skip(1) {
            dp[i][0] = dp[i-1][0] && diff_range.contains(&(x - report[i-1]));
        }
        for (i, x) in report.iter().enumerate().skip(2) {
            dp[i][1] = (dp[i-1][1] && diff_range.contains(&(x - report[i-1])))
                    || (dp[i-2][0] && diff_range.contains(&(x - report[i-2])));
        }

        dp[n-1][0] || dp[n-1][1] || dp[n-2][0]
    }

    reports.iter()
        .filter(|report| is_safe(report, 1..4) || is_safe(report, -3..0))
        .count()
}

fn parse_input(input: impl io::BufRead) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let mut reports = vec![];
    for line in input.lines() {
        let report = Result::from_iter(
            line?.split_whitespace().map(str::parse::<i32>)
        )?;
        reports.push(report);
    }
    Ok(reports)
}
