use std::io;
use std::error::Error;
use std::iter::repeat_n;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin().lock();
    let input = parse_input(stdin)?;

    println!("Part 1: {}", solve_part_1(&input));
    println!("Part 2: {}", solve_part_2(&input));
    Ok(())
}

fn solve_part_1(disk_map: &[u32]) -> usize {
    let disk = disk_map
        .chunks(2)
        .enumerate()
        .flat_map(|(id, chunk)| {
            let file_len = chunk[0] as usize;
            let space_len = *chunk.get(1).unwrap_or(&0) as usize;
            
            let file_blocks  = repeat_n(Some(id), file_len);
            let empty_blocks = repeat_n(None, space_len);
            file_blocks.chain(empty_blocks)
        })
        .collect::<Vec<_>>();

    let mut left = 0;
    let mut right = disk.len();
    let mut checksum = 0;
    while left < right {
        let block = disk[left].or_else(|| {
            right -= 1;
            disk[right]
        });
        if let Some(id) = block {
            checksum += left * id;
            left += 1;
        }
    }

    checksum
}

fn solve_part_2(disk_map: &[u32]) -> usize {
    let mut files = vec![];
    let mut spaces = vec![];
    let mut i = 0;
    for (id, chunk) in disk_map.chunks(2).enumerate() {
        let len = chunk[0] as usize;
        files.push((id, i..i+len));
        i += len;

        let len = *chunk.get(1).unwrap_or(&0) as usize;
        spaces.push(i..i+len);
        i += len;
    }

    let mut checksum = 0;
    for (id, file_range) in files.into_iter().rev() {
        let space = spaces.iter_mut()
            .take_while(|space| space.end <= file_range.start)
            .find(|space| space.len() >= file_range.len());

        if let Some(space) = space {
            let new_file_range = space.start .. space.start+file_range.len();
            space.start += file_range.len();
            checksum += id * new_file_range.sum::<usize>();
        } else {
            checksum += id * file_range.sum::<usize>();
        }
    }

    checksum
}

fn parse_input(input: impl io::Read) -> Result<Vec<u32>, Box<dyn Error>> {
    let input = io::read_to_string(input)?;

    input.trim_end().chars()
        .map(|ch| ch.to_digit(10))
        .collect::<Option<_>>()
        .ok_or("Invalid input format".into())
}
