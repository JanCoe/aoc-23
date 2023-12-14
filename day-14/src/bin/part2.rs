fn rotate(data: &Vec<String>) -> Vec<String> {
    let num_cols = data[0].chars().count();
    let mut vec_of_iter: Vec<_> = Vec::new();

    for row in data.iter() {
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

fn roll(data: &Vec<&str>, sort: bool) -> Vec<String> {
    let mut merged: String;
    let mut merged_vec: Vec<String> = Vec::new();

    for str_slice in data.into_iter() {
        let mut items = vec![];

        let split: Vec<_> = str_slice.split('#').collect();

        for group in split {
            let mut group: Vec<_> = group.chars().collect();
            if sort {
                group.sort();
            }
            let group: Vec<_> = group.into_iter().rev().collect();
            items.push(group);
        }

        merged = items
            .clone()
            .into_iter()
            .map(|v| v.into_iter().collect::<String>())
            .collect::<Vec<String>>()
            .join(&'#'.to_string());
        merged_vec.push(merged);
    }
    merged_vec
}

fn score(data: &Vec<String>) -> usize {
    let mut score = 0;
    let num_cols = data[0].chars().count();

    for col in data.into_iter() {
        let mut ctr = num_cols;

        for item in col.chars() {
            if item == 'O' {
                score += ctr;
            }
            ctr -= 1;
        }
    }
    score
}

fn main() {
    let data = include_str!("../../../input/day-14_test.txt");
    let data = data.lines().map(String::from).collect();

    let num_cycles = 1;
    for ctr in 0..num_cycles {
        //NORTH
        //transpose
        let binding = rotate(&data);
        let data: Vec<_> = binding.iter().map(|s| s.as_str()).collect();

        //roll
        let data = roll(&data, true);

        println!("North: {:?}", data);

        //WEST
        //transpose
        let binding = rotate(&data);
        let data: Vec<_> = binding.iter().map(|s| s.as_str()).collect();

        //roll
        let data = roll(&data, false);

        println!("West: {:?}", data);

        //SOUTH
        //transpose
        let binding = rotate(&data);
        let data: Vec<_> = binding.iter().map(|s| s.as_str()).collect();

        //roll
        let data = roll(&data, true);

        println!("South: {:?}", data);

        //EAST
        //transpose
        let binding = rotate(&data);
        let data: Vec<_> = binding.iter().map(|s| s.as_str()).collect();

        //roll
        let data = roll(&data, false);

        println!("East: {:?}", data);

        //score
        if ctr == (num_cycles - 1) {
            let total_score = score(&data);
            println!("{:?}", data);
            println!("Score: {}", total_score);
        }
    }
}
