use std::{
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("../../input.txt")?;
    let reader = BufReader::new(file);

    let mut lines: Vec<Vec<u8>> = vec![];

    for line in reader.split(b'\n') {
        match line {
            Ok(line) => {
                lines.push(line);
            }
            Err(e) => return Err(e.into()),
        }
    }

    let neighbors = count_rolls_with_few_neighbors(lines)?;

    println!("Found {} rolls for picking", neighbors);

    return Ok(());
}

const ROLL: u8 = b'@';

// Count paper rolls that have fewer than 4 neighboring paper rolls (8-directional adjacency)
fn count_rolls_with_few_neighbors(
    lines: Vec<Vec<u8>>,
) -> Result<usize, Box<dyn std::error::Error>> {
    let mut rolls_with_few_neighbors = 0;

    for (line_index, line) in lines.iter().enumerate() {
        for (char_index, char) in line.iter().enumerate() {
            if char == &ROLL {
                let neighbors = find_all_neighbors(&lines, line_index, char_index)
                    .into_iter()
                    .filter(|neighbor| *neighbor == ROLL)
                    .count();
                // println!(
                //     "found roll at {} {}, which has {}",
                //     line_index, char_index, neighbors
                // );

                if neighbors < 4 {
                    rolls_with_few_neighbors += 1;
                }
            }
        }
    }

    return Ok(rolls_with_few_neighbors);
}

fn find_all_neighbors(lines: &Vec<Vec<u8>>, line_index: usize, char_index: usize) -> Vec<u8> {
    let mut neighbors: Vec<u8> = vec![];

    for line_offset in -1isize..=1 {
        let y = match line_index.checked_add_signed(line_offset) {
            Some(y) => y,
            None => continue,
        };
        if y >= lines.len() {
            continue;
        }

        let line = &lines[y];

        for column_offset in -1isize..=1 {
            if line_offset == 0 && column_offset == 0 {
                // do not count the item itself
                continue;
            }

            let x = match char_index.checked_add_signed(column_offset) {
                Some(x) => x,
                None => continue,
            };

            if x >= line.len() {
                continue;
            }

            neighbors.push(line[x]);
        }
    }

    neighbors
}

#[cfg(test)]
mod tests {
    use crate::count_rolls_with_few_neighbors;

    #[test]
    fn test_1() -> Result<(), Box<dyn std::error::Error>> {
        let neighbors = count_rolls_with_few_neighbors(vec![
            "...".as_bytes().to_vec(),
            "...".as_bytes().to_vec(),
            "...".as_bytes().to_vec(),
        ])?;

        assert_eq!(neighbors, 0);

        Ok(())
    }

    #[test]
    fn test_2() -> Result<(), Box<dyn std::error::Error>> {
        let neighbors = count_rolls_with_few_neighbors(vec![
            "@..".as_bytes().to_vec(),
            "...".as_bytes().to_vec(),
            "..@".as_bytes().to_vec(),
        ])?;

        assert_eq!(neighbors, 2);

        Ok(())
    }

    #[test]
    fn test_3() -> Result<(), Box<dyn std::error::Error>> {
        let neighbors = count_rolls_with_few_neighbors(vec![
            "@..".as_bytes().to_vec(),
            "@@@".as_bytes().to_vec(),
            "@@.".as_bytes().to_vec(),
        ])?;

        assert_eq!(neighbors, 3);

        Ok(())
    }
}
