use std::io;
use std::error::Error;
use std::collections::{HashMap, HashSet};

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin().lock();
    let input = parse_input(stdin)?;

    let (answer_1, answer_2) = solve(&input);
    println!("Part 1: {}", answer_1);
    println!("Part 2: {}", answer_2);
    Ok(())
}

type RulesMap = HashMap<i32, HashSet<i32>>;
type PuzzleInput = (RulesMap, Vec<Vec<i32>>);

fn solve(input: &PuzzleInput) -> (i32, i32) {
    let (rules, updates) = input;

    let mut answers = (0, 0);
    for update in updates {
        if is_ordered_correctly(update, rules) {
            answers.0 += update[ update.len()/2 ];
        } else {
            let update = sort_pages(update.clone(), rules);
            answers.1 += update[ update.len()/2 ];
        }
    }
    answers
}

fn is_ordered_correctly(update: &[i32], rules: &RulesMap) -> bool {
    let mut seen = HashSet::new();
    for page in update {
        seen.insert(*page);

        let Some(succesors) = rules.get(page) else {continue;};
        if !succesors.is_disjoint(&seen) {
            return false;
        }
    }

    true
}

fn sort_pages(mut pages: Vec<i32>, rules: &RulesMap) -> Vec<i32> {
    pages.sort_by(|a, b| {
        let empty_set = HashSet::new();
        if rules.get(a).unwrap_or(&empty_set).contains(b) {
            std::cmp::Ordering::Less
        } else if rules.get(b).unwrap_or(&empty_set).contains(a) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
    pages
}

fn parse_input(input: impl io::BufRead) -> Result<PuzzleInput, Box<dyn Error>> {
    let mut rules = RulesMap::new();
    let mut updates = vec![];

    let mut is_first_section = true;
    for line in input.lines() {
        let line = line?;
        if line.trim().is_empty() {
            is_first_section = false;
            continue;
        }

        if is_first_section {
            let (x,y) = line.split_once('|')
                .ok_or("Invalid format: missing '|' character")?;
    
            let x: i32 = x.parse()?;
            let y: i32 = y.parse()?;
            rules.entry(x).or_default().insert(y);
        } else {
            let pages: Vec<i32> = line.split(',')
                .map(str::parse::<i32>)
                .collect::<Result<_, _>>()?;
            updates.push(pages);
        }
    }

    Ok((rules, updates))
}
