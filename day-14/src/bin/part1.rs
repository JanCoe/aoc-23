struct Data<'a> {
    data: Vec<&'a str>,
}
impl Data<'_> {
    fn new(data: &str) -> Data {
        let data: Vec<_> = data.lines().collect();
        Data { data }
    }

    fn rotate(&self) -> Vec<String> {
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
    let data = include_str!("../../../input/day-14.txt");
    let data = Data::new(data);

    let num_cols = data.data[0].chars().count();

    //transpose
    let binding = data.rotate();
    let transposed: Vec<_> = binding.iter().map(|s| s.as_str()).collect();

    //roll
    let mut merged = String::new();
    let mut merged_vec: Vec<String> = Vec::new();
    for str_slice in transposed.into_iter() {
        let mut items = vec![];

        let split: Vec<_> = str_slice.split('#').collect();

        for group in split {
            let mut group: Vec<_> = group.chars().collect();
            group.sort();
            let group: Vec<_> = group.into_iter().rev().collect();
            items.push(group);
        }

        merged = items
            .clone()
            .into_iter()
            .map(|v| v.into_iter().collect::<String>())
            .collect::<Vec<String>>()
            .join(&'#'.to_string());
        println!("{}", merged);
        merged_vec.push(merged);
    }

    //score
    let mut score = 0;
    for col in merged_vec.into_iter() {
        let mut ctr = num_cols;

        for item in col.chars() {
            if item == 'O' {
                score += ctr;
            }
            ctr -= 1;
        }
    }
    println!("Score: {}", score);
}
