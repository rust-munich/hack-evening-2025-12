struct Grid {
    cells: Vec<Vec<char>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        Self {
            cells: input.lines().map(|l| l.trim().chars().collect()).collect(),
        }
    }

    fn width(&self) -> usize {
        self.cells.first().map_or(0, |r| r.len())
    }

    fn height(&self) -> usize {
        self.cells.len()
    }

    fn get(&self, row: usize, col: usize) -> Option<char> {
        self.cells.get(row)?.get(col).copied()
    }

    fn paperolls(&self) -> PaperrollIterator {
        PaperrollIterator {
            grid: self,
            current_row: 0,
            current_col: 0,
        }
    }
}

struct PaperrollIterator<'a> {
    grid: &'a Grid,
    current_row: usize,
    current_col: usize,
}

impl Iterator for PaperrollIterator<'_> {
    type Item = (usize, usize, char);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current_row >= self.grid.height() {
                return None;
            }
            if self.current_col >= self.grid.width() {
                self.current_col = 0;
                self.current_row += 1;
                continue;
            }

            let (x, y) = (self.current_row, self.current_col);
            let c = self.grid.get(x, y)?;

            if c != '@' {
                // no paperroll here, continue
                self.current_col += 1;
                continue;
            }

            // we are at (x, y) with '@', so we check now all the neighbors,
            // if a neigboar is also '@', we count it,
            // if the count is >= 4 we continue the loop on the next position

            let mut paperrolls = 0;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if nx >= 0 && ny >= 0 {
                        if let Some(nc) = self.grid.get(nx as usize, ny as usize) {
                            if nc == '@' {
                                paperrolls += 1;
                            }
                        }
                    }

                    if paperrolls >= 4 {
                        break;
                    }
                }
            }

            self.current_col += 1;
            if paperrolls >= 4 {
                continue; // this was not a valid paperroll
            }
            // else we found a valid paperroll
            return Some((x, y, c));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid() -> Grid {
        Grid::new(
            r#"..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@."#,
        )
    }

    #[test]
    fn test_grid() {
        let g = grid();

        assert_eq!(g.height(), 10);
        assert_eq!(g.width(), 10);

        assert_eq!(g.get(0, 0), Some('.'));
        assert_eq!(g.get(0, 2), Some('@'));
        assert_eq!(g.get(1, 0), Some('@'));
        assert_eq!(g.get(1, 3), Some('.'));
        assert_eq!(g.get(1, 4), Some('@'));
        assert_eq!(g.get(9, 9), Some('.'));
        assert_eq!(g.get(10, 0), None);
    }

    #[test]
    fn test_iterator() {
        let g = grid();

        for (x, y, c) in g.paperolls() {
            println!("({x}, {y}) = {c}");
        }

        assert_eq!(g.paperolls().count(), 13);
    }

    #[test]
    fn test_input_final() {
        let g = Grid::new(include_str!("../../../input.txt"));
        let paperrolls = g.paperolls().count();
        println!("Paperolls = {paperrolls}");
    }
}
