use std::{
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let file = File::open("/Volumes/CaseSensititve/SourcesCS/rust-munich-2/rust_munich_input.txt")?;
    let reader = BufReader::new(file);

    let mut lines: Vec<Vec<u8>> = vec![];
    let mut columns = 0;

    for line in reader.lines() {
        match line {
            Ok(line) => {
                columns = line.len();
                lines.push(line.into_bytes());
            }
            Err(e) => return Err(e.into()),
        }
    }

    let neighbors = count_rolls_with_few_neighbors(lines, columns)?;

    println!("Found {} neighbors", neighbors);

    return Ok(());
}

const ROLL: u8 = b'@';

// Count paper rolls that have fewer than 4 neighboring paper rolls (8-directional adjacency)
fn count_rolls_with_few_neighbors(
    lines: Vec<Vec<u8>>,
    columns: usize,
) -> Result<usize, Box<dyn std::error::Error>> {
    let mut rolls_with_few_neighbors = 0;

    for (line_index, line) in lines.iter().enumerate() {
        for (char_index, char) in line.iter().enumerate() {
            if char == &ROLL {
                let neighbors = find_all_neighbors(&lines, columns, line_index, char_index)
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

fn find_all_neighbors(
    lines: &Vec<Vec<u8>>,
    columns: usize,
    line_index: usize,
    char_index: usize,
) -> Vec<u8> {
    let mut neighbors: Vec<u8> = vec![];

    if line_index > 0 {
        let previous_line = &lines[line_index - 1];
        neighbors.append(&mut get_item(
            &previous_line,
            neighbor_indices(char_index, true, columns),
        ));
    }

    let current_line = &lines[line_index];
    neighbors.append(&mut get_item(
        &current_line,
        neighbor_indices(char_index, false, columns),
    ));

    if line_index + 1 < lines.len() {
        let next_line = &lines[line_index + 1];
        neighbors.append(&mut get_item(
            &next_line,
            neighbor_indices(char_index, true, columns),
        ));
    }

    neighbors
}

fn neighbor_indices(char_index: usize, include_self: bool, columns: usize) -> Vec<usize> {
    let mut indices: Vec<usize> = vec![];

    if char_index > 0 {
        indices.push(char_index - 1);
    }

    if include_self {
        indices.push(char_index);
    }

    if char_index + 1 < columns {
        indices.push(char_index + 1);
    }

    indices
}

fn get_item(line: &Vec<u8>, indices: Vec<usize>) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];

    for index in indices {
        if index < line.len() {
            result.push(line[index]);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::count_rolls_with_few_neighbors;

    #[test]
    fn test_1() -> Result<(), Box<dyn std::error::Error>> {
        let neighbors = count_rolls_with_few_neighbors(
            vec![
                "...".as_bytes().to_vec(),
                "...".as_bytes().to_vec(),
                "...".as_bytes().to_vec(),
            ],
            3,
        )?;

        assert_eq!(neighbors, 0);

        Ok(())
    }

    #[test]
    fn test_2() -> Result<(), Box<dyn std::error::Error>> {
        let neighbors = count_rolls_with_few_neighbors(
            vec![
                "@..".as_bytes().to_vec(),
                "...".as_bytes().to_vec(),
                "..@".as_bytes().to_vec(),
            ],
            3,
        )?;

        assert_eq!(neighbors, 2);

        Ok(())
    }

    #[test]
    fn test_3() -> Result<(), Box<dyn std::error::Error>> {
        let neighbors = count_rolls_with_few_neighbors(
            vec![
                "@..".as_bytes().to_vec(),
                "@@@".as_bytes().to_vec(),
                "@@.".as_bytes().to_vec(),
            ],
            3,
        )?;

        assert_eq!(neighbors, 3);

        Ok(())
    }
}
