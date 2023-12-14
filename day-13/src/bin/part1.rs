struct Data<'a> {
    data: Vec<&'a str>,
}

impl Data<'_> {
    fn new(data: &str) -> Data {
        let data: Vec<_> = data.split("\n\n").collect();
        Data { data }
    }

    fn get_pattern_by_row(&self, index: usize) -> Vec<&str> {
        let mut pattern_by_row: Vec<_> = Vec::new();
        for pattern in self.data[index].lines() {
            pattern_by_row.push(pattern);
        }
        pattern_by_row
    }

    fn get_pattern_by_col(&self, index: usize) -> Vec<String> {
        let num_cols = self.data[index].lines().next().unwrap().chars().count();

        let mut vec_of_iter: Vec<_> = Vec::new();
        for pattern in self.data[index].lines() {
            vec_of_iter.push(pattern.chars());
        }

        let mut pattern_by_col: Vec<String> = Vec::new();
        for _ in 0..num_cols {
            let mut items = vec![];
            for iter in &mut vec_of_iter {
                if let Some(item) = iter.next() {
                    items.push(item);
                }
            }
            let str_slice: String = items.iter().collect();
            pattern_by_col.push(str_slice);
        }
        pattern_by_col
    }

    fn find_equal(pattern: &Vec<&str>) -> Vec<usize> {
        let mut out: Vec<usize> = vec![];
        let mut previous = pattern[0];
        for (ctr, current) in pattern.into_iter().skip(1).enumerate() {
            if *current == previous {
                out.push(ctr + 1);
            }
            previous = *current;
        }
        out
    }

    fn check_reflection(pattern: &Vec<&str>, index: usize) -> bool {
        let num_rows = pattern.len();

        let fwd_iter = pattern.iter().skip(index + 1);
        let bwd_iter = pattern.iter().rev().skip(num_rows - index + 1);

        for (fwd, bwd) in fwd_iter.zip(bwd_iter) {
            if fwd != bwd {
                return false;
            }
        }
        true
    }
}

fn main() {
    let data = include_str!("../../../input/day-13.txt");
    let data = Data::new(data);

    let mut score = 0;

    for (ctr, _) in data.data.iter().enumerate() {
        let pattern_by_row = data.get_pattern_by_row(ctr);

        let equal_rows = Data::find_equal(&pattern_by_row);

        for equal_row in equal_rows.iter() {
            if Data::check_reflection(&pattern_by_row, *equal_row) {
                score += 100 * equal_row;
            }
        }
    }

    for (ctr, _) in data.data.iter().enumerate() {
        let pattern_by_col = data.get_pattern_by_col(ctr);
        let pattern_by_col: Vec<&str> = pattern_by_col.iter().map(|s| s.as_str()).collect();

        let equal_cols = Data::find_equal(&pattern_by_col);

        for equal_col in equal_cols.iter() {
            if Data::check_reflection(&pattern_by_col, *equal_col) {
                score += equal_col;
            }
        }
    }
    println!("Score: {}", score);
}
