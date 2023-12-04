use std::collections::HashMap;

fn get_numbers(line: &str, skip: usize, take: usize) -> Vec<usize> {
    line.split([':', ' ', '|'])
        .filter_map(|item| item.parse().ok())
        .skip(skip)
        .take(take)
        .collect()
}

fn get_matches(winning_numbers: &Vec<usize>, selected_numbers: &Vec<usize>) -> usize {
    let mut matches = 0;
    for winning_number in winning_numbers {
        if selected_numbers.contains(winning_number) {
            matches += 1;
        }
    }
    matches
}

fn main() {
    let data = include_str!("../../../input/day-04.txt");

    let numbers = data.lines().count();
    let mut cards: HashMap<_, _> = (1..=numbers).into_iter().map(|n| (n, 1_u32)).collect();

    for (ctr, line) in data.lines().enumerate() {
        let winning_numbers = get_numbers(line, 1, 10);
        let selected_numbers = get_numbers(line, 11, 25);
        let matches = get_matches(&winning_numbers, &selected_numbers);

        let number = cards.get(&(ctr + 1)).unwrap().clone();

        for extra in ctr + 1 + 1..=ctr + 1 + matches {
            if let Some(entry) = cards.get_mut(&extra) {
                *entry += number;
            }
        }
    }
    let total = cards.values().sum::<u32>();
    println!("Total: {}", total);
}
