fn get_numbers(line: &str, skip: usize, take: usize) -> Vec<u32> {
    line.split([':', ' ', '|'])
        .filter_map(|item| item.parse().ok())
        .skip(skip)
        .take(take)
        .collect()
}

fn get_matches(winning_numbers: &Vec<u32>, selected_numbers: &Vec<u32>) -> u32 {
    let mut matches = 0;
    for winning_number in winning_numbers {
        if selected_numbers.contains(winning_number) {
            matches += 1;
        }
    }
    matches
}

fn get_score(matches: u32) -> u32 {
    if matches > 0 {
        2_u32.pow(matches - 1)
    } else {
        0
    }
}

fn main() {
    let data = include_str!("../../../input/day-04.txt");

    let mut total = 0;
    for line in data.lines() {
        let winning_numbers = get_numbers(line, 1, 10);
        let selected_numbers = get_numbers(line, 11, 25);
        let matches = get_matches(&winning_numbers, &selected_numbers);
        let score = get_score(matches);
        total += score;
    }
    println!("Total: {}", total);
}

#[cfg(test)]
mod tests {
    use super::{get_matches, get_numbers, get_score};

    #[test]
    fn test_total() {
        let data = include_str!("../../../input/day-04_test.txt");

        let mut total = 0;
        for line in data.lines() {
            let winning_numbers = get_numbers(line, 1, 5);
            let selected_numbers = get_numbers(line, 6, 8);
            let matches = get_matches(&winning_numbers, &selected_numbers);
            let score = get_score(matches);
            total += score;
        }
        assert_eq!(total, 13);
    }
}
