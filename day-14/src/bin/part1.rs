struct Data<'a> {
    data: Vec<&'a str>,
}
impl Data<'_> {
    fn new(data: &str) -> Data {
        let data: Vec<_> = data.lines().collect();
        Data { data }
    }

    fn transpose(&self) -> Vec<String> {
        let num_cols = self.data[0].chars().count();
        let mut vec_of_iter: Vec<_> = Vec::new();

        for row in self.data.iter() {
            vec_of_iter.push(row.chars());
        }

        let mut columns: Vec<String> = Vec::new();
        for _ in 0..num_cols {
            let mut col_items = vec![];
            for iter in &mut vec_of_iter {
                if let Some(item) = iter.next() {
                    col_items.push(item);
                }
            }
            let str_slice: String = col_items.iter().collect();
            columns.push(str_slice);
        }
        columns
    }
}

fn main() {
    let data = include_str!("../../../input/day-14_test.txt");
    let data = Data::new(data);

    let num_cols = data.data[0].chars().count();
    let binding = data.transpose();
    let transposed: Vec<_> = binding.iter().map(|s| s.as_str()).collect();

    let mut score = 0;
    for str_slice in transposed.into_iter() {
        let mut items = vec![];

        let split: Vec<_> = str_slice.split('#').collect();

        for group in split {
            let mut group: Vec<_> = group.chars().collect();
            group.sort();
            let group: Vec<_> = group.into_iter().rev().collect();
            items.push(group);
        }

        let mut ctr = num_cols + 1;
        for column in items.iter() {
            ctr -= 1;
            for item in column.iter() {
                if item == &'O' {
                    score += ctr;
                }
                ctr -= 1;
            }
        }
    }
    println!("Score: {}", score);
}
