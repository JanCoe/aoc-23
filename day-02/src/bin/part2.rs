fn main() {
    let content = include_str!("../../../input/day-02.txt");
    let total: u32 = content.lines().map(|line| power_of_game(line)).sum();
    println!("Total: {:?}", total);
}

fn power_of_game(line: &str) -> u32 {
    let mut blue = 0;
    let mut red = 0;
    let mut green = 0;

    let line = line.split(": ");
    let boxes = line.last().unwrap();
    let turns: Vec<_> = boxes.split("; ").collect();

    for turn in &turns {
        let selections: Vec<_> = turn.split(", ").collect();

        for selection in &selections {
            let go: Vec<_> = selection.split(" ").collect();

            let number: u32 = go[0].parse().unwrap();
            match go[1] {
                "blue" => if number > blue { blue = number},
                "red" => if number > red { red = number},
                "green" => if number > green { green = number},
                _ => panic!("wrong colour"),
            }
        }
    }
    blue * red * green
}

#[cfg(test)]
mod tests {
    use super::{power_of_game};

    #[test]
    fn test_power_of_game() {
        let content = include_str!("../../../input/day-02_test.txt");
        let total: u32 = content.lines().map(|line| power_of_game(line)).sum();
        assert_eq!(total, 2286);
    }
}