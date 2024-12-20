use std::io;
use std::error::Error;
use std::str::Chars;

mod trie;
use trie::*;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin().lock();
    let input = parse_input(stdin)?;

    println!("Part 1: {}", solve_part_1(&input));
    println!("Part 2: {}", solve_part_2(&input));
    Ok(())
}

struct PuzzleInput {
    trie: Trie,
    designs: Vec<String>,
}

fn solve_part_1(input: &PuzzleInput) -> usize {
    input.designs.iter()
        .filter(|d| design_is_possible(d, &input.trie))
        .count()
}

fn design_is_possible(design: &str, trie: &Trie) -> bool {
    fn is_possible(mut chars: Chars, mut current: &Trie, root: &Trie) -> bool {
        while let Some(c) = chars.next() {
            let Some(next) = current.get(c) else {
                return false;
            };
            current = next;
    
            if current.is_end() && is_possible(chars.clone(), root, root) {
                return true;
            }
        }
        
        current.is_end()
    }

    is_possible(design.chars(), trie, trie)
}


fn solve_part_2(input: &PuzzleInput) -> u64 {
    input.designs.iter()
        .map(|d| count_arrangements(d, &input.trie))
        .sum()
}

fn count_arrangements(design: &str, trie: &Trie) -> u64 {
    let chars: Vec<_> = design.chars().collect();
    let mut counts = vec![0; chars.len() + 1];
    counts[chars.len()] = 1;
    
    for (i, _) in chars.iter().enumerate().rev() {
        let mut node = trie;
        for (j, c) in chars[i..].iter().enumerate() {
            let Some(next) = node.get(*c) else {
                break;
            };
            if next.is_end() {
                counts[i] += counts[i + j + 1];
            }
            node = next;
        }
    }

    counts[0]
}


fn parse_input(input: impl io::Read) -> Result<PuzzleInput, Box<dyn Error>> {
    let full_file = io::read_to_string(input)?;
    let mut lines = full_file.lines();

    let Some(first_line) = lines.next() else {
        return Err("Invalid input")?;
    };
    let trie: Trie = first_line.split(',')
        .map(str::trim)
        .collect();

    let designs = lines
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| String::from(line))
        .collect();

    Ok(PuzzleInput{trie, designs})
}
