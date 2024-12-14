use std::io;
use std::error::Error;
use regex::Regex;
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin().lock();
    let input = parse_input(stdin)?;

    println!("Part 1: {}", solve_part_1(&input));
    println!("Part 2: {}", solve_part_2(&input));
    Ok(())
}

const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;

fn solve_part_1(robots: &[Robot]) -> usize {
    let mut quadrants = [0, 0, 0, 0];
    for robot in robots {
        let x = (robot.pos.0 + 100*robot.vel.0).rem_euclid(WIDTH);
        let y = (robot.pos.1 + 100*robot.vel.1).rem_euclid(HEIGHT);

        let is_left = x < WIDTH/2;
        let is_right = x > WIDTH/2;
        let is_top = y < HEIGHT/2;
        let is_bottom = y > HEIGHT/2;

        if is_top && is_left {
            quadrants[0] += 1;
        }
        if is_top && is_right {
            quadrants[1] += 1;
        }
        if is_bottom && is_left {
            quadrants[2] += 1;
        }
        if is_bottom && is_right {
            quadrants[3] += 1;
        }
    }

    quadrants.iter().product()
}

fn solve_part_2(robots: &[Robot]) -> usize {
    fn flood_fill(out: &mut HashSet<(i64, i64)>, s: &HashSet<(i64, i64)>, pos: (i64, i64)) -> usize {
        if !s.contains(&pos) || !out.insert(pos) {
            return 0;
        }
        
        1 + flood_fill(out, s, (pos.0+1, pos.1))
          + flood_fill(out, s, (pos.0-1, pos.1))
          + flood_fill(out, s, (pos.0, pos.1+1))
          + flood_fill(out, s, (pos.0, pos.1-1))
    }
    
    const CHRISTMAS_TREE_MIN_SIZE: usize = 100;
    for t in 1.. {
        let mut locations = HashSet::new();

        for robot in robots {
            let t = t as i64;
            let x = (robot.pos.0 + t*robot.vel.0).rem_euclid(WIDTH);
            let y = (robot.pos.1 + t*robot.vel.1).rem_euclid(HEIGHT);
            locations.insert((x, y));
        }

        let mut filled = HashSet::new();
        for (i, j) in locations.iter() {
            let size = flood_fill(&mut filled, &locations, (*i, *j));
            if size >= CHRISTMAS_TREE_MIN_SIZE {
                return t;
            }
        }
    }
    
    unreachable!()
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    pos: (i64, i64),
    vel: (i64, i64),
}

fn parse_input(input: impl io::BufRead) -> Result<Vec<Robot>, Box<dyn Error>> {
    let rgx = Regex::new(r"^p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

    let mut robots = vec![];
    for line in input.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let Some(capture) = rgx.captures(line) else {
            return Err("Invalid input".into());
        };

        let (_, [px, py, vx, vy]) = capture.extract();
        let px = px.parse().unwrap();
        let py = py.parse().unwrap();
        let vx = vx.parse().unwrap();
        let vy = vy.parse().unwrap();

        robots.push(Robot{
            pos: (px, py),
            vel: (vx, vy)
        });
    }
    Ok(robots)
}
