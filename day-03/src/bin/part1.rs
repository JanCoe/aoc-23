#[derive(Debug)]
struct Point {
    row: isize,
    col: isize,
}

#[derive(Debug)]
struct PartNumber {
    pos: Point,
    number: u32,
}

impl PartNumber {
    fn new(pos: Point, number: u32) -> PartNumber {
        PartNumber { pos, number }
    }

    fn coordinates_to_test(&self, grid: &Grid) -> (Vec<isize>, Vec<isize>) {
        let length = (self.number as f64).log10().floor() as isize + 1;

        let row_start = (self.pos.row - 1).max(0);
        let row_end = (self.pos.row + 1).min(grid.rows as isize - 1);
        let row_slice: Vec<isize> = (row_start..=row_end).collect();

        let col_start = (self.pos.col - 1).max(0);
        let col_end = (self.pos.col + length).min(grid.cols as isize - 1);
        let col_slice: Vec<isize> = (col_start..=col_end).collect();

        (row_slice, col_slice)
    }

    fn get_valid_number(&self, grid: &Grid) -> u32 {
        let (row_slice, col_slice) = self.coordinates_to_test(grid);
        for row in &row_slice {
            let this_row = grid.content.lines().skip(*row as usize).next().unwrap();
            for col in &col_slice {
                let symbol = this_row.chars().skip(*col as usize).next().unwrap();
                if !symbol.is_digit(10) & !(symbol == '.') {
                    return self.number;
                }
            }
        }
        return 0;
    }
}

struct Grid<'a> {
    content: &'a str,
    symbols: Vec<char>,
    rows: usize,
    cols: usize,
    numbers: Vec<PartNumber>,
}

impl Grid<'_> {
    fn new(content: &str) -> Grid {
        let symbols = Self::unique_symbols(content);
        let numbers = Self::get_part_numbers_vec(content, &symbols);
        Grid {
            content,
            symbols,
            rows: content.lines().count(),
            cols: content.lines().next().unwrap().chars().count(),
            numbers,
        }
    }

    fn unique_symbols(content: &str) -> Vec<char> {
        let mut unique_chars = Vec::new();
        for ch in content.chars() {
            if !unique_chars.contains(&ch) && !ch.is_digit(10) {
                unique_chars.push(ch);
            }
        }
        unique_chars
    }

    fn get_part_numbers_vec(content: &str, symbols: &Vec<char>) -> Vec<PartNumber> {
        let mut part_numbers = Vec::new();

        for (row, line) in content.lines().enumerate() {
            let numbers: Vec<u32> = line
                .split(symbols.as_slice())
                .filter_map(|item| item.parse().ok())
                .collect();

            for number in &numbers {
                part_numbers.push(PartNumber::new(
                    Point {
                        row: row as isize,
                        col: line.find(&number.to_string()).unwrap() as isize,
                    },
                    *number,
                ));
            }
        }
        part_numbers
    }
}

fn main() {
    let content = include_str!("../../../input/day-03.txt");
    let grid = Grid::new(content);

    let total: u32 = grid.numbers.iter().map(|p| p.get_valid_number(&grid)).sum();

    for part in grid.numbers {
        println!("Number: {}", part.number);
    }

    println!("Total: {}", total);
}

#[cfg(test)]
mod tests {
    use super::Grid;

    #[test]
    fn test_valid_parts() {
        let content = include_str!("../../../input/day-03_test.txt");
        let grid = Grid::new(content);

        let total: u32 = grid.numbers.iter().map(|p| p.get_valid_number(&grid)).sum();

        assert_eq!(total, 4361 - 633);
    }
}
